use std::fmt;

use crate::{gc::GcObject, objects::ObjectType, value::Value};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Array {
    pub header: GcObject,
    pub items: Vec<Value>,
}

impl Array {
    pub fn new(items: Vec<Value>) -> Self {
        Array {
            header: GcObject::new(ObjectType::Array),
            items,
        }
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.items
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
