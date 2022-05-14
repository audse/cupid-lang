use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Property {
	FunctionCall(FunctionCallNode),
	Symbol(SymbolNode),
	Other(BoxAST),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyNode {
	pub left: BoxAST,
	pub right: Property,
	pub meta: Meta<()>
}

impl From<&mut ParseNode> for PropertyNode {
	fn from(node: &mut ParseNode) -> Self {
		let left = parse(&mut node.children[0]);
		
		let right = match &*node.children[1].name {
			"function_call" => Property::FunctionCall(FunctionCallNode::from(&mut node.children[1])),
			"identifier" => Property::Symbol(SymbolNode::from(&mut node.children[1])),
			_ => Property::Other(parse(&mut node.children[1]))
		};
		Self {
			left,
			right,
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
	}
}

impl AST for PropertyNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		use Property::*;
		let mut left = self.left.resolve(scope)?;
		
		match &self.right {
			FunctionCall(function_call) => {
				if let Some(implementation_value) = function_call.call_implemented_function(&mut left, scope) {
					return implementation_value;
				}
				if let Some(map_value) = self.resolve_map_function(&left, function_call, scope) {
					return map_value;
				}
				Err(no_function_error(&left, function_call))
			},
			Symbol(symbol) => self.resolve_symbol_property(&left, symbol, scope),
			Other(value) => {
				let right = value.resolve(scope)?;
				match left.value.get_property(&right) {
					Ok(value) => Ok(value),
					Err(string) => Err(right.error_raw(string)),
				}
			}
		}
	}
}

impl PropertyNode {	
	fn resolve_map_function(&self, left: &ValueNode, function_call: &FunctionCallNode, scope: &mut LexicalScope) -> Option<Result<ValueNode, Error>> {
		// the property is a function within a map
		match &left.value.get_property(&function_call.function.0) {
			Ok(property) => if let Value::Function(function) = &property.value {
				if let Ok((_, value)) = function.to_owned().call_function(&function_call.args, scope) {
					Some(Ok(value))
				} else {
					None
				}
			} else {
				None
			},
			Err(e) => Some(Err(left.error_raw(e)))
		}
	}
	
	fn resolve_symbol_property(&self, left: &ValueNode, right: &SymbolNode, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		match &left.value {
			// try to get symbol's identifier first
			Value::Map(_) => match left.value.get_property(&right.0) {
				Ok(value) => Ok(value),
				Err(_) => {
					// then try to get symbol's scoped meaning
					let property = right.resolve(scope)?;
					match left.value.get_property(&property) {
						Ok(value) => Ok(value),
						Err(string) => Err(right.0.error_raw(string))
					}
				}
			},
			x => match x.get_property(&right.0) {
				Ok(value) => Ok(value),
				Err(string) => Err(right.0.error_raw(string)),
			}
		}
	}
}

fn no_function_error(node: &ValueNode, function: &FunctionCallNode) -> Error {
	function.function.error_raw_context(
		format!("undefined: could not find function {} in {node}", function.function),
		format!("accessing {node} (type {})", unwrap_or_string(&node.type_hint))
	)
}