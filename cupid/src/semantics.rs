use crate::*;


pub fn parse(node: &mut ParseNode) -> BoxAST {
	match node.name.as_str() {
		"file" => BoxAST::new(FileNode::from(node)),
		"expression" => parse(&mut node.children[0]),
		
		"builtin_type_definition" => BoxAST::new(BuiltinTypeNode::from(node)),
		"trait_definition" => BoxAST::new(TraitNode::from(node)),
		"implement_type" => BoxAST::new(UseBlockNode::from(node)),
		"implement_trait" => BoxAST::new(UseTraitBlockNode::from(node)),
		
		"typed_declaration" => BoxAST::new(DeclarationNode::from(node)),	
		"assignment" => BoxAST::new(AssignmentNode::from(node)),
				
		"array_type_hint"
		| "function_type_hint"
		| "map_type_hint"
		| "primitive_type_hint"
		| "struct_type_hint" => BoxAST::new(TypeHintNode::from(node)),
		
		"block" => BoxAST::new(BlockNode::from(node)),
		
		"property_access" => BoxAST::new(PropertyNode::from(node)),
		
		/* TODO */
		"compare_op" 
		| "add"
		| "multiply" 
		| "exponent" => if node.children.len() > 1 {
			 BoxAST::new(OperationNode::parse_as_function(node))
		} else {
			 parse(&mut node.children[0])
		},
		"unary_op" => parse(&mut node.children[0]),
		/* END TODO */
		
		"log" => BoxAST::new(LogNode::from(node)),
		"function_call" => BoxAST::new(FunctionCallNode::from(node)),
		"function" => BoxAST::new(FunctionNode::from(node)),
		"array" => BoxAST::new(ArrayNode::from(node)),
		
		"identifier"
		| "self"
		| "array_kw"
		| "map_kw"
		| "fun_kw" => BoxAST::new(SymbolNode::from(node)),
		
		"boolean"
		| "none"
		| "char"
		| "string"
		| "decimal"
		| "number" => BoxAST::new(ValueNode::from(node)),
		
		_ => panic!("unexpected node {:?}", node)
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileNode {
	pub expressions: Vec<BoxAST>,
	pub meta: Meta<()>
}

impl From<&mut ParseNode> for FileNode {
	fn from(node: &mut ParseNode) -> Self {
    	Self {
			expressions: node.children.iter_mut().map(parse).collect(),
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
	}
}

impl AST for FileNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
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