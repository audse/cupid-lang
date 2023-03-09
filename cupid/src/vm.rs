use crate::{
    // chunk::Instruction,
    compiler::compile,
    error::CupidError,
    expose,
    gc::{Gc, GcRef},
    objects::{BoundMethod, Class, Closure, Instance, NativeFunction, Str, Upvalue},
    table::Table,
    value::Value,
};
use std::{fmt::Display, ops::Deref, time::SystemTime};

pub mod frame;
pub use self::frame::*;

pub mod runtime;
pub use self::runtime::*;

pub mod stack;
pub use self::stack::*;

#[derive(Debug, Clone)]
pub struct Vm {
    pub gc: Gc,
    pub frames: Frames,
    pub stack: Stack<Value>,
    pub globals: Table,
    pub open_upvalues: Vec<GcRef<Upvalue>>,
    pub init_string: GcRef<Str>,
    pub start_time: SystemTime,
}

impl Default for Vm {
    fn default() -> Self {
        let mut gc = Gc::default();
        let init_string = gc.intern("init".to_owned());

        Self {
            gc,
            frames: Frames::default(),
            stack: Stack::default(),
            globals: Table::default(),
            open_upvalues: Vec::with_capacity(stack::SIZE),
            init_string,
            start_time: SystemTime::now(),
        }
    }
}

impl Vm {
    pub fn initialize(&mut self) {
        self.define_native("clock", NativeFunction(expose::cupid_clock));
        self.define_native("panic", NativeFunction(expose::cupid_panic));
        self.define_native("push", NativeFunction(expose::cupid_push));
        self.define_native("pop", NativeFunction(expose::cupid_pop));
        self.define_native("len", NativeFunction(expose::cupid_len));
        self.define_native("get", NativeFunction(expose::cupid_get));
        self.stack.top = self.stack.stack.as_mut_ptr();
    }

    pub fn interpret(&mut self, code: &str) -> Result<(), CupidError> {
        let function = compile(code, &mut self.gc)?;
        self.stack.push(Value::Function(function));
        let closure = self.alloc(Closure::new(function));
        self.frames.increment(CallFrame::new(closure, 0));
        self.run()
    }

    fn define_native(&mut self, name: &str, native: NativeFunction) {
        let name = self.gc.intern(name.to_owned());
        self.globals.set(name, Value::NativeFunction(native));
    }

    pub fn call_value(&mut self, arg_count: usize) -> Result<(), CupidError> {
        let callee = self.stack.peek(arg_count);
        match callee {
            Value::BoundMethod(bound) => {
                self.stack.set_at(arg_count, bound.receiver);
                self.call(bound.method, arg_count)
            }
            Value::Class(class) => {
                let instance = self.alloc(Instance::new(class));
                self.stack.set_at(arg_count, Value::Instance(instance));
                if let Some(initializer) = class.methods.get(self.init_string) {
                    if let Value::Closure(initializer) = initializer {
                        return self.call(initializer, arg_count);
                    }
                    return self.runtime_err("Initializer is not closure");
                } else if arg_count != 0 {
                    let msg = format!("Expected 0 arguments but got {}.", arg_count);
                    return self.runtime_err(&msg);
                }
                Ok(())
            }
            Value::Closure(closure) => self.call(closure, arg_count),
            Value::NativeFunction(native) => {
                let left = self.stack.len() - arg_count;
                let result = native.0(self, &self.stack.stack[left..]);
                self.stack.truncate(left - 1);
                self.stack.push(result);
                Ok(())
            }
            _ => self.runtime_err("Can only call functions and classes."),
        }
    }

    pub fn call(&mut self, closure: GcRef<Closure>, arg_count: usize) -> Result<(), CupidError> {
        let function = closure.function;
        if arg_count != function.arity {
            self.runtime_err(&format!(
                "Expected {} arguments but got {}.",
                function.arity, arg_count
            ))
        } else if self.frames.count == Frames::MAX {
            self.runtime_err("Stack overflow.")
        } else {
            let frame = CallFrame::new(closure, self.stack.len() - arg_count - 1);
            self.frames.increment(frame);
            Ok(())
        }
    }

    pub fn invoke(&mut self, name: GcRef<Str>, arg_count: usize) -> Result<(), CupidError> {
        let receiver = self.stack.peek(arg_count);
        if let Value::Instance(instance) = receiver {
            if let Some(field) = instance.fields.get(name) {
                self.stack.set_at(arg_count, field);
                self.call_value(arg_count)
            } else {
                let class = instance.class;
                self.invoke_from_class(class, name, arg_count)
            }
        } else {
            self.runtime_err("Only instances have methods.")
        }
    }

    pub fn invoke_from_class(
        &mut self,
        class: GcRef<Class>,
        name: GcRef<Str>,
        arg_count: usize,
    ) -> Result<(), CupidError> {
        if let Some(method) = class.methods.get(name) {
            if let Value::Closure(closure) = method {
                self.call(closure, arg_count)
            } else {
                panic!("Got method that is not closure!")
            }
        } else {
            let msg = format!("Undefined property '{}'.", name.deref());
            self.runtime_err(&msg)
        }
    }

    pub fn bind_method(&mut self, class: GcRef<Class>, name: GcRef<Str>) -> Result<(), CupidError> {
        if let Some(method) = class.methods.get(name) {
            let receiver = self.stack.peek(0);
            let method = match method {
                Value::Closure(closure) => closure,
                _ => panic!("Inconsistent state. Method is not closure"),
            };
            let bound = self.alloc(BoundMethod::new(receiver, method));
            self.stack.pop();
            self.stack.push(Value::BoundMethod(bound));
            Ok(())
        } else {
            let msg = format!("Undefined property '{}'.", name.deref());
            self.runtime_err(&msg)
        }
    }

    pub fn capture_upvalue(&mut self, location: usize) -> GcRef<Upvalue> {
        for &upvalue in &self.open_upvalues {
            if upvalue.location == location {
                return upvalue;
            }
        }
        let upvalue = Upvalue::new(location);
        let upvalue = self.alloc(upvalue);
        self.open_upvalues.push(upvalue);
        upvalue
    }

    pub fn close_upvalues(&mut self, last: usize) {
        let mut i = 0;
        while i != self.open_upvalues.len() {
            let mut upvalue = self.open_upvalues[i];
            if upvalue.location >= last {
                // PERF: Remove is expensive
                self.open_upvalues.remove(i);
                let location = upvalue.location;
                upvalue.closed = Some(self.stack.stack[location]);
            } else {
                i += 1;
            }
        }
    }

    pub fn define_method(&mut self, name: GcRef<Str>) {
        let method = self.stack.peek(0);
        match self.stack.peek(1) {
            Value::Class(mut class) => {
                class.methods.set(name, method);
                self.stack.pop();
            }
            Value::RoleImpl(mut role) => {
                role.class.methods.set(name, method);
                self.stack.pop();
            }
            _ => panic!("Invalid state: trying to define a method of non class"),
        }
    }

    pub fn alloc<T: Display + std::fmt::Debug + 'static>(&mut self, object: T) -> GcRef<T> {
        self.mark_and_sweep();
        self.gc.alloc(object)
    }

    pub fn intern(&mut self, name: String) -> GcRef<Str> {
        self.mark_and_sweep();
        self.gc.intern(name)
    }

    pub fn mark_and_sweep(&mut self) {
        if self.gc.should_gc() {
            self.mark_roots();
            self.gc.collect_garbage();
        }
    }

    pub fn mark_roots(&mut self) {
        for &value in &self.stack.stack[0..self.stack.len()] {
            self.gc.mark_value(value);
        }

        for frame in &self.frames.frames[..self.frames.count] {
            self.gc.mark_object(frame.closure)
        }

        for &upvalue in &self.open_upvalues {
            self.gc.mark_object(upvalue);
        }

        self.gc.mark_table(&self.globals);
        self.gc.mark_object(self.init_string);
    }
}
