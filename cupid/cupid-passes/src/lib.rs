#![allow(unused_variables)]
#![feature(derive_default_enum, is_some_with, explicit_generic_args_with_impl_trait)]

//! Each AST pass takes a node from the previous pass and transforms it
//! # Stages
//!  1. `pre_analysis`
//!  2. `package_resolution`
//!  3. `type_scope_analysis`
//!  4. `type_name_resolution`
//!  5. `scope_analysis`
//!  6. `name_resolution`
//!  7. `type_inference`
//!  8. `type_checking`
//!  9. `flow_checking`
//! 10. `linting`

pub(self) use cupid_util::ErrCode;

pub mod env;
pub(self) use env::{Address, ScopeId, Env, Mut, database::{Query, ReadQuery, WriteQuery}};

mod tests;

pub mod util;
pub(self) use util::attributes::*;
pub(self) use util::static_nodes::*;

pub type PassErr = (Address, ErrCode);
pub type PassResult<T> = Result<T, PassErr>;

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
