use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FunctionFlag {
	Add,
	Subtract,
	Multiply,
	Divide,
	Modulus,
	Power,
	Equal,
	NotEqual,
	Less,
	LessEqual,
	Greater,
	GreaterEqual,
	And,
	Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallNode {
	pub function: SymbolNode,
	pub args: ArgumentsNode,
	pub meta: Meta<FunctionFlag>,
}

impl From<&mut ParseNode> for FunctionCallNode {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			function: SymbolNode::from(&mut node.children[0]),
			args: ArgumentsNode::from(&mut node.children[1]),
			meta: Meta::new(node.tokens.to_owned(), None, vec![])
		}
	}
}

impl AST for FunctionCallNode {
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		// check for builtin functions first
		if self.meta.flags.len() > 0 {
			return self.resolve_builtin_function(scope);
		}
		let function = self.function.resolve(scope)?;
		
		if let Value::Function(function) = &function.value {
			function.call_function(&self.args, scope)
		} else {
			panic!("not function: {function}")
		}
	}
}

impl FunctionCallNode {
	fn resolve_builtin_function(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		use FunctionFlag::*;
		use Value::*;
		let left = self.args.0[0].resolve(scope)?.value;
		let right = self.args.0[1].resolve(scope)?.value;
		// if let Some(body) = function.body
		let value = match self.meta.flags[..] {
			[Add, ..] => left + right,
			[Subtract, ..] => left - right,
			[Multiply, ..] => left * right,
			[Divide, ..] => left / right,
			[Modulus, ..] => left % right,
			[Power, ..] => left.pow(&right, &self.meta.tokens[0]),
			[Equal, ..] => Boolean(left == right),
			[NotEqual, ..] => Boolean(left != right),
			[Less, ..] => Boolean(left < right),
			[LessEqual, ..] => Boolean(left <= right),
			[Greater, ..] => Boolean(left > right),
			[GreaterEqual, ..] => Boolean(left >= right),
			[And, ..] => left & right,
			[Or, ..] => left | right,
			_ => left
		};
		Ok(ValueNode::from_value(value))
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgumentsNode(pub Vec<BoxAST>);

impl From<&mut ParseNode> for ArgumentsNode {
	fn from(node: &mut ParseNode) -> Self {
		Self(node.map_mut(&|c| BoxAST::from(parse(c))))
	}
}

impl AST for ArgumentsNode {
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		let mut values: Vec<Value> = vec![];
		for arg in self.0.iter() {
			let value = arg.resolve(scope)?;
			values.push(value.value);
		}
		Ok(ValueNode::from_value(Value::Values(values)))
	}
}