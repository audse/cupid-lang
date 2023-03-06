use std::{fmt, ops::Deref};

use crate::{
    chunk::{Chunk, Value},
    gc::{GcObject, GcRef},
    objects::{FunctionUpvalue, ObjectType, Str},
    vm::Vm,
};

#[derive(Clone, Copy)]
pub struct NativeFunction(pub fn(&Vm, &[Value]) -> Value);

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn>")
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

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.name.deref().s == "script" {
            write!(f, "<script>")
        } else {
            write!(f, "<fn {}>", self.name.deref())
        }
    }
}
