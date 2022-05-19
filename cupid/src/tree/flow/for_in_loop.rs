use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ForInLoopNode {
	pub symbols: Vec<SymbolNode>,
	pub map: BoxAST,
	pub body: BlockNode,
	pub meta: Meta<()>
}

impl From<&mut ParseNode> for Result<ForInLoopNode, Error> {
	fn from(node: &mut ParseNode) -> Self {
		Ok(ForInLoopNode {
			symbols: if let Some(params) = node.get_mut("for_loop_parameters") {
				params.map_mut_result(&|s| Result::<SymbolNode, Error>::from(s))?
			} else {
				vec![]
			},
			map: parse(&mut node.children[1])?,
			body: Result::<BlockNode, Error>::from(&mut node.children[2])?,
			meta: Meta::with_tokens(node.tokens.to_owned()),
		})
	}
}

impl AST for ForInLoopNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let map_value = self.map.resolve(scope)?;
		let not_map_err = error_expected_map(&map_value);
		let map: Vec<(ValueNode, ValueNode)> = match map_value.value {
			Value::Array(a) => Value::Array(a).array_to_vec(),
			Value::Map(m) => Value::Map(m).map_to_vec(),
			_ => return Err(not_map_err)
		};
		let mut result = ValueNode::from((Value::None, &Meta::<Flag>::from(&self.meta)));
		
		for (i, (key, value)) in map.into_iter().enumerate() {
			scope.add(Context::Loop);
			
			match self.symbols.len() {
				3 => {
					let index = ValueNode::from((Value::Integer(i as i32), &key.meta));
					set_loop_symbol(&self.symbols[0], index, scope)?;
					set_loop_symbol(&self.symbols[1], key, scope)?;
					set_loop_symbol(&self.symbols[2], value, scope)?;
				},
				2 => {
					set_loop_symbol(&self.symbols[0], key, scope)?;
					set_loop_symbol(&self.symbols[1], value, scope)?;
				},
				1 => set_loop_symbol(&self.symbols[0], value, scope)?,
				_ => panic!("too many params")
			}
			result = self.body.resolve(scope)?;
			scope.pop();
		}
		Ok(result)
	}
}

fn set_loop_symbol(symbol: &SymbolNode, mut value: ValueNode, scope: &mut LexicalScope) -> Result<(), Error> {
	value.set_meta_identifier(&symbol.0);
	let declaration = value.into_declaration(false);
	scope.set_symbol(symbol, declaration)?;
	Ok(())
}

fn error_expected_map(node: &ValueNode) -> Error {
	node.error_raw(format!(
		"expected an array or map, found {node} ({})", 
		unwrap_or_string(&node.type_hint)
	))
}