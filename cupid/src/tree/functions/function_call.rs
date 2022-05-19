use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallNode {
    pub function: SymbolNode,
    pub args: ArgumentsNode,
    pub meta: Meta<FunctionFlag>,
}

impl From<&mut ParseNode> for Result<FunctionCallNode, Error> {
    fn from(node: &mut ParseNode) -> Self {
        Ok(FunctionCallNode {
            function: Result::<SymbolNode, Error>::from(&mut node.children[0])?,
            args: Result::<ArgumentsNode, Error>::from(&mut node.children[1])?,
            meta: Meta::new(node.tokens.to_owned(), None, vec![]),
        })
    }
}

impl AST for FunctionCallNode {
    fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
        self.call(scope)
    }
    fn as_function_call(&self) -> Option<&FunctionCallNode> {
        Some(&self)
    }
}

impl FunctionCallNode {

    fn is_operation(&self) -> bool {
		matches!(&self.meta.flags.get(0), Some(FunctionFlag::Operation(_)))
			&& !self.args.empty()
    }

    fn call(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
        // Look for operation functions (including property access)
        if self.is_operation() {
            let first_arg = self.args.0[0].resolve(scope)?;
            self.call_operation(first_arg, scope)
        } else {
            self.call_normal_function(scope)
        }
    }

    pub fn call_normal_function(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
        // calls a function that is neither operation nor a property/implementation method
        let mut function = self.function.resolve(scope)?;

        if let Value::Function(function) = &mut function.value {
			function.create_environment(None, &self.args, scope)?;
			let value = function.body.resolve(scope)?;
            // update stored function with changed closure
            function.drop_and_modify(&self.function, scope)?;
            Ok(value)
        } else {
            Err(function.error_raw(format!("expected a function, not {function}")))
        }
    }
	
	pub fn call_operation(
		&self,
		left_value: ValueNode,
		scope: &mut LexicalScope,
	) -> Result<ValueNode, Error> {
		use FunctionFlag::*;
		use OperationFlag::*;
	
		match self.meta.flags[..] {
			[Operation(Get), ..] => return self.resolve_get(&left_value, &self.args, scope),
			_ => {
				let right_value = if !self.args.0.is_empty() {
					Some(self.args.0[0].resolve(scope)?)
				} else {
					Option::None
				};
				OperationNode::resolve_as_default(&self, left_value, right_value)
			}
		}
	}

    fn call_implemented_function(
        &self,
        first_arg: &ValueNode,
        implementation: (Implementation, Option<Implementation>),
        mut function: FunctionNode,
        scope: &mut LexicalScope,
    ) -> Result<ValueNode, Error> {
        scope.add(Context::Implementation);
		
        implementation.0.set_generic_symbols(&first_arg.meta, scope)?;
		if let Some(trait_implement) = implementation.1 {
			trait_implement.set_generic_symbols(&first_arg.meta, scope)?;
		}
		
		function.create_environment(Some(first_arg.to_owned()), &self.args, scope)?;
        let value = function.body.resolve(scope)?;
		
		// can't update associated/implemented method's closures, so we drop without modifying
		function.drop_environment(scope)?;
		scope.pop();

        // update original value, if calling a mutating method
        if let (Some(self_value), Some(symbol)) = (
			function.get_self_symbol(),
			&first_arg.meta.identifier
		) {
            if function.params.mut_self {
                let symbol = &**symbol;
                scope.set_symbol(&SymbolNode(symbol.to_owned()), self_value.as_assignment())?;
            }
        }
        Ok(value)
    }
	fn resolve_get(&self, value: &ValueNode, args: &ArgumentsNode, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let property_arg = &args.0[1];
		if let Some(property_symbol) = &property_arg.as_symbol() {
			// if property is a symbol e.g. `get(person, name)`
			if let Ok(value) = value.get_property(&property_symbol.0) {
				Ok(value)
			} else {
				let right_value = property_symbol.resolve(scope)?;
				value.get_property(&right_value)
			}
		} else if let Some(mut property_function) = property_arg.as_function_call().cloned() {
			match value.get_method(&property_function.function, scope) {
				Ok((Some(implementation), function)) => {
					if function.empty() { // empty functions use default behavior
						property_function.call_operation(value.to_owned(), scope)
					} else {
						self.insert_first_arg(&mut property_function, &function, scope)?;
						property_function.call_implemented_function(&value, implementation, function, scope)
					}
				},
				Ok((Option::None, _)) => {
					// todo? what happens here?
					property_function.call(scope)
				},
				Err(_) => {
					let args: Vec<ValueNode> = args.resolve_to(scope)?;
					Err(error_not_implemented(value, &property_function.function.0, &args))
				}
			}
		} else {
			// if property is some other kind of accessor
			// e.g. `get(my_array, 0)`
			let property = property_arg.resolve(scope)?;
			value.get_property(&property)
		}
	}
	
	fn insert_first_arg(&self, to_call: &mut FunctionCallNode, to_function: &FunctionNode, scope: &mut LexicalScope) -> Result<(), Error> {
		let first_arg: BoxAST = self.args.0[0].symbol_or_resolve(scope)?;
		if to_function.params.self_symbol.is_some() {
			to_call.args.0.insert(0, first_arg);
		}
		Ok(())
	}
}

fn error_not_implemented(value: &ValueNode, function: &ValueNode, args: &Vec<ValueNode>) -> Error {
    let args: Vec<String> = args
        .iter()
        .map(|a| format!("{a} [{}]", unwrap_or_string(&a.type_hint)))
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
        ),
    )
}
