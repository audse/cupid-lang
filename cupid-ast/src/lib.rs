pub use std::collections::HashMap;
pub use std::borrow::Cow;
use std::hash::{
	Hash,
	Hasher,
};
pub use lazy_static::lazy_static;

pub use cupid_lex::{
	Error,
	Token,
};
pub use cupid_parse::ParseNode;

mod analysis;
pub use analysis::*;

mod diagnostics;
pub use diagnostics::*;

mod to_ast;
pub use to_ast::*;

pub type ErrCode = usize;