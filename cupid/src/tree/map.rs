use std::collections::HashMap;
use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MapNode<'src> {
	pub items: Vec<(BoxAST, BoxAST)>,
	pub meta: Meta<'src, ()>
}

impl<'src> From<&mut ParseNode<'_>> for MapNode<'src> {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			items: node.filter_map_mut(&|child| if &*child.name == "map_entry" {
				Some((parse(&mut child.children[0]), parse(&mut child.children[1])))
			} else {
				None
			}),
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
	}
}

impl<'src> AST for MapNode<'src> {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let mut items = HashMap::new();
    	for (i, (key_node, value_node)) in self.items.iter().enumerate() {
			let key = key_node.resolve(scope)?;
			let value = value_node.resolve(scope)?;
			items.insert(key, (i, value));
		}
		Ok(ValueNode::from((
			Value::Map(items), 
			&Meta::<Flag>::from(&self.meta)
		)))
	}
}