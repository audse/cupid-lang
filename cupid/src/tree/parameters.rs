use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
	pub type_hint: TypeHintNode,
	pub symbol: SymbolNode,
	pub default: OptionAST,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametersNode {
	pub symbols: Vec<Parameter>,
	pub self_symbol: Option<Box<SymbolNode>>,
	pub mut_self: bool,
}

impl From<&mut ParseNode> for ParametersNode {
	fn from(node: &mut ParseNode) -> Self {
		let mut_self = node.tokens
			.iter_mut()
			.any(|t| &*t.source == "mut");
		let self_symbol = if let Some(symbol) = node.get_mut("self") {
			Some(Box::new(SymbolNode::from(symbol)))
		} else {
			None
		};
		let symbols = node.filter_map_mut(&|n: &mut ParseNode| match &*n.name {
				"annotated_parameter" => Some(Parameter {
					type_hint: TypeHintNode::from(&mut n.children[0]), 
					symbol: SymbolNode::from(&mut n.children[1]), 
					default: OptionAST::None // TODO default vals
				}),
				"self" => None,
				_ => panic!("unexpected params, {n:?}")
			});
		Self {
			symbols,
			self_symbol,
			mut_self
		}
	}
}

impl AST for ParametersNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		for Parameter { type_hint, symbol, .. } in self.symbols.iter() {
			let symbol_value = SymbolValue::Declaration { 
				type_hint: Some(type_hint.to_owned()),
				mutable: false, 
				value: ValueNode::new_none() 
			};
			scope.set_symbol(symbol, symbol_value)?;
		}
		Ok(ValueNode::new_none())
	}
}