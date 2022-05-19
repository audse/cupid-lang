use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationNode {
	pub left: BoxAST,
	pub right: Option<BoxAST>,
	pub operators: Vec<Token>,
}

impl From<&mut ParseNode> for Result<OperationNode, Error> {
	fn from(node: &mut ParseNode) -> Self {
		Ok(OperationNode {
			left: parse(&mut node.children[0])?,
			right: if node.children.len() > 1 {
				Some(parse(&mut node.children[1])?)
			} else {
				None
			},
			operators: node.tokens.to_owned()
		})
	}
}

impl From<OperationNode> for Result<FunctionCallNode, Error> {
	fn from(op: OperationNode) -> Self {
		let operators: Vec<&str> = op.operators.iter().map(|t| t.source()).collect();
		let (function_name, flag) = get_operation(&operators.join(" "));
		let mut args = vec![];
		if function_name == "get!" {
			args.push(op.left);
		}
		if let Some(right) = op.right {
			args.push(right);
		}
		Ok(FunctionCallNode {
			function: SymbolNode::from_value_and_tokens(
				Value::String(function_name.into()), 
				op.operators.to_owned()
			),
			args: ArgumentsNode(args),
			meta: Meta::new(op.operators, None, vec![flag.into()])
		})
	}
}

impl OperationNode {
	pub fn parse_as_get_function(node: &mut ParseNode) -> Result<FunctionCallNode, Error> {
		let (function_name, flag) = get_operation(".");
		let left = parse(&mut node.children[0])?;
		let right = BoxAST::new(Self::parse_as_function(node)?);
		
		Ok(FunctionCallNode {
			function: SymbolNode::from_value_and_tokens(
				Value::String(function_name.into()), 
				node.tokens.to_owned()
			),
			args: ArgumentsNode(vec![left, right]),
			meta: Meta::new(node.tokens.to_owned(), None, vec![flag.into()])
		})
	}
	pub fn parse_as_function(node: &mut ParseNode) -> Result<FunctionCallNode, Error> {
		Result::<FunctionCallNode, Error>::from(Result::<Self, Error>::from(node)?)
	}
	pub fn resolve_as_default(
		function: &FunctionCallNode,
		mut left_value: ValueNode,
		right_value: Option<ValueNode>,
	) -> Result<ValueNode, Error> {	
		let left = left_value.value.to_owned();
		let right = if let Some(right_value) = right_value {
			right_value.value.to_owned()
		} else {
			Value::None
		};
	
		let value = if let Some(operation_flag) = function.meta.flags.get(0) {
			do_operation((*operation_flag).into(), left, right.to_owned())
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
					"attempting to perform operation {:?} on {left_value} and {right}",
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
	pub fn parse_as_function(node: &mut ParseNode) -> Result<FunctionCallNode, Error> {
		let (function_name, flag) = get_operation(".");	
		let left = parse(&mut node.children[0])?;
		let right = Result::<TypeHintNode, Error>::from(&mut node.children[1])?;
		
		Ok(FunctionCallNode {
			function: SymbolNode::from_value_and_tokens(
				Value::String(function_name.into()),
				node.tokens.to_owned()
			),
			args: ArgumentsNode(vec![left, BoxAST::new(right)]),
			meta: Meta::new(node.tokens.to_owned(), None, vec![flag.into()])
		})
	}
}