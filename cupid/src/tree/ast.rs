use std::ops::Deref;
use crate::*;

pub trait CloneAST {
	fn clone_ast(&self) -> Box<dyn AST>;
}

impl <T> CloneAST for T where T: AST + Clone + 'static {
	fn clone_ast(&self) -> Box<dyn AST> {
		Box::new(self.clone())
	}
}

pub trait AST: std::fmt::Debug + CloneAST + serde_traitobject::Serialize + serde_traitobject::Deserialize {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error>;
}

pub trait ResolveTo<T>: AST {
	fn resolve_to(&self, scope: &mut LexicalScope) -> Result<T, Error>;
}

impl Clone for Box<dyn AST> {
	fn clone(&self) -> Self {
    	self.clone_ast()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxAST {
	#[serde(with = "serde_traitobject")]
	pub inner: Box<dyn AST>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionAST {
	#[serde(with = "serde_traitobject")]
	Some(Box<dyn AST>),
	None,
}

pub trait FromParent<T> {
	fn from_parent(parent: T) -> Self;
}