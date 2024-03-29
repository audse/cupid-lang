use std::{fmt, ops::Deref};

use crate::{
    chunk::Chunk,
    gc::{GcObject, GcRef},
    objects::{FunctionUpvalue, ObjectType, Str},
    value::Value,
    vm::Vm,
};

#[derive(Clone, Copy)]
pub struct NativeFunction(pub fn(&Vm, &[Value]) -> Value);

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fun>")
    }
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

#[repr(C)]
pub struct Function {
    pub header: GcObject,
    pub arity: usize,
    pub chunk: Chunk,
    pub name: GcRef<Str>,
    pub upvalues: Vec<FunctionUpvalue>,
}

impl Function {
    pub fn new(name: GcRef<Str>) -> Self {
        Self {
            header: GcObject::new(ObjectType::Function),
            arity: 0,
            chunk: Chunk::default(),
            name,
            upvalues: Vec::new(),
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name.deref().s)
            .field("arity", &self.arity)
            // .field("header", &self.header)
            .field("chunk", &self.chunk)
            .field("upvalues", &self.upvalues)
            .finish()
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.name.deref().s == "script" {
            write!(f, "<script>")
        } else {
            write!(f, "<fun {}>", self.name.deref())
        }
    }
}
