use std::{fmt, ops::Deref};

use crate::{
    gc::{GcObject, GcRef},
    objects::{ObjectType, Str},
    table::Table,
};

#[repr(C)]
pub struct Class {
    pub header: GcObject,
    pub name: GcRef<Str>,
    pub methods: Table,
}

impl Class {
    pub fn new(name: GcRef<Str>) -> Self {
        Class {
            header: GcObject::new(ObjectType::Class),
            name,
            methods: Table::default(),
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.deref())
    }
}
