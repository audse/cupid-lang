use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperationNode {
	pub left: BoxAST,
	pub right: BoxAST,
	pub operator: String,
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
	pub fn parse_as_function(node: &mut ParseNode) -> FunctionCallNode {
		use FunctionFlag::*;
		let (function, flag) = match node.tokens[0].source.as_str() {
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
			x => panic!("unrecognized operation {}", x),
		};
		let function = Value::String(function.to_string());
		let function_symbol = SymbolNode(ValueNode {
			type_kind: TypeKind::infer(&function),
			value: function,
			meta: Meta::with_tokens(node.tokens.to_owned()),
		});
		let left = parse(&mut node.children[0]);
		let right = parse(&mut node.children[1]);
		
		FunctionCallNode {
			function: function_symbol,
			args: ArgumentsNode(vec![left, right]),
			meta: Meta::new(node.tokens.to_owned(), None, vec![flag])
		}
	}
}