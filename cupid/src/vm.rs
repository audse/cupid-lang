use crate::{
    chunk::Instruction,
    compiler::compile,
    error::CupidError,
    expose,
    gc::{Gc, GcRef},
    objects::{
        Array, BoundMethod, Class, Closure, Instance, NativeFunction, RoleImpl, Str, Upvalue,
    },
    table::Table,
    value::Value,
};
use std::{fmt::Display, ops::Deref, time::SystemTime};

pub mod frame;
pub use self::frame::*;

pub mod stack;
pub use self::stack::*;

pub struct Vm {
    pub gc: Gc,
    pub frames: Frames,
    pub stack: Stack,
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
            open_upvalues: Vec::with_capacity(Stack::SIZE),
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

    pub fn runtime_error(&self, msg: &str) -> Result<(), CupidError> {
        let current_frame = &self.frames.frames[self.frames.count - 1];
        eprintln!("{}", msg);
        eprintln!("[line {}] in script", current_frame.line());
        Err(CupidError::RuntimeError)
    }

    pub fn into_runtime_error(&self, msg: &str) -> CupidError {
        self.runtime_error(msg).unwrap_err()
    }

    fn run(&mut self) -> Result<(), CupidError> {
        let mut state = self.frames.state();

        loop {
            let instruction = state.instruction();
            state.set_instruction(1);

            match instruction {
                Instruction::Add => {
                    let (b, a) = (self.stack.pop(), self.stack.pop());
                    let result = a.add(b, self).map_err(|e| self.into_runtime_error(e))?;
                    self.stack.push(result);
                }
                Instruction::Array(item_count) => {
                    let mut items = vec![];
                    for _ in 0..item_count {
                        let item = self.stack.pop();
                        items.push(item);
                    }
                    items.reverse();
                    let array = self.alloc(Array::new(items));
                    self.stack.push(Value::Array(array));
                }
                Instruction::Class(constant) => {
                    let class_name = state.chunk.read_string(constant);
                    let class = self.alloc(Class::new(class_name));
                    self.stack.push(Value::Class(class));
                }
                Instruction::CloseUpvalue => {
                    let top = self.stack.len() - 1;
                    self.close_upvalues(top);
                    self.stack.pop();
                }
                Instruction::Closure(constant) => {
                    let function = state.chunk.read_constant(constant);
                    if let Value::Function(function) = function {
                        let upvalue_count = function.upvalues.len();
                        let mut closure = Closure::new(function);

                        for i in 0..upvalue_count {
                            let upvalue = function.upvalues[i];
                            let obj_upvalue = if upvalue.is_local {
                                let location = state.frame.slot + upvalue.index as usize;
                                self.capture_upvalue(location)
                            } else {
                                state.frame.closure.upvalues[upvalue.index as usize]
                            };
                            closure.upvalues.push(obj_upvalue)
                        }

                        let closure = self.alloc(closure);
                        self.stack.push(Value::Closure(closure));
                    } else {
                        panic!("Closure instruction without function value");
                    }
                }
                Instruction::Call(arg_count) => {
                    self.call_value(arg_count as usize)?;
                    state = self.frames.state();
                }
                Instruction::Constant(constant) => {
                    let value = state.chunk.read_constant(constant);
                    self.stack.push(value);
                }
                Instruction::DefineGlobal(constant) => {
                    let global_name = state.chunk.read_string(constant);
                    let value = self.stack.pop();
                    self.globals.set(global_name, value);
                }
                Instruction::Divide => {
                    let (b, a) = (self.stack.pop(), self.stack.pop());
                    self.stack
                        .push(a.divide(b).map_err(|e| self.into_runtime_error(e))?);
                }
                Instruction::Equal => {
                    let (b, a) = (self.stack.pop(), self.stack.pop());
                    self.stack.push(Value::Bool(a == b));
                }
                Instruction::False => self.stack.push(Value::Bool(false)),
                Instruction::GetGlobal(constant) => {
                    let global_name = state.chunk.read_string(constant);
                    match self.globals.get(global_name) {
                        Some(value) => self.stack.push(value),
                        None => {
                            return self.runtime_error(&format!(
                                "Undefined variable '{}'.",
                                global_name.deref()
                            ));
                        }
                    }
                }
                Instruction::GetLocal(slot) => {
                    let i = slot as usize + state.frame.slot;
                    self.stack.push(self.stack.stack[i]);
                }
                Instruction::GetProperty(constant) => {
                    if let Value::Instance(instance) = self.stack.peek(0) {
                        let class = instance.class;
                        let property_name = state.chunk.read_string(constant);
                        let value = instance.fields.get(property_name);
                        match value {
                            Some(value) => {
                                self.stack.pop();
                                self.stack.push(value);
                            }
                            None => self.bind_method(class, property_name)?,
                        }
                    } else {
                        return self.runtime_error("Only instances have properties.");
                    }
                }
                Instruction::GetSuper(constant) => {
                    let method_name = state.chunk.read_string(constant);
                    if let Value::Class(superclass) = self.stack.pop() {
                        self.bind_method(superclass, method_name)?;
                    } else {
                        return self.runtime_error("Super found no class");
                    }
                }
                Instruction::GetUpvalue(slot) => {
                    let value = {
                        let upvalue = state.frame.closure.upvalues[slot as usize];
                        if let Some(value) = upvalue.closed {
                            value
                        } else {
                            self.stack.stack[upvalue.location]
                        }
                    };
                    self.stack.push(value);
                }
                Instruction::Greater => {
                    let (b, a) = (self.stack.pop(), self.stack.pop());
                    self.stack
                        .push(a.greater(b).map_err(|e| self.into_runtime_error(e))?);
                }
                Instruction::Inherit => {
                    let pair = (self.stack.peek(0), self.stack.peek(1));
                    if let (Value::Class(mut subclass), Value::Class(superclass)) = pair {
                        subclass.methods = Table::default();
                        subclass.methods.add_all(&superclass.methods);
                        self.stack.pop();
                    } else {
                        return self.runtime_error("Superclass must be a class.");
                    }
                }
                Instruction::Invoke((constant, arg_count)) => {
                    let name = state.chunk.read_string(constant);
                    self.invoke(name, arg_count as usize)?;
                    state = self.frames.state();
                }
                Instruction::Jump(offset) => {
                    state.set_instruction(offset as isize);
                }
                Instruction::JumpIfFalse(offset) => {
                    if self.stack.peek(0).is_falsey() {
                        state.set_instruction(offset as isize);
                    }
                }
                Instruction::Less => {
                    let (b, a) = (self.stack.pop(), self.stack.pop());
                    self.stack
                        .push(a.lesser(b).map_err(|e| self.into_runtime_error(e))?);
                }
                Instruction::Loop(offset) => {
                    state.set_instruction(-1 - (offset as isize));
                }
                Instruction::Method(constant) => {
                    let method_name = state.chunk.read_string(constant);
                    self.define_method(method_name);
                }
                Instruction::Multiply => {
                    let (b, a) = (self.stack.pop(), self.stack.pop());
                    self.stack
                        .push(a.multiply(b).map_err(|e| self.into_runtime_error(e))?);
                }
                Instruction::Negate => match self.stack.peek(0) {
                    Value::Float(value) => {
                        self.stack.pop();
                        self.stack.push(Value::Float(-value));
                    }
                    Value::Int(value) => {
                        self.stack.pop();
                        self.stack.push(Value::Int(-value));
                    }
                    _ => return self.runtime_error("Operand must be a number."),
                },
                Instruction::Nil => self.stack.push(Value::Nil),
                Instruction::Not => {
                    let value = self.stack.pop();
                    self.stack.push(Value::Bool(value.is_falsey()));
                }
                Instruction::Pop => {
                    self.stack.pop();
                }
                Instruction::Log => {
                    println!("{}", self.stack.pop());
                }
                Instruction::Return => {
                    self.frames.count -= 1;
                    let return_value = self.stack.pop();
                    self.close_upvalues(state.frame.slot);

                    if self.frames.count == 0 {
                        return Ok(());
                    } else {
                        self.stack.truncate(state.frame.slot);
                        self.stack.push(return_value);
                        state = self.frames.state();
                    }
                }
                Instruction::RoleImpl(role_name) => {
                    let role = state.chunk.read_string(role_name);
                    match self.stack.peek(0) {
                        Value::Class(class) => {
                            let role_impl = self.alloc(RoleImpl::new(role, class));
                            self.stack.push(Value::RoleImpl(role_impl));
                        }
                        _ => {
                            return self.runtime_error("Traits may only be implemented on classes.")
                        }
                    }
                }
                Instruction::SetGlobal(constant) => {
                    let global_name = state.chunk.read_string(constant);
                    let value = self.stack.peek(0);
                    if self.globals.set(global_name, value) {
                        self.globals.delete(global_name);
                        return self.runtime_error(&format!(
                            "Undefined variable '{}'.",
                            global_name.deref()
                        ));
                    }
                }
                Instruction::SetLocal(slot) => {
                    let i = slot as usize + state.frame.slot;
                    let value = self.stack.peek(0);
                    self.stack.stack[i] = value;
                }
                Instruction::SetProperty(constant) => {
                    if let Value::Instance(mut instance) = self.stack.peek(1) {
                        let property_name = state.chunk.read_string(constant);
                        let value = self.stack.pop();
                        instance.fields.set(property_name, value);
                        self.stack.pop();
                        self.stack.push(value);
                    } else {
                        return self.runtime_error("Only instances have fields.");
                    }
                }
                Instruction::SetUpvalue(slot) => {
                    let mut upvalue = state.frame.closure.upvalues[slot as usize];
                    let value = self.stack.peek(0);
                    if upvalue.closed.is_none() {
                        self.stack.stack[upvalue.location] = value;
                    } else {
                        upvalue.closed = Some(value);
                    }
                }
                Instruction::Subtract => {
                    let (b, a) = (self.stack.pop(), self.stack.pop());
                    self.stack
                        .push(a.subtract(b).map_err(|e| self.into_runtime_error(e))?);
                }
                Instruction::SuperInvoke((constant, arg_count)) => {
                    let method_name = state.chunk.read_string(constant);
                    if let Value::Class(class) = self.stack.pop() {
                        self.invoke_from_class(class, method_name, arg_count as usize)?;
                        state = self.frames.state();
                    } else {
                        panic!("super invoke with no class");
                    }
                }
                Instruction::True => self.stack.push(Value::Bool(true)),
            };
        }
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
                    return self.runtime_error("Initializer is not closure");
                } else if arg_count != 0 {
                    let msg = format!("Expected 0 arguments but got {}.", arg_count);
                    return self.runtime_error(&msg);
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
            _ => self.runtime_error("Can only call functions and classes."),
        }
    }

    pub fn call(&mut self, closure: GcRef<Closure>, arg_count: usize) -> Result<(), CupidError> {
        let function = closure.function;
        if arg_count != function.arity {
            self.runtime_error(&format!(
                "Expected {} arguments but got {}.",
                function.arity, arg_count
            ))
        } else if self.frames.count == Frames::MAX {
            self.runtime_error("Stack overflow.")
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
            self.runtime_error("Only instances have methods.")
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
            self.runtime_error(&msg)
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
            self.runtime_error(&msg)
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
