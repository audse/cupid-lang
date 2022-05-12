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
	As,
	IsType,
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
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		
		// check for builtin/implemented functions first
		if !self.args.0.is_empty() {
			let mut left = self.args.0[0].resolve(scope)?;
			if let Some(value) = self.call_implemented_function(&mut left, scope) {
				return value;
			}
		}
		
		let function = self.function.resolve(scope)?;
		
		if let Value::Function(function) = &function.value {
			function.call_function(&self.args, scope)
		} else {
			Err(function.error_raw(format!("expected a function, not {function}")))
		}
	}
}

impl FunctionCallNode {
	pub fn resolve_builtin_function(&self, mut left_value: ValueNode, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		use FunctionFlag::*;
		use Value::*;
		let left = left_value.value.to_owned();
		let right = self.args.0[1].resolve(scope)?.value;
		
		let value = match self.meta.flags[..] {
			[Add, ..] => left + right,
			[Subtract, ..] => left - right,
			[Multiply, ..] => left * right,
			[Divide, ..] => left / right,
			[Modulus, ..] => left % right,
			[Power, ..] => match left.pow(&right, &self.meta.tokens[0]) {
				Ok(val) => val,
				Err(e) => return Err(left_value.error_raw(e))
			},
			[Equal, ..] => Boolean(left == right),
			[NotEqual, ..] => Boolean(left != right),
			[Less, ..] => Boolean(left < right),
			[LessEqual, ..] => Boolean(left <= right),
			[Greater, ..] => Boolean(left > right),
			[GreaterEqual, ..] => Boolean(left >= right),
			[And, ..] => left & right,
			[Or, ..] => left | right,
			[As, ..] => left.cast(right),
			_ => left
		};
		left_value.value = value;
		Ok(left_value)
	}
	
	pub fn call_implemented_function(&self, from: &mut ValueNode, scope: &mut LexicalScope) -> Option<Result<ValueNode, Error>> {
		let from_clone = from.to_owned();
		if let Some(type_hint) = &from.type_hint {
			let (_, mut type_kind) = match type_hint.resolve_to(scope) {
				Ok((s, t)) => (s, t),
				Err(e) => return Some(Err(e))
			};
			if let Some((implementation, function)) = type_kind.get_trait_function(&self.function, scope) {
				
				// use built in for empty function bodies
				if function.body.expressions.is_empty() && !self.meta.flags.is_empty() {
					return Some(self.resolve_builtin_function(from_clone, scope));
				}
				
				scope.add(Context::Function);
				for generic in implementation.generics.iter() {
					if let Err(e) = create_generic_symbol(generic, &from.meta, scope) {
						return Some(Err(e))
					}
				}
				if function.params.use_self {
					if let Err(e) = create_self_symbol(&function, from_clone, scope) {
						return Some(Err(e))
					};
				}
				
				let value = if !self.meta.flags.is_empty() {
					let args = ArgumentsNode(self.args.0.iter().skip(1).cloned().collect());
					function.call_function(&args, scope)
				} else {
					function.call_function(&self.args, scope)
				};
				
				scope.pop();
				Some(value)
			} else {
				if !self.meta.flags.is_empty() {
					_ = match self.args.0[1].resolve(scope) {
						Ok(right) => right,
						Err(e) => return Some(Err(e))
					};
					let args = match self.args.resolve_to(scope) {
						Ok(args) => args,
						Err(e) => return Some(Err(e))
					};
					return Some(Err(error_not_implemented(&from, &self.function.0, &args)));
				}
				None
			}
		} else {
			None
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgumentsNode(pub Vec<BoxAST>);

impl From<&mut ParseNode> for ArgumentsNode {
	fn from(node: &mut ParseNode) -> Self {
		Self(node.map_mut(&parse))
	}
}

impl AST for ArgumentsNode {
	fn resolve(&self, _scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		unreachable!("cannot resolve arguments as a whole")
	}
}

impl ResolveTo<Vec<ValueNode>> for ArgumentsNode {
	fn resolve_to(&self, scope: &mut LexicalScope) -> Result<Vec<ValueNode>, Error> {
		let mut values = vec![];
		for arg in self.0.iter() {
			let value = arg.resolve(scope)?;
			values.push(value);
		}
		Ok(values)
	}
}

fn error_not_implemented(value: &ValueNode, function: &ValueNode, args: &Vec<ValueNode>) -> Error {
	let args: Vec<String> = args
		.iter()
		.map(|a| format!(
			"{a} [{}]",
			unwrap_or_string(&a.type_hint)
		))
		.collect();
	function.error_raw_context(
		format!(
			"function `{function}` is not implemented for ({}, {})", 
			unwrap_or_string(&value.type_hint),
			unwrap_or_string(&value.type_hint)
		),
		format!(
			"attempting to call function `{function}` with args ({})",
			args.join(", ")
		)
	)
}