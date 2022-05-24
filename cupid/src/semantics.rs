use crate::*;

fn box_node<'a, T: 'static + AST>(node: &'a mut ParseNode) -> Result<BoxAST, Error> 
	where Result<T, Error>: FromParse
{
	let result = Result::<T, Error>::from_parse(node)?;
	Ok(BoxAST::new(result))
}

pub fn parse(node: &mut ParseNode) -> Result<BoxAST, Error> {
	match &*node.name {
		"file" => box_node::<FileNode>(node),
		"expression"
		| "group" => parse(&mut node.children[0]),
		"empty" 
		| "comment"
		| "package" => Ok(BoxAST::new(EmptyNode)),
		
		// Type definitions
		"builtin_type_definition" => box_node::<BuiltinTypeNode>(node),
		"alias_type_definition" => box_node::<AliasTypeDeclaration>(node),
		"struct_type_definition" => box_node::<StructTypeDeclaration>(node),
		"sum_type_definition" => box_node::<SumTypeDeclaration>(node),
		
		// Type implementations
		"trait_definition" => box_node::<TraitNode>(node),
		"implement_type" => box_node::<UseBlockNode>(node),
		"implement_trait" => box_node::<UseTraitBlockNode>(node),
		
		// Variable definitions
		"typed_declaration" => box_node::<DeclarationNode>(node),
		"assignment" => box_node::<AssignmentNode>(node),
		
		// Type hints
		"array_type_hint"
		| "function_type_hint"
		| "map_type_hint"
		| "primitive_type_hint"
		| "struct_type_hint" => box_node::<TypeHintNode>(node),
		
		// Terms
		"block" => box_node::<BlockNode>(node),
		"if_block" => box_node::<IfBlockNode>(node),
		"for_loop" => box_node::<ForInLoopNode>(node),
		"while_loop" => box_node::<WhileLoopNode>(node),
		
		"type_cast" =>  if node.children.len() > 1 {
			Ok(BoxAST::new(TypeCastNode::parse_as_get_function(node)?))
		} else {
			 parse(&mut node.children[0])
		},
		
		"logic_op"
		| "compare_op" 
		| "add"
		| "multiply" 
		| "exponent"=> if node.children.len() > 1 {
			 Ok(BoxAST::new(OperationNode::parse_as_get_function(node)?))
		} else {
			 parse(&mut node.children[0])
		},
		
		"property" => if node.children.len() > 1 {
			Ok(BoxAST::new(OperationNode::parse_as_function(node)?))
		} else {
			 parse(&mut node.children[0])
		},
		
		"function_call" => if node.children.len() > 1 {
			box_node::<FunctionCallNode>(node)
		} else {
			 parse(&mut node.children[0])
		},
		
		"unary_op" => Ok(BoxAST::new(OperationNode::parse_as_get_function(node)?)),
		
		"log" => box_node::<LogNode>(node),
		
		// Atoms
		"builtin_function_call" => box_node::<BuiltinFunctionCallNode>(node),
		"function" => box_node::<FunctionNode>(node),
		
		"bracket_array" => box_node::<ArrayNode>(node),
		
		"map" => box_node::<MapNode>(node),
		"range" => box_node::<RangeNode>(node),
		
		"identifier"
		| "self"
		| "array_kw"
		| "map_kw"
		| "fun_kw" => box_node::<SymbolNode>(node),
		
		"boolean"
		| "none"
		| "char"
		| "string"
		| "decimal"
		| "number"
		| "pointer" => box_node::<ValueNode>(node),
		
		"error" => Err(Error::from_token(&node.tokens[0], "error", "error")),
		
		_ => panic!("unexpected node {node:?}")
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileNode {
	pub expressions: Vec<BoxAST>,
	pub meta: Meta<()>
}

impl FromParse for Result<FileNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
    	Ok(FileNode {
			expressions: node.map(&parse)?,
			meta: Meta::with_tokens(node.tokens.to_owned())
		})
	}
}

impl AST for FileNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let values = self.resolve_to(scope)?;
		
		let meta: Meta<Flag> = Meta {
			tokens: vec![],
			token_store: Some(scope.push_tokens(self.meta.tokens.to_owned())),
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

impl Display for FileNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "{self:?}")
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmptyNode;
impl AST for EmptyNode {
	fn resolve(&self, _scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		Ok(ValueNode::new_none())
		// Ok(())
	}
}

impl Display for EmptyNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "{self:?}")
	}
}