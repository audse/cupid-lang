use crate::{parse, SymbolNode, AST, ParseNode, RLexicalScope, RSymbolValue, RScope, ValueNode, Error, Meta, TypeHintNode, Value, TypeKind, BoxAST};

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
			BoxAST::from(parse(&mut node.children[2]))
		} else {
			BoxAST { 
				inner: Box::new(ValueNode { 
					value: Value::None, 
					type_kind: TypeKind::infer(&Value::None),
					meta: Meta::new(vec![], None, vec![]) 
				})
			}
		};
		Self {
			type_hint: TypeHintNode::from(&mut node.children[0]),
			symbol: SymbolNode::from(&mut node.children[1]),
			mutable: node.tokens.iter().find(|t| t.source.as_str() == "mut").is_some(),
			value,
			meta: Meta::with_tokens(node.tokens.to_owned()),
		}
	}
}

impl AST for DeclarationNode {
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		let mut value = self.value.resolve(scope)?;
		
		// add meta info to value node
		value.set_meta_identifier(&self.symbol.0);
		
		let type_hint = self.type_hint.resolve_to_type_kind(scope)?;
		
		scope.set_symbol(&self.symbol, &RSymbolValue::Declaration { 
			type_hint, 
			mutable: self.mutable, 
			value
		})
	}
}