use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GenericsNode(pub Vec<TypeHintNode>);

impl FromParent<&mut ParseNode> for Option<GenericsNode> {
	fn from_parent(node: &mut ParseNode) -> Self {
		let generics_node = node.children.iter_mut().find(|n| &*n.name == "generics");
		generics_node.map(GenericsNode::from)
	}
}

impl FromParent<&mut ParseNode> for Vec<GenericsNode> {
	fn from_parent(node: &mut ParseNode) -> Self {
		node.children
			.iter_mut()
			.filter(|n| &*n.name == "generics")
			.map(GenericsNode::from)
			.collect()
	}
}

impl From<&mut ParseNode> for GenericsNode {
	fn from(node: &mut ParseNode) -> Self {
		Self(node.children.iter_mut().map(|g| TypeHintNode::generic_arg(
			g.children[0].tokens[0].source.to_owned(),
			g.children.get_mut(1).map(TypeHintNode::from),
			g.children[0].tokens.to_owned()
		)).collect())
	}
}

impl AST for GenericsNode {
	fn resolve(&self, _scope: &mut LexicalScope) -> Result<ValueNode, Error> {
    	todo!()
	}
}

impl ResolveTo<Vec<GenericType>> for GenericsNode {
	fn resolve_to(&self, _scope: &mut LexicalScope) -> Result<Vec<GenericType>, Error> {
    	let mut generic_types: Vec<GenericType> = vec![];
		for generic in self.0.iter() {
			generic_types.push(GenericType {
				identifier: generic.identifier.to_owned(),
				type_value: if !generic.args.is_empty() {
					Some(generic.args[0].to_owned())
				} else {
					None
				}
			});
		}
		Ok(generic_types)
	}
}
