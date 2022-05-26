use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinFunctionCallNode(pub FunctionCallNode);

impl FromParse for Result<BuiltinFunctionCallNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
    	Ok(BuiltinFunctionCallNode(Result::<FunctionCallNode, Error>::from_parse(node)?))
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

fn get_string(from: &ValueNode, scope: &mut LexicalScope) -> Result<String, Error> {
	match &from.value {
		Value::String(pattern) => Ok(pattern.to_string()),
		Value::Char(pattern) => Ok(pattern.to_string()),
		x => Err(from.error(format!("expected a string or char, found {x}"), scope))
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
		match start_arg.value {
			Value::String(string) => match func.as_str() {
				"split" => {
					let pattern = get_string(&other_args[0], scope)?;
					Ok(string_split(start_arg_copy, string.to_owned(), Some(&pattern.into()), None))
				},
				"split_at" => {
					if let Value::Integer(index) = other_args[0].value {
						Ok(string_split(start_arg_copy, string.to_owned(), None, Some(index as usize)))
					} else {
						Err(self.0.function.error("expected a index to split at", scope))
					}
				},
				"split_n" => {
					let pattern = get_string(&other_args[0], scope)?;
					if let Value::Integer(index) = other_args[1].value {
						Ok(string_split(start_arg_copy, string.to_owned(), Some(&pattern.into()), Some(index as usize)))
					} else {
						Err(self.0.function.error("expected a index to split at", scope))
					}
				},
				"char" => {
					if let Value::Integer(index) = other_args[0].value {
						if let Some(c) = string.chars().nth(index as usize) {
							start_arg.value = Value::Char(c);
							start_arg.type_hint = TypeKind::infer_id(&start_arg);
							Ok(start_arg)
						} else {
							Err(self.0.function.error("no char at index", scope))
						}
					} else {
						Err(self.0.function.error("expected char index", scope))
					}
				},
				"replace" => {
					let pattern = get_string(&other_args[0], scope)?;
					let new = get_string(&other_args[1], scope)?;
					let value = string.replace(pattern.as_str(), new.as_str());
					start_arg.value = Value::String(value.into());
					Ok(start_arg)
				},
				"replace_n" => {
					let pattern = get_string(&other_args[0], scope)?;
					let new = get_string(&other_args[1], scope)?;
					if let Value::Integer(n) = &other_args[2].value {
						let value = string.replacen(pattern.as_str(), new.as_str(), *n as usize);
						start_arg.value = Value::String(value.into());
						Ok(start_arg)
					} else {
						Err(self.0.function.error("expected pattern, replacement string, and number of replacements", scope))
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
						Err(self.0.function.error("No elements in array to pop", scope))
					}
				},
				"remove" => {
					if let Value::Integer(index) = other_args[0].value {
						let item = array.remove(index as usize);
						mutate(symbol, &start_arg)?;
						Ok(item)
					} else {
						Err(self.0.function.error("expected array index", scope))
					}
				},
				"insert" => {
					if let Value::Integer(index) = other_args[0].value {
						array.insert(index as usize, other_args[1].to_owned());
						mutate(symbol, &start_arg)?;
						Ok(start_arg)
					} else {
						Err(self.0.function.error("expected array index as first argument", scope))
					}
				},
				"length" => {
					start_arg.value = Value::Integer(array.len() as i32);
					start_arg.type_hint = TypeKind::infer_id(&start_arg);
					Ok(start_arg)
				}
				x => todo!("{x} not implemented for array")
			},
			Value::Map(ref mut map) =>  match func.as_str() {
				"get" => match start_arg.value.get_property(&other_args[0]) {
					Ok(val) => Ok(val),
					Err(e) => Err(start_arg.error(e, scope))
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
						Err(self.0.function.error("map does not contain that property", scope))
					}
				},
				"length" => {
					start_arg.value = Value::Integer(map.len() as i32);
					start_arg.type_hint = TypeKind::infer_id(&start_arg);
					Ok(start_arg)
				},
				x => todo!("{x} not implemented for map")
			}
			z => todo!("no builtin methods for {z}")
		}
	}
}

impl Display for BuiltinFunctionCallNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "{self:?}")
	}
}