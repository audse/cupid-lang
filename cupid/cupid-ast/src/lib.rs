pub use std::collections::HashMap;
pub use std::borrow::Cow;
use std::hash::{
	Hash,
	Hasher,
};
use derive_more::*;
use tabled::*;
use colored::Colorize;

use cupid_lex::{
	token::Token,
	node::ParseNode,
};
use cupid_util::*;
use cupid_trace::trace_this;

pub type Source = usize;
pub type ASTErr = (Exp, ErrCode);

pub mod diagnostics;
pub use diagnostics::*;

pub mod attributes;
pub use attributes::*;

pub mod block;
pub use block::*;

pub mod builders;
pub mod declaration;
pub use declaration::*;

pub mod expression;
pub use expression::*;

pub mod function_call;
pub use function_call::*;

pub mod function;
pub use function::*;

pub mod ident;
pub use ident::*;

pub mod property;
pub use property::*;

pub mod type_system;
pub use type_system::*;

pub mod value;
pub use value::*;

pub mod scope;
pub use scope::*;

pub mod utils;
pub use utils::*;