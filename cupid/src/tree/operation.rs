use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperationNode {
	pub left: BoxAST,
	pub right: BoxAST,
	pub operator: Cow<'static, str>,
}

impl From<&mut ParseNode> for OperationNode {
	fn from(node: &mut ParseNode) -> Self {
    	Self {
			left: parse(&mut node.children[0]),
			right: parse(&mut node.children[1]),
			operator: node.tokens[0].source.to_owned(),
		}
	}
}

impl OperationNode {
	pub fn parse_as_get_function(node: &mut ParseNode) -> FunctionCallNode {
		let (function, flag) = ("get", FunctionFlag::Get);
		let function = Value::String(function.into());
		let mut function_symbol = SymbolNode(ValueNode {
			type_hint: None,
			value: function,
			meta: Meta::with_tokens(node.tokens.to_owned()),
		});
		function_symbol.0.type_hint = TypeKind::infer_id(&function_symbol.0);
		let left = parse(&mut node.children[0]);
		let right = BoxAST::new(Self::parse_as_function(node));
		
		FunctionCallNode {
			function: function_symbol,
			args: ArgumentsNode(vec![left, right]),
			meta: Meta::new(node.tokens.to_owned(), None, vec![flag])
		}
	}
	pub fn parse_as_function(node: &mut ParseNode) -> FunctionCallNode {
		use FunctionFlag::*;
		let (function_name, flag) = match &*node.tokens[0].source {
			"+" => ("add", Add),
			"-" => ("subtract", Subtract),
			"*" => ("multiply", Multiply),
			"/" => ("divide", Divide),
			"%" => ("modulus", Modulus),
			"^" => ("power", Power),
			"is" => ("equal", Equal),
			"not" => ("not_equal", NotEqual),
			"<" => ("less", Less),
			"<=" => ("less_equal", LessEqual),
			">" => ("greater", Greater),
			">=" => ("greater_equal", GreaterEqual),
			"and" => ("and", And),
			"or" => ("or", Or),
			"as" => ("cast", As),
			"istype" => ("istype", IsType),
			"." => ("get", Get),
			_ => panic!("unrecognized operation"),
		};
		let function = Value::String(function_name.into());
		let mut function_symbol = SymbolNode(ValueNode {
			type_hint: None,
			value: function,
			meta: Meta::with_tokens(node.tokens.to_owned()),
		});
		function_symbol.0.type_hint = TypeKind::infer_id(&function_symbol.0);
		let left = parse(&mut node.children[0]);
		let right = parse(&mut node.children[1]);
		let args = if function_name == "get" {
			vec![left, right]
		} else {
			vec![right]
		};
		
		FunctionCallNode {
			function: function_symbol,
			args: ArgumentsNode(args),
			meta: Meta::new(node.tokens.to_owned(), None, vec![flag])
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeCastNode {
	pub left: BoxAST,
	pub right: TypeHintNode,
}

impl TypeCastNode {
	pub fn parse_as_function(node: &mut ParseNode) -> FunctionCallNode {
		let function = Value::String("cast".into());
		let mut function_symbol = SymbolNode(ValueNode {
			type_hint: None,
			value: function,
			meta: Meta::with_tokens(node.tokens.to_owned()),
		});
		function_symbol.0.type_hint = TypeKind::infer_id(&function_symbol.0);
		
		let left = parse(&mut node.children[0]);
		let right = TypeHintNode::from(&mut node.children[1]);
		
		FunctionCallNode {
			function: function_symbol,
			args: ArgumentsNode(vec![left, BoxAST::new(right)]),
			meta: Meta::new(node.tokens.to_owned(), None, vec![FunctionFlag::As])
		}
	}
}