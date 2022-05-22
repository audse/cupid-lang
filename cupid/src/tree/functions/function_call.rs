use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallNode {
    pub function: SymbolNode,
    pub args: ArgumentsNode,
    pub meta: Meta<FunctionFlag>,
}

impl FromParse for Result<FunctionCallNode, Error> {
    fn from_parse(node: &mut ParseNode) -> Self {
        Ok(FunctionCallNode {
            function: Result::<SymbolNode, Error>::from_parse(&mut node.children[0])?,
            args: Result::<ArgumentsNode, Error>::from_parse(&mut node.children[1])?,
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
			let (first_arg, args) = self.args.resolve_as_method(scope)?;
            self.call_method(first_arg, args, scope)
        } else {
            self.call_function(scope)
        }
    }

    pub fn call_function(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
        // calls a function that is neither operation nor a property/implementation method
        let mut function = self.function.resolve(scope)?;

        if let Value::Function(function) = &mut function.value {
			function.create_environment(None, &self.args, scope)?;
			let value = function.body.resolve(scope)?;
            // update stored function with changed closure
            function.drop_and_modify(&self.function, scope)?;
            Ok(value)
        } else {
            Err(function.error(format!("expected a function, not {function}"), scope))
        }
    }
	
	pub fn call_method(
		&self,
		object: ValueNode,
		args: ArgumentsNode,
		scope: &mut LexicalScope,
	) -> Result<ValueNode, Error> {
		use FunctionFlag::*;
		use OperationFlag::*;
		
		match self.meta.flags[..] {
			[Operation(Get), ..] => return self.resolve_get(object, args, scope),
			_ => {
				let right_value = if args.0.len() > 1 {
					Some(args.resolve_nth(1, scope)?)
				} else {
					Option::None
				};
				OperationNode::resolve_as_default(&self, object, right_value, scope)
			}
		}
	}

    fn call_implemented_function(
        &self,
        object: ValueNode,
		args: ArgumentsNode,
        mut implementation: (Implementation, Option<Implementation>),
        mut function: FunctionNode,
        scope: &mut LexicalScope,
    ) -> Result<ValueNode, Error> {
        scope.add(Context::Implementation);
		
		implementation.0.infer_arguments(&object);
        implementation.0.set_generic_symbols(&object.meta, scope)?;
		if let Some(trait_implement) = implementation.1 {
			trait_implement.set_generic_symbols(&object.meta, scope)?;
		}
		
		function.create_environment(Some(object.to_owned()), &args, scope)?;
        let value = function.body.resolve(scope)?;
		
		// can't update associated/implemented method's closures, so we drop without modifying
		function.drop_environment(scope)?;
		scope.pop();

        // update original value, if calling a mutating method
        if let (Some(self_value), Some(symbol)) = (
			function.get_self_symbol(),
			&object.meta.identifier
		) {
            if function.params.mut_self {
                let symbol = &**symbol;
                scope.set_symbol(&SymbolNode(symbol.to_owned()), self_value.as_assignment())?;
            }
        }
        Ok(value)
    }
	fn resolve_get(&self, object: ValueNode, args: ArgumentsNode, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let property_arg = &args.0[0];
		if let Some(property_symbol) = &property_arg.as_symbol() {
			// if property is a symbol e.g. `get(person, name)`
			if let Ok(value) = object.get_property(&property_symbol.0, scope) {
				Ok(value)
			} else {
				let right_value = property_symbol.resolve(scope)?;
				object.get_property(&right_value, scope)
			}
		} else if let Some(property_function) = property_arg.as_function_call().cloned() {
			
			match object.get_method(&property_function.function, scope) {
				Ok((Some(implementation), function)) => {
					
					// resolve property's args first, within current scope
					let mut method_args: Vec<BoxAST> = property_function.args
						.resolve_to(scope)?
						.into_iter()
						.map(BoxAST::new)
						.collect();
					if function.params.self_symbol.is_some() {
						// TODO convert self symbol to pointer
						method_args.insert(0, BoxAST::new(object.to_owned()));
					}
					let method_args = ArgumentsNode(method_args);
					
					if function.empty() { // empty functions use default behavior
						property_function.call_method(object.to_owned(), method_args, scope)
					} else {
						property_function.call_implemented_function(object, method_args, implementation, function, scope)
					}
				},
				Ok((Option::None, _)) => {
					// todo? what happens here?
					property_function.call(scope)
				},
				Err(_) => {
					// println!("{}", e.string(""));
					let args: Vec<ValueNode> = args.resolve_to(scope)?;
					Err(error_not_implemented(&object, &property_function.function.0, &args, scope))
				}
			}
		} else {
			// if property is some other kind of accessor
			// e.g. `get(my_array, 0)`
			let property = property_arg.resolve(scope)?;
			object.get_property(&property, scope)
		}
	}
}

fn error_not_implemented(value: &ValueNode, function: &ValueNode, args: &Vec<ValueNode>, scope: &mut LexicalScope) -> Error {
    let args: Vec<String> = args
        .iter()
        .map(|a| format!("{a} [{}]", unwrap_or_string(&a.type_hint)))
        .collect();
    function.error_context(
        format!(
            "function `{function}` is not implemented for ({}, {})",
            unwrap_or_string(&value.type_hint),
            unwrap_or_string(&value.type_hint)
        ),
        format!(
            "attempting to call function `{function}` with args ({})",
            args.join(", ")
        ),
		scope
    )
}


impl Display for FunctionCallNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "{}({})", self.function, self.args)
	}
}