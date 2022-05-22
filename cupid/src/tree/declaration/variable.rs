use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeclarationNode {
	pub type_hint: TypeHintNode,
	pub symbol: SymbolNode,
	pub mutable: bool,
	pub value: BoxAST,
	pub meta: Meta<()>,
}

impl FromParse for Result<DeclarationNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		let value = if node.children.len() > 2 {
			parse(&mut node.children[2])?
		} else {
			BoxAST::new(ValueNode { 
				value: Value::None, 
				type_hint: None,
				meta: Meta::new(vec![], None, vec![]) 
			})
		};
		Ok(DeclarationNode {
			type_hint: Result::<TypeHintNode, Error>::from_parse(&mut node.children[0])?,
			symbol: Result::<SymbolNode, Error>::from_parse(&mut node.children[1])?,
			mutable: node.tokens.iter().any(|t| &*t.source == "mut"),
			value,
			meta: Meta::with_tokens(node.tokens.to_owned()),
		})
	}
}

impl AST for DeclarationNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut value = self.value.resolve(scope)?;
		
		// add meta info to value node
		value.set_meta_identifier(&self.symbol.0);
		
		let type_hint: TypeKind = self.type_hint.resolve_to(scope)?;
		let type_hint = if let TypeKind::Generic(type_kind) = type_hint {
			type_kind.type_value
		} else {
			Some(self.type_hint.to_owned())
		};
		
		value.type_hint = type_hint.to_owned();
		
		// TODO hacky fix
		// manual fix for arrays that have been incorrectly parsed as maps
		if let (Value::Map(map), Some(type_hint)) = (&value.value, &value.type_hint) {
			if map.is_empty() && &*type_hint.identifier == "array" {
				value.value = Value::Array(vec![]);
			}
		}
		
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

impl Display for DeclarationNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "{self:?}")
	}
}