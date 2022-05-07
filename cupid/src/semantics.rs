use crate::*;


pub fn parse(node: &mut ParseNode) -> BoxAST {
	match node.name.as_str() {
		"file" => return BoxAST::new(FileNode::from(node)),
		"expression" => return parse(&mut node.children[0]),
		
		"builtin_type_definition" => return BoxAST::new(BuiltinTypeNode::from(node)),
		"trait_definition" => return BoxAST::new(TraitNode::from(node)),
		"implement_type" => return BoxAST::new(UseBlockNode::from(node)),
		"implement_trait" => return BoxAST::new(UseTraitBlockNode::from(node)),
		
		"typed_declaration" => return BoxAST::new(DeclarationNode::from(node)),	
		"assignment" => return BoxAST::new(AssignmentNode::from(node)),
				
		"array_type_hint"
		| "function_type_hint"
		| "map_type_hint"
		| "primitive_type_hint"
		| "struct_type_hint" => return BoxAST::new(TypeHintNode::from(node)),
		
		"block" => return BoxAST::new(BlockNode::from(node)),
		
		"property_access" => return BoxAST::new(PropertyNode::from(node)),
		
		/* TODO */
		"compare_op" 
		| "add"
		| "multiply" 
		| "exponent" => if node.children.len() > 1 {
			return BoxAST::new(OperationNode::parse_as_function(node))
		} else {
			return parse(&mut node.children[0])
		},
		"unary_op" => return parse(&mut node.children[0]),
		/* END TODO */
		
		"log" => return BoxAST::new(LogNode::from(node)),
		"function_call" => return BoxAST::new(FunctionCallNode::from(node)),
		"function" => return BoxAST::new(FunctionNode::from(node)),
		"array" => return BoxAST::new(ArrayNode::from(node)),
		
		"identifier"
		| "self"
		| "array_kw"
		| "map_kw"
		| "fun_kw" => return BoxAST::new(SymbolNode::from(node)),
		
		"boolean"
		| "none"
		| "char"
		| "string"
		| "decimal"
		| "number" => return BoxAST::new(ValueNode::from(node)),
		
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