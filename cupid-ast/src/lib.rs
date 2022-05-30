pub use std::collections::HashMap;
pub use std::borrow::Cow;
use std::hash::{
	Hash,
	Hasher,
};
pub use lazy_static::lazy_static;
pub use derive_more::*;
pub use tabled::*;

pub use cupid_lex::{
	Error,
	Token,
};
pub use cupid_parse::ParseNode;
pub use cupid_util::*;

mod analysis;
pub use analysis::*;

mod create;
pub use create::*;

mod diagnostics;
pub use diagnostics::*;

pub type Source = usize;
pub type ErrCode = usize;
pub type ASTErr = (Source, ErrCode);

mod nodes;
pub use nodes::*;

mod scope;
pub use scope::*;

mod utils;
pub use utils::*;