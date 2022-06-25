pub mod decl;
pub mod trait_def;
pub mod type_def;

pub enum Stmt {
    Decl(decl::Decl),
    TraitDef(trait_def::TraitDef),
    TypeDef(type_def::TypeDef),
}