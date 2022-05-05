use crate::{parse, SymbolNode, AST, ParseNode, RLexicalScope, ValueNode, Error, Meta, RSymbolValue, RScope};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AssignmentNode {
	pub symbol: SymbolNode,
	#[serde(with = "serde_traitobject")]
	pub value: Box<dyn AST>,
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
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		let mut value = self.value.resolve(scope)?;
		
		// add meta info to value node
		value.set_meta_identifier(&self.symbol.0);
		
		scope.set_symbol(&self.symbol, &RSymbolValue::Assignment { value })
	}
}