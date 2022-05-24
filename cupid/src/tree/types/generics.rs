use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GenericsNode(pub Vec<TypeHintNode>);

impl FromParent<&mut ParseNode> for Result<Option<GenericsNode>, Error> {
	fn from_parent(node: &mut ParseNode) -> Self {
		let generics_node = node.get_mut("generics");
		match generics_node.map(Result::<GenericsNode, Error>::from_parse) {
			Some(Ok(val)) => Ok(Some(val)),
			Some(Err(e)) => Err(e),
			None => Ok(None)
		}
	}
}

impl FromParent<&mut ParseNode> for Result<Vec<GenericsNode>, Error> {
	fn from_parent(node: &mut ParseNode) -> Self {
		node.filter_map(|n| if &*n.name == "generics" {
			Some(Result::<GenericsNode, Error>::from_parse(n))
		} else {
			None
		})
	}
}

impl FromParse for Result<GenericsNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		let generics = node.map(|g| {
			let arg = match g.children.get_mut(1).map(Result::<TypeHintNode, Error>::from_parse) {
				Some(Ok(value)) => Some(value),
				Some(Err(err)) => return Err(err),
				None => None
			};
			Ok(TypeHintNode::generic_arg(
				g.children[0].tokens[0].source.to_owned(),
				arg,
				g.children[0].tokens.to_owned()
			))
		})?;
		Ok(GenericsNode(generics))
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


impl Display for GenericsNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		let generics: Vec<String> = self.0.iter().map(|g| g.to_string()).collect();
		write!(f, "[{}]", generics.join(", "))
	}
}