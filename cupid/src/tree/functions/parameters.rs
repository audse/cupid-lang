use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
	pub type_hint: Option<TypeHintNode>,
	pub symbol: SymbolNode,
	pub default: OptionAST,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametersNode {
	pub symbols: Vec<Parameter>,
	pub self_symbol: Option<Box<SymbolNode>>,
	pub mut_self: bool,
}

impl From<&mut ParseNode> for Result<ParametersNode, Error> {
	fn from(node: &mut ParseNode) -> Self {
		let mut_self = node.tokens
			.iter_mut()
			.any(|t| &*t.source == "mut");
		
		// TODO make separate impl?
		let symbols = node.filter_map_mut_result(&|n: &mut ParseNode| {
			let (type_hint, symbol, default) = match &*n.name {
				"annotated_parameter" => (
					match Result::<TypeHintNode, Error>::from(&mut n.children[0]) {
						Ok(type_hint) => Some(type_hint),
						Err(e) => return Some(Err(e)),
					}, 
					match Result::<SymbolNode, Error>::from(&mut n.children[1]) {
						Ok(symbol) => symbol,
						Err(e) => return Some(Err(e)),
					}, 
					OptionAST::None // TODO default vals
				),
				"self" => (
					None,
					match Result::<SymbolNode, Error>::from(n) {
						Ok(symbol) => symbol,
						Err(e) => return Some(Err(e)),
					}, 
					OptionAST::None // TODO default vals
				),
				_ => panic!("unexpected params, {n:?}")
			};
			Some(Ok(Parameter {
				type_hint, 
				symbol, 
				default
			}))
		})?;
		let self_symbol: Option<Box<SymbolNode>> = symbols
			.iter()
			.find(|s| s.type_hint.is_none())
			.map(|s| {
				let mut s = s.symbol.to_owned();
				s.0.type_hint = None;
				Box::new(s)
			});
		Ok(ParametersNode {
			symbols,
			self_symbol,
			mut_self
		})
	}
}

impl AST for ParametersNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		for Parameter { type_hint, symbol, .. } in self.symbols.iter() {
			let symbol_value = SymbolValue::Declaration { 
				type_hint: type_hint.to_owned(),
				mutable: false, 
				value: ValueNode::new_none() 
			};
			scope.set_symbol(symbol, symbol_value)?;
		}
		Ok(ValueNode::new_none())
	}
}