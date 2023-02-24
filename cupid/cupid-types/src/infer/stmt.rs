use cupid_ast::{
    stmt::{
        allocate::{Allocate, Allocation},
        assign::Assign,
        decl::Decl,
        implement::Impl,
        trait_def::TraitDef,
        type_def::TypeDef,
        Stmt,
    },
    types::typ::Type,
};

use crate::infer::Infer;

impl Infer<Type> for Stmt {
    fn infer(&self) -> Type {
        Type::none()
    }
}

impl Infer<Type> for Decl {
    fn infer(&self) -> Type {
        Type::none()
    }
}

impl Infer<Type> for Impl {
    fn infer(&self) -> Type {
        Type::none()
    }
}

impl Infer<Type> for TraitDef {
    fn infer(&self) -> Type {
        Type::none()
    }
}

impl Infer<Type> for TypeDef {
    fn infer(&self) -> Type {
        Type::none()
    }
}

impl Infer<Type> for Allocation {
    fn infer(&self) -> Type {
        match self {
            Self::Expr(e) => e.borrow().infer(),
            Self::Trait(t) => t.borrow().infer(),
            Self::Type(t) => t.borrow().infer(),
        }
    }
}

impl Infer<Type> for Allocate {
    fn infer(&self) -> Type {
        Type::none()
    }
}

impl Infer<Type> for Assign {
    fn infer(&self) -> Type {
        Type::none()
    }
}
