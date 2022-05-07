use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignatureNode {
	pub type_hint: TypeHintNode,
	pub symbol: SymbolNode,
	pub params: ParametersNode,
}

impl From<&mut ParseNode> for FunctionSignatureNode {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			type_hint: TypeHintNode::from(&mut node.children[0]),
			symbol: SymbolNode::from(&mut node.children[1]),
			params: ParametersNode::from(&mut node.children[2]),
		}
	}
}

impl FunctionSignatureNode {
	pub fn parse_as_declaration(node: &mut ParseNode) -> DeclarationNode {
		let value = FunctionNode {
			params: ParametersNode::from(&mut node.children[2]),
			body: BlockNode { expressions: vec![] }
		};
		DeclarationNode { 
			type_hint: TypeHintNode::from(&mut node.children[0]), 
			symbol: SymbolNode::from(&mut node.children[1]), 
			mutable: false, 
			value: BoxAST::new(value),
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
	}
}
// 
// impl AST for FunctionSignatureNode {
// 	fn resolve(&self, _scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
// 		Ok(ValueNode::from_value(Value::Function(self.to_owned())))
// 	}
// }