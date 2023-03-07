use std::fmt;

use crate::{gc::GcObject, objects::ObjectType, value::Value};

#[repr(C)]
#[derive(Debug)]
pub struct Upvalue {
    pub header: GcObject,
    pub location: usize,
    pub closed: Option<Value>,
}

impl Upvalue {
    pub fn new(location: usize) -> Self {
        Upvalue {
            header: GcObject::new(ObjectType::Upvalue),
            location,
            closed: None,
        }
    }
}

impl fmt::Display for Upvalue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "upvalue")
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FunctionUpvalue {
    pub index: u8,
    pub is_local: bool,
}
