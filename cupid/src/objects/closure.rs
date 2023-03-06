use std::{fmt, ops::Deref};

use crate::{
    gc::{GcObject, GcRef},
    objects::{Function, ObjectType, Upvalue},
};

#[repr(C)]
pub struct Closure {
    pub header: GcObject,
    pub function: GcRef<Function>,
    pub upvalues: Vec<GcRef<Upvalue>>,
}

impl Closure {
    pub fn new(function: GcRef<Function>) -> Self {
        Closure {
            header: GcObject::new(ObjectType::Closure),
            function,
            upvalues: Vec::new(),
        }
    }
}

impl fmt::Display for Closure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.function.deref())
    }
}
