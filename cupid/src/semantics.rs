use crate::*;


pub fn parse(node: &mut ParseNode) -> BoxAST {
	match &*node.name {
		"file" => BoxAST::new(FileNode::from(node)),
		"expression" => parse(&mut node.children[0]),
		"empty" 
		| "comment"
		| "package" => BoxAST::new(EmptyNode),
		
		// Type definitions
		"builtin_type_definition" => BoxAST::new(BuiltinTypeNode::from(node)),
		"alias_type_definition" => BoxAST::new(AliasTypeDeclaration::from(node)),
		"struct_type_definition" => BoxAST::new(StructTypeDeclaration::from(node)),
		"sum_type_definition" => BoxAST::new(SumTypeDeclaration::from(node)),
		
		// Type implementations
		"trait_definition" => BoxAST::new(TraitNode::from(node)),
		"implement_type" => BoxAST::new(UseBlockNode::from(node)),
		"implement_trait" => BoxAST::new(UseTraitBlockNode::from(node)),
		
		// Variable definitions
		"typed_declaration" => BoxAST::new(DeclarationNode::from(node)),	
		"assignment" => BoxAST::new(AssignmentNode::from(node)),
		
		// Type hints
		"array_type_hint"
		| "function_type_hint"
		| "map_type_hint"
		| "primitive_type_hint"
		| "struct_type_hint" => BoxAST::new(TypeHintNode::from(node)),
		
		// Terms
		"block" => BoxAST::new(BlockNode::from(node)),
		"for_loop" => BoxAST::new(ForInLoopNode::from(node)),
		"while_loop" => BoxAST::new(WhileLoopNode::from(node)),
		"property_access" => BoxAST::new(PropertyNode::from(node)),
		
		"type_cast" =>  if node.children.len() > 1 {
			BoxAST::new(TypeCastNode::parse_as_function(node))
		} else {
			 parse(&mut node.children[0])
		},
		
		"compare_op" 
		| "add"
		| "multiply" 
		| "exponent" => if node.children.len() > 1 {
			 BoxAST::new(OperationNode::parse_as_function(node))
		} else {
			 parse(&mut node.children[0])
		},
		"unary_op" => parse(&mut node.children[0]), // TODO
		
		"log" => BoxAST::new(LogNode::from(node)),
		
		// Atoms
		"function_call" => BoxAST::new(FunctionCallNode::from(node)),
		"function" => BoxAST::new(FunctionNode::from(node)),
		"bracket_array" => BoxAST::new(ArrayNode::from(node)),
		"map" => BoxAST::new(MapNode::from(node)),
		
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
		
		_ => panic!("unexpected node {node:?}")
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
		let values = self.resolve_to(scope)?;
		let meta: Meta<Flag> = Meta {
			tokens: self.meta.tokens.to_owned(),
			identifier: None,
			flags: vec![]
		};
		Ok(ValueNode::new(Value::Values(values), meta))
	}
}

impl ResolveTo<Vec<ValueNode>> for FileNode {
	fn resolve_to(&self, scope: &mut LexicalScope) -> Result<Vec<ValueNode>, Error> {
		let mut values: Vec<ValueNode> = vec![];
		for exp in self.expressions.iter() {
			let value = exp.resolve(scope)?;
			values.push(value);
		}
		Ok(values)
	}
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmptyNode;
impl AST for EmptyNode {
	fn resolve(&self, _scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		Ok(ValueNode {
			value: Value::None,
			type_hint: None,
			meta: Meta::default()
		})
	}
}