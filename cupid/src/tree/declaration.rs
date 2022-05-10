use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeclarationNode<'src> {
	pub type_hint: TypeHintNode<'src>,
	pub symbol: SymbolNode<'src>,
	pub mutable: bool,
	pub value: BoxAST,
	pub meta: Meta<'src, ()>,
}

impl<'src> From<&mut ParseNode<'src>> for DeclarationNode<'src> {
	fn from(node: &mut ParseNode) -> Self {
		let value = if node.children.len() > 2 {
			parse(&mut node.children[2])
		} else {
			BoxAST::new(ValueNode { 
				value: Value::None, 
				type_kind: TypeKind::infer(&Value::None),
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

impl<'src> AST for DeclarationNode<'src> {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut value = self.value.resolve(scope)?;
		
		// add meta info to value node
		value.set_meta_identifier(&self.symbol.0);
		
		let type_hint = self.type_hint.resolve_to_type_kind(scope)?;
		
		// set symbol type as value's type
		let mut symbol = self.symbol.to_owned();
		symbol.0.type_kind = type_hint.to_owned();
		
		if type_hint.is_equal(&value.value) {
			value.type_kind = TypeKind::most_specific(&type_hint, &value.type_kind).to_owned();
		} else {
			return Err(value.error_raw_context(
				format!("type mismatch: cannot assign {value} to {symbol}"),
				format!("assigning type {} to type {type_hint}", TypeKind::infer(&value.value))
			));
		}
		
		scope.set_symbol(&symbol, SymbolValue::Declaration { 
			type_hint, 
			mutable: self.mutable, 
			value
		})
	}
}