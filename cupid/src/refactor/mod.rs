mod array;
pub use array::*;

mod assignment;
pub use assignment::*;

mod block;
pub use block::*;

mod builtin_type;
pub use builtin_type::*;

mod declaration;
pub use declaration::*;

mod function;
pub use function::*;

mod function_call;
pub use function_call::*;

// mod function_signature;
// pub use function_signature::*;

mod generics;
pub use generics::*;

mod implementation;
pub use implementation::*;

mod implementation_node;
pub use implementation_node::*;

mod log;
pub use log::*;

mod operation;
pub use operation::*;

mod property;
pub use property::*;

mod scope;
pub use scope::*;

mod semantics;
pub use semantics::*;

mod symbol;
pub use symbol::*;

mod traits;
pub use traits::*;

mod type_hint;
pub use type_hint::*;

mod use_block;
pub use use_block::*;

mod use_trait_block;
pub use use_trait_block::*;

mod value;
pub use value::*;

use std::ops::Deref;
use crate::{RLexicalScope, Error};

pub trait CloneAST {
	fn clone_ast(&self) -> Box<dyn AST>;
}

impl <T> CloneAST for T where T: AST + Clone + 'static {
	fn clone_ast(&self) -> Box<dyn AST> {
		Box::new(self.clone())
	}
}

pub trait AST: std::fmt::Debug + CloneAST + serde_traitobject::Serialize + serde_traitobject::Deserialize {
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error>;
}

impl Clone for Box<dyn AST> {
	fn clone(&self) -> Self {
    	self.clone_ast()
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BoxAST {
	#[serde(with = "serde_traitobject")]
	inner: Box<dyn AST>,
}

impl BoxAST {
	pub fn new(inner: impl AST + 'static) -> Self {
		Self {
			inner: Box::new(inner),
		}
	}
}

impl Deref for BoxAST {
	type Target = dyn AST;
	fn deref(&self) -> &Self::Target {
    	&*self.inner
	}
}

impl From<Box<dyn AST>> for BoxAST {
	fn from(b: Box<dyn AST>) -> Self {
    	BoxAST { inner: b }
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum OptionAST {
	#[serde(with = "serde_traitobject")]
	Some(Box<dyn AST>),
	None
}