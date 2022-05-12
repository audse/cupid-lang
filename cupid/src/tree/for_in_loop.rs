use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ForInLoopNode {
	pub symbols: Vec<SymbolNode>,
	pub map: BoxAST,
	pub body: BlockNode,
	pub meta: Meta<()>
}

impl From<&mut ParseNode> for ForInLoopNode {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			symbols: if let Some(params) = node.get_mut("for_loop_parameters") {
				params.map_mut(&|s| SymbolNode::from(s))
			} else {
				vec![]
			},
			map: parse(&mut node.children[1]),
			body: BlockNode::from(&mut node.children[2]),
			meta: Meta::with_tokens(node.tokens.to_owned()),
		}
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
		
		for (i, (mut key, mut value)) in map.into_iter().enumerate() {
			scope.add(Context::Loop);
			match self.symbols.len() {
				3 => {
					// index
					let mut index = ValueNode::from((Value::Integer(i as i32), &key.meta));
					index.set_meta_identifier(&self.symbols[0].0);
					scope.set_symbol(&self.symbols[0], SymbolValue::from(index))?;
					
					// key
					key.set_meta_identifier(&self.symbols[1].0);
					scope.set_symbol(&self.symbols[1], SymbolValue::from(key))?;
					
					// value
					value.set_meta_identifier(&self.symbols[2].0);
					scope.set_symbol(&self.symbols[2], SymbolValue::from(value))?;
				},
				2 => {
					// key
					key.set_meta_identifier(&self.symbols[0].0);
					scope.set_symbol(&self.symbols[0], SymbolValue::from(key))?;
					
					// value
					value.set_meta_identifier(&self.symbols[1].0);
					scope.set_symbol(&self.symbols[1], SymbolValue::from(value))?;
				},
				1 => {
					// value
					value.set_meta_identifier(&self.symbols[0].0);
					scope.set_symbol(&self.symbols[0], SymbolValue::from(value))?;
				},
				_ => panic!("too many params")
			}
			result = self.body.resolve(scope)?;
			scope.pop();
		}
		Ok(result)
	}
}

fn error_expected_map(node: &ValueNode) -> Error {
	node.error_raw(format!(
		"expected an array or map, found {node} ({})", 
		unwrap_or_string(&node.type_hint)
	))
}