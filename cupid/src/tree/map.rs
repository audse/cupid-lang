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

impl From<&mut ParseNode> for MapNode {
	fn from(node: &mut ParseNode) -> Self {
		use MapKey::*;
		Self {
			items: node.filter_map_mut(&|child| if &*child.name == "map_entry" {
				Some((
					if child.children[0].name == "identifier" {
						Symbol(SymbolNode::from(&mut child.children[0]))
					} else {
						AST(parse(&mut child.children[0]))
					}, 
					parse(&mut child.children[1])
				))
			} else {
				None
			}),
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
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