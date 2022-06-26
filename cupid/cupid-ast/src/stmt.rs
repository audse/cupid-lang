use derive_more::{From, TryInto, IsVariant, Unwrap};
pub mod decl;
pub mod trait_def;
pub mod type_def;

#[derive(Debug, Clone, From, TryInto, IsVariant, Unwrap)]
pub enum Stmt {
    Decl(decl::Decl),
    TraitDef(trait_def::TraitDef),
    TypeDef(type_def::TypeDef),
}