use std::fmt;

use crate::scope::symbol::ClassId;

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
pub enum Type<'src> {
    Array,
    Bool,
    Class(ClassId<'src>),
    Int,
    Float,
    Nil,
    String,
    Instance,
    Function,
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
        write!(f, "<{:?}>", self)
    }
}
