use crate::*;

// 1. recursively resolves types and mutates AST nodes with type info
// 2. checks for correctness in resolved types
pub trait TypeCheck {
	fn resolve(&mut self, scope: &mut LexicalScope) -> Result<(), Error>;
	fn check(&self) -> Result<(), Error> { Ok(()) }
}

// collects multiple tokens to use for better error handling
pub trait CollectRelevantTokens {
	// creates a vector of all tokens that have are related to the current context
	fn collect_all(nodes: Vec<&impl AST>) -> Vec<&Token>;

	fn collect(nodes: Vec<&impl AST>) -> Vec<&Token> {
		// override to filter for relevant tokens
		Self::collect_all(nodes)
	}
	
	// reduces multiple separate tokens into one supertoken
	fn reduce(tokens: Vec<&Token>) -> Token;
}

trait Recover {
	fn try_skipping() -> Result<(ParseNode, bool), Error>;
}
