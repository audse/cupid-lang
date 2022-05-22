use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
	pub type_hint: Option<TypeHintNode>,
	pub symbol: SymbolNode,
	pub default: OptionAST,
}

impl Display for Parameter {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
    	write!(f,
			"{}{}{}",
			unwrap_or_string(&self.type_hint),
			self.symbol,
			if let OptionAST::Some(default) = &self.default {
				format!("= {default}")
			} else {
				String::new()
			}
		)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametersNode {
	pub symbols: Vec<Parameter>,
	pub self_symbol: Option<Box<SymbolNode>>,
	pub mut_self: bool,
}

impl FromParse for Result<ParametersNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		let mut_self = node.tokens
			.iter_mut()
			.any(|t| &*t.source == "mut");
		
		// TODO make separate impl?
		let symbols = node.filter_map_mut_result(&|n: &mut ParseNode| {
			let (type_hint, symbol, default) = match &*n.name {
				"annotated_parameter" => (
					match Result::<TypeHintNode, Error>::from_parse(&mut n.children[0]) {
						Ok(type_hint) => Some(type_hint),
						Err(e) => return Some(Err(e)),
					}, 
					match Result::<SymbolNode, Error>::from_parse(&mut n.children[1]) {
						Ok(symbol) => symbol,
						Err(e) => return Some(Err(e)),
					}, 
					OptionAST::None // TODO default vals
				),
				"self" => (
					None,
					match Result::<SymbolNode, Error>::from_parse(n) {
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
				Box::new(s.symbol.to_owned())
			});
		Ok(ParametersNode {
			symbols,
			self_symbol,
			mut_self
		})
	}
}

impl AST for ParametersNode {
	fn resolve(&self, _scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		unreachable!()
		// for Parameter { type_hint, symbol, .. } in self.symbols.iter() {
		// 	let symbol_value = SymbolValue::Declaration { 
		// 		type_hint: type_hint.to_owned(),
		// 		mutable: false, 
		// 		value: ValueNode::new_none() 
		// 	};
		// 	scope.set_symbol(symbol, symbol_value)?;
		// }
		// Ok(ValueNode::new_none())
	}
}

impl Display for ParametersNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		let params: Vec<String> = self.symbols.iter().map(|p| p.to_string()).collect();
		write!(f, "{}{}", if let Some(s) = &self.self_symbol {
			format!("{s}, ")
		} else {
			String::new()
		}, params.join(", "))
	}
}