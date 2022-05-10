use crate::{parse, SymbolNode, AST, ParseNode, LexicalScope, ValueNode, Error, Meta, SymbolValue, Scope, BoxAST};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AssignmentNode {
	pub symbol: SymbolNode,
	pub value: BoxAST,
	pub meta: Meta<()>
}

impl From<&mut ParseNode> for AssignmentNode {
	fn from(node: &mut ParseNode) -> Self {
    	Self {
			symbol: SymbolNode::from(&mut node.children[0]),
			value: parse(&mut node.children[1]),
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
	}
}

impl AST for AssignmentNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut value = self.value.resolve(scope)?;
		
		// add meta info to value node
		value.set_meta_identifier(&self.symbol.0);
		
		scope.set_symbol(&self.symbol, SymbolValue::Assignment { value })
	}
}