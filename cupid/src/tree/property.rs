use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Property {
	FunctionCall(FunctionCallNode),
	Symbol(SymbolNode),
	Other(BoxAST),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PropertyNode {
	pub left: BoxAST,
	pub right: Property,
	pub meta: Meta<()>
}

impl From<&mut ParseNode> for PropertyNode {
	fn from(node: &mut ParseNode) -> Self {
		let left = parse(&mut node.children[0]);
		
		let right = match node.children[1].name.as_str() {
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
				if let Some(implementation_value) = self.resolve_implementation_function(&mut left, function_call, scope) {
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
				match left.value.get_property(&right.value) {
					Ok(value) => Ok(ValueNode::from_value(value)),
					Err(string) => Err(right.error_raw(string)),
				}
			}
		}
	}
}

impl PropertyNode {
	fn create_self_symbol(&self, function: &FunctionNode, value: &ValueNode, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let self_symbol = &function.params.symbols[0].symbol;
		let declare = SymbolValue::Declaration { 
			type_hint: value.type_kind.to_owned(), 
			mutable: function.params.mut_self,
			value: value.to_owned()
		};
		scope.set_symbol(self_symbol, &declare)
	}
	
	fn resolve_implementation_function(&self, left: &mut ValueNode, function_call: &FunctionCallNode, scope: &mut LexicalScope) -> Option<Result<ValueNode, Error>> {
		// the property is a function from a type or trait implementation
		if let Some(function) = left.type_kind.find_function_value(&function_call.function, scope) {
			if function.params.use_self {
				if let Err(e) = self.create_self_symbol(&function, left, scope) {
					return Some(Err(e))
				};
			}
			Some(function.call_function(&function_call.args, scope))
		} else {
			None
		}
	}
	
	fn resolve_map_function(&self, left: &ValueNode, function_call: &FunctionCallNode, scope: &mut LexicalScope) -> Option<Result<ValueNode, Error>> {
		// the property is a function within a map
		if let Ok(Value::Function(function)) = &left.value.get_property(&function_call.function.0.value) {
			if function.params.use_self {
				if let Err(e) = self.create_self_symbol(function, left, scope) {
					return Some(Err(e))
				};
			}
			Some(function.call_function(&function_call.args, scope))
		} else {
			None
		}
	}
	
	fn resolve_symbol_property(&self, left: &ValueNode, right: &SymbolNode, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		match &left.value {
			// try to get symbol's identifier first
			Value::Map(_) => match left.value.get_property(&right.0.value) {
				Ok(value) => Ok(ValueNode::from_value(value)),
				Err(_) => {
					// then try to get symbol's scoped meaning
					let property = right.resolve(scope)?;
					match left.value.get_property(&property.value) {
						Ok(value) => Ok(ValueNode::from_value(value)),
						Err(string) => Err(right.0.error_raw(string))
					}
				}
			},
			x => match x.get_property(&right.0.value) {
				Ok(value) => Ok(ValueNode::from_value(value)),
				Err(string) => Err(right.0.error_raw(string)),
			}
		}
	}
}

fn no_function_error(node: &ValueNode, function: &FunctionCallNode) -> Error {
	node.error_raw(format!("undefined: could not find function {} in {}", function.function, node))
}