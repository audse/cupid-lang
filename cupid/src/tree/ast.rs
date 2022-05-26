use std::ops::Deref;
use crate::*;

pub trait FromParse {
	fn from_parse(node: &mut ParseNode) -> Self;
}

pub trait CloneAST {
	fn clone_ast(&self) -> Box<dyn AST>;
}

impl <T> CloneAST for T where T: AST + Clone + 'static {
	fn clone_ast(&self) -> Box<dyn AST> {
		Box::new(self.clone())
	}
}

pub trait AST: std::fmt::Debug + CloneAST + serde_traitobject::Serialize + serde_traitobject::Deserialize + Display {
	
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error>;
	
	fn as_symbol(&self) -> Option<&SymbolNode> { None }
	fn as_function_call(&self) -> Option<&FunctionCallNode> { None }
	fn as_builtin_function_call(&self) -> Option<&BuiltinFunctionCallNode> { None }
}

pub trait ResolveTo<T>: AST {
	fn resolve_to(&self, scope: &mut LexicalScope) -> Result<T, Error>;
}

impl Clone for Box<dyn AST> {
	fn clone(&self) -> Self {
    	self.clone_ast()
	}
}

#[derive(Clone, Serialize, Deserialize)]
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
	pub fn symbol_or_resolve(&self, scope: &mut LexicalScope) -> Result<Self, Error> {
		// there are a few situations where we don't want to resolve symbols,
		// but we do want to resolve anything else
		// e.g. `person.name` (name is undefined) vs `people.0`
		if self.as_symbol().is_some() {
			Ok(self.to_owned())
		} else {
			Ok(Self::new(self.resolve(scope)?))
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

impl std::fmt::Debug for BoxAST {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Box({:#?})", self.inner)
	}
}

impl Display for BoxAST {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
    	write!(f, "{}", self.inner)
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