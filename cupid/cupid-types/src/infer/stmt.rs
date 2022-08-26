use cupid_ast::{
    stmt::{assign::Assign, decl::Decl, trait_def::TraitDef, type_def::TypeDef, Stmt},
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

impl Infer<Type> for Assign {
    fn infer(&self) -> Type {
        Type::none()
    }
}
