use cupid_ast::{stmt::{Stmt, decl::Decl, type_def::TypeDef, trait_def::TraitDef}, types::typ::Type};

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