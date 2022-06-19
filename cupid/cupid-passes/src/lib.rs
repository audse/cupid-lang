#![allow(unused_variables)]
#![feature(derive_default_enum, is_some_with, trait_alias)]

pub mod env;
pub(crate) use env::{Address, Source, ScopeId, Env, Mut};

pub mod tests;

pub mod util;
pub(crate) use util::attributes::*;
pub(crate) use util::static_nodes::*;

pub type PassErr = (Source, ErrCode);
pub type PassResult<T> = Result<T, PassErr>;
pub type ErrCode = usize;

/// Each AST pass takes a node from the previous pass and transforms it
/// # Stages
///  1. `pre_analysis`
///  2. `package_resolution`
///  3. `type_scope_analysis`
///  4. `type_name_resolution`
///  5. `scope_analysis`
///  6. `name_resolution`
///  7. `type_inference`
///  8. `type_checking`
///  9. `flow_checking`
/// 10. `linting`

pub mod flow_checking;
pub mod linting;
pub mod name_resolution;
pub mod package_resolution;
pub mod pre_analysis;
pub mod scope_analysis;
pub mod type_checking;
pub mod type_inference;
pub mod type_name_resolution;
pub mod type_scope_analysis;
