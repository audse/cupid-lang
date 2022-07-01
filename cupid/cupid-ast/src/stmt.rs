use derive_more::{From, TryInto, IsVariant, Unwrap};

use crate::attr::{GetAttr, Attr};
pub mod decl;
pub mod trait_def;
pub mod type_def;

#[derive(Debug, Clone, From, TryInto, IsVariant, Unwrap, serde::Serialize, serde::Deserialize)]
pub enum Stmt {
    Decl(decl::Decl),
    TraitDef(trait_def::TraitDef),
    TypeDef(type_def::TypeDef),
}

impl GetAttr for Stmt {
    fn attr(&self) -> Attr {
        match self {
            Self::Decl(x) => x.attr(),
            Self::TraitDef(x) => x.attr(),
            Self::TypeDef(x) => x.attr()
        }
    }
    fn attr_mut(&mut self) -> &mut Attr {
        match self {
            Self::Decl(x) => x.attr_mut(),
            Self::TraitDef(x) => x.attr_mut(),
            Self::TypeDef(x) => x.attr_mut()
        }
    }
}