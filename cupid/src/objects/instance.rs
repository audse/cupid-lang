use std::{fmt, ops::Deref};

use crate::{
    gc::{GcObject, GcRef},
    objects::{Class, ObjectType},
    table::Table,
};

#[repr(C)]
#[derive(Debug)]
pub struct Instance {
    pub header: GcObject,
    pub class: GcRef<Class>,
    pub fields: Table,
}

impl Instance {
    pub fn new(class: GcRef<Class>) -> Self {
        Instance {
            header: GcObject::new(ObjectType::Instance),
            class,
            fields: Table::default(),
        }
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class.name.deref())
    }
}
