use crate::*;


pub fn parse(node: &mut ParseNode) -> Box<dyn AST> {
	match node.name.as_str() {
		"file" => return Box::new(FileNode::from(node)),
		"expression" => return parse(&mut node.children[0]),
		
		"builtin_type_definition" => return Box::new(BuiltinTypeNode::from(node)),
		"implement_type" => return Box::new(UseBlockNode::from(node)),
		
		"typed_declaration" => return Box::new(DeclarationNode::from(node)),	
		"assignment" => return Box::new(AssignmentNode::from(node)),
		
		"array_type_hint"
		| "function_type_hint"
		| "map_type_hint"
		| "primitive_type_hint"
		| "struct_type_hint" => return Box::new(TypeHintNode::from(node)),
		
		"block" => return Box::new(BlockNode::from(node)),
		
		"function" => return Box::new(FunctionNode::from(node)),
		
		"identifier"
		| "self"
		| "array_kw"
		| "map_kw"
		| "fun_kw" => return Box::new(SymbolNode::from(node)),
		
		"boolean"
		| "none"
		| "char"
		| "string"
		| "decimal"
		| "number" => return Box::new(ValueNode::from(node)),
		
		_ => panic!("unexpected node {:?}", node)
	};
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileNode {
	pub expressions: Vec<BoxAST>,
	pub meta: Meta<()>
}

impl From<&mut ParseNode> for FileNode {
	fn from(node: &mut ParseNode) -> Self {
    	Self {
			expressions: node.children.iter_mut().map(|e| BoxAST::from(parse(e))).collect(),
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
	}
}

impl AST for FileNode {
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		let mut values: Vec<Value> = vec![];
		for exp in self.expressions.iter() {
			let value = exp.resolve(scope)?;
			values.push(value.value);
		}
		let meta: Meta<Flag> = Meta {
			tokens: self.meta.tokens.to_owned(),
			identifier: None,
			flags: vec![]
		};
		Ok(ValueNode::new(Value::Values(values), meta))
	}
}