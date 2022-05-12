use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeclarationNode {
	pub type_hint: TypeHintNode,
	pub symbol: SymbolNode,
	pub mutable: bool,
	pub value: BoxAST,
	pub meta: Meta<()>,
}

impl From<&mut ParseNode> for DeclarationNode {
	fn from(node: &mut ParseNode) -> Self {
		let value = if node.children.len() > 2 {
			parse(&mut node.children[2])
		} else {
			BoxAST::new(ValueNode { 
				value: Value::None, 
				type_hint: None,
				meta: Meta::new(vec![], None, vec![]) 
			})
		};
		Self {
			type_hint: TypeHintNode::from(&mut node.children[0]),
			symbol: SymbolNode::from(&mut node.children[1]),
			mutable: node.tokens.iter().any(|t| &*t.source == "mut"),
			value,
			meta: Meta::with_tokens(node.tokens.to_owned()),
		}
	}
}

impl AST for DeclarationNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut value = self.value.resolve(scope)?;
		
		// add meta info to value node
		value.set_meta_identifier(&self.symbol.0);
		
		let type_hint = Some(self.type_hint.to_owned());
		
		// set symbol type as value's type
		let mut symbol = self.symbol.to_owned();
		symbol.0.type_hint = type_hint.to_owned();
		
		scope.set_symbol(&symbol, SymbolValue::Declaration { 
			type_hint, 
			mutable: self.mutable, 
			value
		})
	}
}