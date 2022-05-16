use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinFunctionCallNode(pub FunctionCallNode);

impl From<&mut ParseNode> for BuiltinFunctionCallNode {
	fn from(node: &mut ParseNode) -> Self {
    	Self(FunctionCallNode::from(node))
	}
}

impl AST for BuiltinFunctionCallNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let arguments = self.0.args.resolve_to(scope)?;
		let symbol = if let Some(node) = self.0.args.0.first() {
			(*node).as_symbol()
		} else {
			None
		};
		self.do_function(arguments, symbol, scope)
	}
}

fn string_split(mut start_arg: ValueNode, start_string: Str, pattern: Option<&Str>, n: Option<usize>) -> ValueNode {
	let string = start_string.to_string();
	let strings: Vec<&str> = if let Some(pattern) = pattern {
		let pattern = &(pattern.to_string());
		if let Some(n) = n {
			start_string.splitn(n, pattern).collect()
		} else {
			start_string.split(pattern).collect()
		}
	} else if let Some(n) = n {
		let (a, b) = start_string.split_at(n as usize);
		vec![a, b]
	} else {
		vec![&string]
	};
	let strings: Vec<ValueNode> = strings
		.iter()
		.map(|s| {
			let mut item = start_arg.to_owned();
			item.value = Value::String(s.to_string().into());
			item
		})
		.collect();
	start_arg.value = Value::Array(strings);
	start_arg.type_hint = TypeKind::infer_id(&start_arg);
	start_arg
}

impl BuiltinFunctionCallNode {
	fn do_function(&self, arguments: Vec<ValueNode>, symbol: Option<&SymbolNode>, scope: &mut LexicalScope) -> Result<ValueNode, Error> {		
		let mut mutate = |symbol: Option<&SymbolNode>, value: &ValueNode| -> Result<(), Error> {
			if let Some(symbol) = symbol {
				// TODO set parent of "self" IF mutable
				scope.set_symbol(symbol, value.as_assignment())?;
			}
			Ok(())
		};
		
		let func = self.0.function.get_identifier_string().to_owned();
		let mut start_arg = arguments[0].to_owned();
		let start_arg_copy = start_arg.to_owned();
		let mut other_args: Vec<ValueNode> = arguments.into_iter().skip(1).collect();
		match &mut start_arg.value {
			Value::String(string) => match func.as_str() {
				"split" => {
					if let Value::String(pattern) = &other_args[0].value {
						Ok(string_split(start_arg_copy, string.to_owned(), Some(pattern), None))
					} else {
						Err(self.0.function.error_raw("expected a string to split with"))
					}
				},
				"split_at" => {
					if let Value::Integer(index) = other_args[0].value {
						Ok(string_split(start_arg_copy, string.to_owned(), None, Some(index as usize)))
					} else {
						Err(self.0.function.error_raw("expected a index to split at"))
					}
				},
				"split_n" => {
					if let Value::String(pattern) = &other_args[0].value {
						if let Value::Integer(index) = other_args[1].value {
							Ok(string_split(start_arg_copy, string.to_owned(), Some(pattern), Some(index as usize)))
						} else {
							Err(self.0.function.error_raw("expected a index to split at"))
						}
					} else {
						Err(self.0.function.error_raw("expected a string to split with"))
					}
				},
				"char" => {
					if let Value::Integer(index) = other_args[0].value {
						if let Some(c) = string.chars().nth(index as usize) {
							start_arg.value = Value::Char(c);
							start_arg.type_hint = TypeKind::infer_id(&start_arg);
							Ok(start_arg)
						} else {
							Err(self.0.function.error_raw("no char at index"))
						}
					} else {
						Err(self.0.function.error_raw("expected char index"))
					}
				},
				"replace" => {
					if let (Value::String(pattern), Value::String(new)) = (&other_args[0].value, &other_args[1].value) {
						let pattern = &(pattern.to_string());
						let new = &(new.to_string());
						start_arg.value = Value::String(string.replace(pattern, new).into());
						Ok(start_arg)
					} else {
						Err(self.0.function.error_raw("expected pattern and replacement string"))
					}
				},
				"replace_n" => {
					if let (
						Value::String(pattern), 
						Value::String(new), 
						Value::Integer(n)
					) = (&other_args[0].value, &other_args[1].value, &other_args[2].value) {
						let pattern = &(pattern.to_string());
						let new = &(new.to_string());
						start_arg.value = Value::String(string.replacen(pattern, new, *n as usize).into());
						Ok(start_arg)
					} else {
						Err(self.0.function.error_raw("expected pattern, replacement string, and number of replacements"))
					}
				},
				"length" => {
					start_arg.value = Value::Integer(string.len() as i32);
					start_arg.type_hint = TypeKind::infer_id(&start_arg);
					Ok(start_arg)
				},
				_ => panic!("expected string function")
			},
			Value::Array(ref mut array) => match func.as_str() {
				"push" => {
					array.append(&mut other_args);
					mutate(symbol, &start_arg)?;
					Ok(start_arg)
				},
				"pop" => {
					let last_item = array.pop();
					if let Some(last_item) = last_item {
						mutate(symbol, &start_arg)?;
						Ok(last_item)
					} else {
						Err(self.0.function.error_raw("No elements in array to pop"))
					}
				},
				"remove" => {
					if let Value::Integer(index) = other_args[0].value {
						let item = array.remove(index as usize);
						mutate(symbol, &start_arg)?;
						Ok(item)
					} else {
						Err(self.0.function.error_raw("expected array index"))
					}
				},
				"insert" => {
					if let Value::Integer(index) = other_args[0].value {
						array.insert(index as usize, other_args[1].to_owned());
						mutate(symbol, &start_arg)?;
						Ok(start_arg)
					} else {
						Err(self.0.function.error_raw("expected array index as first argument"))
					}
				},
				"length" => {
					start_arg.value = Value::Integer(array.len() as i32);
					start_arg.type_hint = TypeKind::infer_id(&start_arg);
					Ok(start_arg)
				}
				_ => todo!()
			},
			Value::Map(ref mut map) =>  match func.as_str() {
				"get" => match start_arg.value.get_property(&other_args[0]) {
					Ok(val) => Ok(val),
					Err(e) => Err(start_arg.error_raw(e))
				},
				"set" => {
					match map.get_mut(&other_args[0]) {
						Some(val) => {
							*val = (val.0, other_args[1].to_owned());
						},
						None => {
							map.insert(other_args[0].to_owned(), (map.len(), other_args[1].to_owned()));
						}
					}
					mutate(symbol, &start_arg)?;
					Ok(start_arg)
				},
				"remove" => {
					if let Some((_, val)) = map.remove(&other_args[0]) {
						// TODO update all other indices?
						mutate(symbol, &start_arg)?;
						Ok(val)
					} else {
						Err(self.0.function.error_raw("map does not contain that property"))
					}
				},
				"length" => {
					start_arg.value = Value::Integer(map.len() as i32);
					start_arg.type_hint = TypeKind::infer_id(&start_arg);
					Ok(start_arg)
				},
				_ => todo!()
			}
			_ => todo!()
		}
	}
}