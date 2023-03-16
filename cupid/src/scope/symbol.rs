use crate::{pointer::Pointer, ty::Type};

use super::Scope;

#[derive(Debug, Clone, Copy, Default)]
pub struct Symbol<'src> {
    pub ty: Type<'src>,
    pub value: SymbolValue<'src>,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum SymbolValue<'src> {
    Class(ClassId<'src>),
    #[default]
    Unset,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub struct ClassId<'src>(pub &'src str);

#[derive(Debug, Clone)]
pub struct Class<'src> {
    pub scope: Pointer<Scope<'src>>,
}
