use std::collections::HashMap;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapKey {
	AST(BoxAST),
	Symbol(SymbolNode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapNode {
	pub items: Vec<(MapKey, BoxAST)>,
	pub meta: Meta<()>
}

impl From<&mut ParseNode> for Result<MapNode, Error> {
	fn from(node: &mut ParseNode) -> Self {
		use MapKey::*;
		let items = node.filter_map_mut_result(&|child| if &*child.name == "map_entry" {
			let key = if child.children[0].name == "identifier" {
				match Result::<SymbolNode, Error>::from(&mut child.children[0]) {
					Ok(value) => Symbol(value),
					Err(e) => return Some(Err(e))
				}
			} else {
				match parse(&mut child.children[0]) {
					Ok(ast) => AST(ast),
					Err(e) => return Some(Err(e))
				}
			};
			let value = match parse(&mut child.children[1]) {
				Ok(value) => value,
				Err(e) => return Some(Err(e))
			};
			Some(Ok((key, value)))
		} else {
			None
		})?;
		Ok(MapNode {
			items,
			meta: Meta::with_tokens(node.tokens.to_owned())
		})
	}
}

impl AST for MapNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut items = HashMap::new();
    	for (i, (key_node, value_node)) in self.items.iter().enumerate() {
			let key = match key_node {
				MapKey::Symbol(symbol) => symbol.0.to_owned(),
				MapKey::AST(node) => node.resolve(scope)?
			};
			let value = value_node.resolve(scope)?;
			items.insert(key, (i, value));
		}
		Ok(ValueNode::from((
			Value::Map(items), 
			&Meta::<Flag>::from(&self.meta)
		)))
	}
}