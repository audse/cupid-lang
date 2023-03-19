use std::fmt;

use crate::{arena::EntryId, scope::symbol::ClassId};

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
pub enum Type<'src> {
    Array(EntryId),
    Bool,
    Class(ClassId<'src>),
    Int,
    Float,
    Nil,
    String,
    Instance(ClassId<'src>),
    Function {
        returns: EntryId,
    },
    #[default]
    Unknown,
    Unit,
    Type,
}

impl<'src> Type<'src> {
    pub fn class(name: &'src str) -> Self {
        Type::Class(ClassId(name))
    }
}

impl fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Class(class) => write!(f, "Class('{}')", class.0),
            Self::Instance(instance) => write!(f, "Instance('{}')", instance.0),
            _ => write!(f, "{:?}", self),
        }
    }
}
