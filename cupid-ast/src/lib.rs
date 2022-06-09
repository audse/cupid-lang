#![feature(explicit_generic_args_with_impl_trait)]
#![feature(associated_type_bounds)]
pub use std::collections::HashMap;
pub use std::borrow::Cow;
use std::hash::{
	Hash,
	Hasher,
};
pub use lazy_static::lazy_static;
pub use derive_more::*;
pub use tabled::*;
pub use colored::Colorize;

pub use cupid_lex::{
	Error,
	Token,
	ParseNode,
};
pub use cupid_util::*;

pub type Source = usize;
pub type ASTErr = (Exp, ErrCode);

mod diagnostics;
pub use diagnostics::*;
mod attributes;
pub use attributes::*;

mod block;
pub use block::*;

mod builders;
pub use builders::*;

mod declaration;
pub use declaration::*;

mod expression;
pub use expression::*;

mod function_call;
pub use function_call::*;

mod function;
pub use function::*;

mod ident;
pub use ident::*;

mod property;
pub use property::*;

mod type_system;
pub use type_system::*;

mod value;
pub use value::*;

mod scope;
pub use scope::*;

mod utils;
pub use utils::*;