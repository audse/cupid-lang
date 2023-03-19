use crate::{arena::EntryId, ast::GetTy, pointer::Pointer, ty::Type};

use super::Scope;

#[derive(Debug, Clone, Copy, Default)]
pub struct Symbol<'src> {
    pub ty: Type<'src>,
    pub value: SymbolValue<'src>,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum SymbolValue<'src> {
    Class(ClassId<'src>),
    Instance(ClassId<'src>),
    Expr(EntryId),
    #[default]
    Unset,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
pub struct ClassId<'src>(pub &'src str);

#[derive(Debug, Clone)]
pub struct ClassTable<'src> {
    pub scope: Pointer<Scope<'src>>,
}

impl<'src> GetTy<'src> for Symbol<'src> {
    fn ty(&self) -> Type<'src> {
        self.ty
    }
    fn set_ty(&mut self, ty: Type<'src>) {
        self.ty = ty;
    }
}
