use std::{fmt, ops::Deref};

use crate::{
    gc::{GcObject, GcRef},
    objects::{Class, ObjectType, Str},
    table::Table,
};

#[repr(C)]
#[derive(Debug)]
pub struct RoleImpl {
    pub header: GcObject,
    pub name: GcRef<Str>,
    pub class: GcRef<Class>,
    pub methods: Table,
}

impl RoleImpl {
    pub fn new(name: GcRef<Str>, class: GcRef<Class>) -> Self {
        RoleImpl {
            header: GcObject::new(ObjectType::Role),
            name,
            class,
            methods: Table::default(),
        }
    }
}

impl fmt::Display for RoleImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class.deref())
    }
}
