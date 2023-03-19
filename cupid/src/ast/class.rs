use std::cell::{Ref, RefMut};

use super::{Define, Expr, ExprHeader, Header, Method, SourceId};
use crate::{pointer::Pointer, scope::Scope, token::Token, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Class<'src> {
        pub name: Token<'src>,
        pub super_class: Option<Token<'src>>,
        pub fields: Vec<Define<'src>>,
        pub methods: Vec<Method<'src>>,
        pub class_scope: Pointer<Scope<'src>>,
    }
}

impl<'src> Class<'src> {
    pub fn class_scope(&self) -> Ref<Scope<'src>> {
        self.class_scope.borrow()
    }
    pub fn class_scope_mut(&mut self) -> RefMut<Scope<'src>> {
        self.class_scope.borrow_mut()
    }
}

pub struct ClassSource<'src> {
    pub open_brace: Token<'src>,
    pub close_brace: Token<'src>,
    pub name: Token<'src>,
    pub super_class: Option<Token<'src>>,
    pub super_class_name: Option<Token<'src>>,
    pub fields: SourceId,
    pub methods: SourceId,
}

impl<'src> From<Class<'src>> for Expr<'src> {
    fn from(value: Class<'src>) -> Self {
        Expr::Class(value.into())
    }
}
