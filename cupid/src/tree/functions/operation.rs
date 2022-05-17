use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationNode {
	pub left: BoxAST,
	pub right: BoxAST,
	pub operator: Cow<'static, str>,
}

impl OperationNode {
	pub fn parse_as_get_function(node: &mut ParseNode) -> FunctionCallNode {
		let (function_name, flag) = get_operation(".");
		let left = parse(&mut node.children[0]);
		let right = BoxAST::new(Self::parse_as_function(node));
		
		FunctionCallNode {
			function: SymbolNode::from_value_and_tokens(
				Value::String(function_name.into()), 
				node.tokens.to_owned()
			),
			args: ArgumentsNode(vec![left, right]),
			meta: Meta::new(node.tokens.to_owned(), None, vec![flag.into()])
		}
	}
	pub fn parse_as_function(node: &mut ParseNode) -> FunctionCallNode {
		let (function_name, flag) = get_operation(&*node.tokens[0].source);
		let left = parse(&mut node.children[0]);
		let right = parse(&mut node.children[1]);
		let args = if function_name == "get" {
			vec![left, right]
		} else {
			vec![right]
		};
		FunctionCallNode {
			function: SymbolNode::from_value_and_tokens(
				Value::String(function_name.into()), 
				node.tokens.to_owned()
			),
			args: ArgumentsNode(args),
			meta: Meta::new(node.tokens.to_owned(), None, vec![flag.into()])
		}
	}
	pub fn resolve_as_default(
		function: &FunctionCallNode,
		mut left_value: ValueNode,
		right_value: ValueNode,
	) -> Result<ValueNode, Error> {	
		let left = left_value.value.to_owned();
		let right = right_value.value.to_owned();
	
		let value = if let Some(operation_flag) = function.meta.flags.get(0) {
			do_operation((*operation_flag).into(), left, right)
		} else {
			Err(format!("unrecognized operation: {:?}", function.meta.flags))
		};
		match value {
			Ok(value) => {
				left_value.value = value;
				Ok(left_value.to_owned())
			}
			Err(string) => Err(left_value.error_raw_context(
				string,
				format!(
					"attempting to perform operation {:?} on {left_value} and {right_value}",
					function.meta.flags
				),
			)),
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
		let (function_name, flag) = get_operation(".");	
		let left = parse(&mut node.children[0]);
		let right = TypeHintNode::from(&mut node.children[1]);
		
		FunctionCallNode {
			function: SymbolNode::from_value_and_tokens(
				Value::String(function_name.into()),
				node.tokens.to_owned()
			),
			args: ArgumentsNode(vec![left, BoxAST::new(right)]),
			meta: Meta::new(node.tokens.to_owned(), None, vec![flag.into()])
		}
	}
}