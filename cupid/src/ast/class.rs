use std::cell::{Ref, RefMut};

use super::{Define, Expr, ExprHeader, Header, Method};
use crate::{pointer::Pointer, scope::Scope, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Class<'src> {
        pub name: &'src str,
        pub super_class: Option<&'src str>,
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

impl<'src> From<Class<'src>> for Expr<'src> {
    fn from(value: Class<'src>) -> Self {
        Expr::Class(value.into())
    }
}
