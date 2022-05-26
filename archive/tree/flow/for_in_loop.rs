use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ForInLoopNode {
	pub symbols: Vec<SymbolNode>,
	pub map: BoxAST,
	pub body: BlockNode,
	pub meta: Meta<()>
}

impl FromParse for Result<ForInLoopNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		Ok(ForInLoopNode {
			symbols: if let Some(params) = node.get_mut("for_loop_parameters") {
				params.map(|s| Result::<SymbolNode, Error>::from_parse(s))?
			} else {
				vec![]
			},
			map: parse(&mut node.children[1])?,
			body: Result::<BlockNode, Error>::from_parse(&mut node.children[2])?,
			meta: Meta::with_tokens(node.tokens.to_owned()),
		})
	}
}

impl AST for ForInLoopNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let map_value = self.map.resolve(scope)?;
		let map: Vec<(ValueNode, ValueNode)> = match &map_value.value {
			Value::Array(a) => Value::Array(a.to_owned()).array_to_vec(),
			Value::Map(m) => Value::Map(m.to_owned()).map_to_vec(),
			_ => return Err(error_expected_map(&map_value, scope))
		};
		
		let (key_type_hint, value_type_hint) = if let Some(type_hint) = map_value.type_hint {
			let type_kind: TypeKind = type_hint.resolve_to(scope)?;
			match type_kind {
				TypeKind::Array(array_type) => (None, Some(array_type.element_type)),
				TypeKind::Map(map_type) => (Some(map_type.key_type), Some(map_type.value_type)),
				_ => (None, None)
			}
		} else {
			(None, None)
		};
		
		let mut result = ValueNode::from((Value::None, &Meta::<Flag>::from(&self.meta)));
		
		for (i, (key, value)) in map.into_iter().enumerate() {
			scope.add(Context::Loop);
			
			match self.symbols.len() {
				3 => {
					let index = ValueNode::from((Value::Integer(i as i32), &key.meta));
					set_loop_symbol(&self.symbols[0], index, &None, scope)?;
					set_loop_symbol(&self.symbols[1], key, &key_type_hint, scope)?;
					set_loop_symbol(&self.symbols[2], value, &value_type_hint, scope)?;
				},
				2 => {
					set_loop_symbol(&self.symbols[0], key, &key_type_hint, scope)?;
					set_loop_symbol(&self.symbols[1], value, &value_type_hint, scope)?;
				},
				1 => set_loop_symbol(&self.symbols[0], value, &value_type_hint, scope)?,
				_ => panic!("too many params")
			}
			result = self.body.resolve(scope)?;
			scope.pop();
		}
		Ok(result)
	}
}

fn set_loop_symbol(symbol: &SymbolNode, value: ValueNode, type_hint: &Option<TypeHintNode>, scope: &mut LexicalScope) -> Result<(), Error> {
	let type_hint = if let Some(type_hint) = type_hint {
		type_hint.to_owned()
	} else {
		TypeKind::infer_id(&value).unwrap()
	};
	let declaration = value.into_declaration_node(symbol.to_owned(), type_hint, false);
	declaration.resolve(scope)?;
	// value.set_meta_identifier(&symbol.0);
	// let mut declaration = value.into_declaration(false);
	// if let Some(type_hint) = type_hint {
	// 	declaration.type_hint = type_hint
	// }
	// scope.set_symbol(symbol, declaration)?;
	Ok(())
}

fn error_expected_map(node: &ValueNode, scope: &mut LexicalScope) -> Error {
	node.error(format!(
		"expected an array or map, found {node} ({})", 
		unwrap_or_string(&node.type_hint)
	), scope)
}


impl Display for ForInLoopNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "{self:?}")
	}
}