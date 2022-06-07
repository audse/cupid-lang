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

mod nodes;
pub use nodes::*;

mod scope;
pub use scope::*;

mod utils;
pub use utils::*;