use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FunctionFlag {
	Add,
	Subtract,
	Multiply,
	Divide,
	Modulus,
	Power,
	Equal,
	NotEqual,
	Less,
	LessEqual,
	Greater,
	GreaterEqual,
	And,
	Or,
	As,
	IsType,
	
	Get,
	// Set,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallNode {
	pub function: SymbolNode,
	pub args: ArgumentsNode,
	pub meta: Meta<FunctionFlag>,
}

impl From<&mut ParseNode> for FunctionCallNode {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			function: SymbolNode::from(&mut node.children[0]),
			args: ArgumentsNode::from(&mut node.children[1]),
			meta: Meta::new(node.tokens.to_owned(), None, vec![])
		}
	}
}

impl AST for FunctionCallNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		self.call(scope)
	}
	
	fn as_function_call(&self) -> Option<&FunctionCallNode> { Some(&self) }
}

trait DoOperation {
	fn resolve_get(value: &ValueNode, args: &ArgumentsNode, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let property_arg = &args.0[1];
		if let Some(property_symbol) = &property_arg.as_symbol() {
			// if property is a symbol
			// e.g. `get(person, name)`
			if let Ok(value) = value.get_property(&property_symbol.0) {
				Ok(value)
			} else {
				let right_value = property_symbol.resolve(scope)?;
				value.get_property(&right_value)
			}
		} else if let Some(mut property_function) = property_arg.as_function_call().cloned() {
			match value.get_method(&property_function.function, scope) {
				Ok((Some(implementation), function)) => {
					if function.empty() {
						property_function.resolve_operation(value.to_owned(), scope)
					} else {
						let arg = if let Some(_) = &property_arg.as_symbol() {
							args.0[0].to_owned()
						} else {
							BoxAST::new(args.0[0].resolve(scope)?)
						};
						property_function.args.0.insert(0, arg);
						property_function.call_implemented_function(&value, implementation, function, scope)
					}
				},
				Ok((Option::None, _)) => {
					// todo? what happens here?
					property_function.call(scope)
				},
				Err(e) => Err(e)
			}
		} else {
			// if property is some other kind of accessor
			// e.g. `get(my_array, 0)`
			let property = property_arg.resolve(scope)?;
			value.get_property(&property)
		}
	}
	fn resolve_other_ops(function: &FunctionCallNode, mut left_value: ValueNode, right_value: ValueNode) -> Result<ValueNode, Error> {
		use FunctionFlag::*;
		use Value::*;
		
		let left = left_value.value.to_owned();
		let right = right_value.value.to_owned();
		
		let value = match function.meta.flags[..] {
			[Add, ..] => left + right,
			[Subtract, ..] => left - right,
			[Multiply, ..] => left * right,
			[Divide, ..] => left / right,
			[Modulus, ..] => left % right,
			[Power, ..] => left.pow(&right),
			[Equal, ..] => Ok(Boolean(left == right)),
			[NotEqual, ..] => Ok(Boolean(left != right)),
			[Less, ..] => left.compare(right, "<"),
			[LessEqual, ..] => left.compare(right, "<="),
			[Greater, ..] => left.compare(right, ">"),
			[GreaterEqual, ..] => left.compare(right, ">="),
			[And, ..] => left & right,
			[Or, ..] => left | right,
			[As, ..] => left.cast(right),
			_ => Err(format!("unrecognized operation: {:?}", function.meta.flags))
		};
		match value {
			Ok(value) => {
				left_value.value = value;
				Ok(left_value.to_owned())
			},
			Err(string) => {
				Err(left_value.error_raw_context(
					string,
					format!("attempting to perform operation {:?} on {left_value} and {right_value}", function.meta.flags)
				))
			}
		}
	}
}

impl DoOperation for FunctionCallNode {}


impl FunctionCallNode {
	
	pub fn resolve_operation(&self, left_value: ValueNode, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		use FunctionFlag::*;
		
		match self.meta.flags[..] {
			[Get, ..] => {
				return Self::resolve_get(&left_value, &self.args, scope)
			},
			_ => {
				let right_value = self.args.0[0].resolve(scope)?;
				Self::resolve_other_ops(&self, left_value, right_value)
			}
		}
	}
	
	fn is_builtin(&self) -> bool {
		!self.meta.flags.is_empty() && !self.args.0.is_empty()
	}
	
	fn call(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		// Look for builtin functions (including property access)
		if self.is_builtin() {
			let first_arg = self.args.0[0].resolve(scope)?;
			self.resolve_operation(first_arg, scope)
		} else {
			self.call_normal_function(scope)
		}
	}
	
	pub fn call_normal_function(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		// calls a function that is neither builtin nor a property/implementation method
		let mut function = self.function.resolve(scope)?;
		
		if let Value::Function(function) = &mut function.value {
			let (function, value) = function.call_function(&self.args, scope)?;
			
			// update stored function with changed closure
			scope.modify_symbol(&self.function, &|symbol_value| match symbol_value {
				SymbolValue::Declaration { value: fun_value, .. } => if let Value::Function(_) = fun_value.value {
					fun_value.value = Value::Function(function.to_owned());
				},
				_ => ()
			})?;
			Ok(value)
		} else {
			Err(function.error_raw(format!("expected a function, not {function}")))
		}
	}
	
	pub fn get_implemented_function_body(&self, first_arg: &ValueNode, scope: &mut LexicalScope) -> Result<Option<(Implementation, FunctionNode)>, Error> {
		if let Some(type_hint) = &first_arg.type_hint {
			let type_hint: TypeHintNode = type_hint.resolve_to(scope)?;
			let mut type_kind: TypeKind = type_hint.resolve_to(scope)?;
			if let Some(value) = type_kind.get_trait_function(&self.function, scope) {
				return Ok(Some(value));
			}
		}
		Ok(None)
	}
	
	fn call_implemented_function(&self, first_arg: &ValueNode, implementation: Implementation, mut function: FunctionNode, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		scope.add(Context::Implementation);
		implementation.set_generic_symbols(&first_arg.meta, scope)?;
		let (self_value, value) = function.call_function_with(first_arg.to_owned(), &self.args, scope)?;
		
		// update original value, if calling a mutating method
		if let Some(symbol) = &first_arg.meta.identifier {
			if function.params.mut_self {
				let symbol = &**symbol;
				scope.set_symbol(&SymbolNode(symbol.to_owned()), self_value.as_assignment())?;
			}
		}
		
		function.drop_environment(scope)?;
		
		scope.pop();
		Ok(value)
	}
}


fn error_not_implemented(value: &ValueNode, function: &ValueNode, args: &Vec<ValueNode>) -> Error {
	let args: Vec<String> = args
		.iter()
		.map(|a| format!(
			"{a} [{}]",
			unwrap_or_string(&a.type_hint)
		))
		.collect();
	function.error_raw_context(
		format!(
			"function `{function}` is not implemented for ({}, {})", 
			unwrap_or_string(&value.type_hint),
			unwrap_or_string(&value.type_hint)
		),
		format!(
			"attempting to call function `{function}` with args ({})",
			args.join(", ")
		)
	)
}