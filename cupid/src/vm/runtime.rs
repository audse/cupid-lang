use std::ops::Deref;

use crate::{
    chunk::Instruction,
    error::CupidError,
    objects::{Array, Class, Closure, RoleImpl},
    table::Table,
    value::Value,
};

use super::Vm;

pub trait Runtime {
    fn into_runtime_err(&self, msg: impl std::fmt::Display) -> CupidError;
    fn runtime_err(&self, msg: impl std::fmt::Display) -> Result<(), CupidError>;
    fn run(&mut self) -> Result<(), CupidError>;
}

impl Runtime for Vm {
    fn into_runtime_err(&self, msg: impl std::fmt::Display) -> CupidError {
        self.runtime_err(msg).unwrap_err()
    }

    fn runtime_err(&self, msg: impl std::fmt::Display) -> Result<(), CupidError> {
        eprintln!("{}", msg);
        eprintln!("[line {}] in script", self.frames.frame().line());
        Err(CupidError::RuntimeError)
    }

    fn run(&mut self) -> Result<(), CupidError> {
        let mut state = self.frames.state();

        loop {
            let instruction = state.instruction();
            state.set_instruction(1);

            match instruction {
                Instruction::Add => {
                    let (b, a) = (self.stack.pop(), self.stack.pop());
                    let result = a.add(b, self).map_err(|e| self.into_runtime_err(e))?;
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
                        .push(a.divide(b).map_err(|e| self.into_runtime_err(e))?);
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
                            return self.runtime_err(&format!(
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
                        return self.runtime_err("Only instances have properties.");
                    }
                }
                Instruction::GetSuper(constant) => {
                    let method_name = state.chunk.read_string(constant);
                    if let Value::Class(superclass) = self.stack.pop() {
                        self.bind_method(superclass, method_name)?;
                    } else {
                        return self.runtime_err("Super found no class");
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
                        .push(a.greater(b).map_err(|e| self.into_runtime_err(e))?);
                }
                Instruction::Inherit => {
                    let pair = (self.stack.peek(0), self.stack.peek(1));
                    if let (Value::Class(mut subclass), Value::Class(superclass)) = pair {
                        subclass.methods = Table::default();
                        subclass.methods.add_all(&superclass.methods);
                        self.stack.pop();
                    } else {
                        return self.runtime_err("Superclass must be a class.");
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
                        .push(a.lesser(b).map_err(|e| self.into_runtime_err(e))?);
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
                        .push(a.multiply(b).map_err(|e| self.into_runtime_err(e))?);
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
                    _ => return self.runtime_err("Operand must be a number."),
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
                        _ => return self.runtime_err("Traits may only be implemented on classes."),
                    }
                }
                Instruction::SetGlobal(constant) => {
                    let global_name = state.chunk.read_string(constant);
                    let value = self.stack.peek(0);
                    if self.globals.set(global_name, value) {
                        self.globals.delete(global_name);
                        return self.runtime_err(&format!(
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
                        return self.runtime_err("Only instances have fields.");
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
                        .push(a.subtract(b).map_err(|e| self.into_runtime_err(e))?);
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
}
