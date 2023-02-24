use derive_more::{From, IsVariant, TryInto, Unwrap};

use crate::attr::{Attr, GetAttr};
pub mod allocate;
pub mod assign;
pub mod decl;
pub mod implement;
pub mod trait_def;
pub mod type_def;

#[derive(Debug, Clone, From, TryInto, IsVariant, Unwrap, serde::Serialize, serde::Deserialize)]
pub enum Stmt {
    Allocate(allocate::Allocate),
    Assign(assign::Assign),
    Decl(decl::Decl),
    Impl(implement::Impl),
    TraitDef(trait_def::TraitDef),
    TypeDef(type_def::TypeDef),
}

impl GetAttr for Stmt {
    fn attr(&self) -> Attr {
        match self {
            Self::Allocate(x) => x.attr(),
            Self::Assign(x) => x.attr(),
            Self::Decl(x) => x.attr(),
            Self::Impl(x) => x.attr(),
            Self::TraitDef(x) => x.attr(),
            Self::TypeDef(x) => x.attr(),
        }
    }
}
