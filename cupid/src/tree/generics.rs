use crate::{SymbolNode, ParseNode, AST, LexicalScope, ValueNode, Error};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenericsNode {
	pub generics: Vec<SymbolNode>
}

impl GenericsNode {
	pub fn from_parent(node: &mut ParseNode) -> Option<Self> {
		let generics_node = node.children.iter_mut().find(|n| n.name.as_str() == "generics");
		generics_node.map(GenericsNode::from)
	}
}

impl From<&mut ParseNode> for GenericsNode {
	fn from(node: &mut ParseNode) -> Self {
    	GenericsNode {
			generics: node.children.iter_mut().map(SymbolNode::from).collect()
		}
	}
}

impl AST for GenericsNode {
	fn resolve(&self, _scope: &mut LexicalScope) -> Result<ValueNode, Error> {
    	todo!()
	}
}