use std::{fmt, ops::Deref};

use crate::{
    chunk::Value,
    gc::{GcObject, GcRef},
    objects::{Closure, ObjectType},
};

#[repr(C)]
pub struct BoundMethod {
    pub header: GcObject,
    pub receiver: Value,
    pub method: GcRef<Closure>,
}

impl BoundMethod {
    pub fn new(receiver: Value, method: GcRef<Closure>) -> Self {
        BoundMethod {
            header: GcObject::new(ObjectType::BoundMethod),
            receiver,
            method,
        }
    }
}

impl fmt::Display for BoundMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.method.function.deref())
    }
}
