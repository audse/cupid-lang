use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionNode {
	pub params: ParametersNode,
	pub body: BlockNode,
	pub meta: Meta<()>,
	pub scope: SingleScope,
}

impl From<&mut ParseNode> for FunctionNode {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			params: ParametersNode::from(&mut node.children[0]),
			body: BlockNode::from(&mut node.children[1]),
			meta: Meta::with_tokens(node.tokens.to_owned()),
			scope: SingleScope::new(Context::Closure),
		}
	}
}

impl AST for FunctionNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let meta = Meta::<Flag>::from(&self.meta);
		let mut function = self.to_owned();
		
		// save surrounding closure as scope
		if let Some(last_scope) = scope.last() {
			if last_scope.context == Context::Closure {
				function.scope = last_scope.to_owned();
			}
		}
		
		Ok(ValueNode::from((
			Value::Function(function), 
			&meta
		)))
	}
}

impl FunctionNode {
	// fn infer_return_type(&self) -> TypeKind { todo!() }
	
	pub fn call_function(&mut self, args: &ArgumentsNode, scope: &mut LexicalScope) -> Result<(&mut Self, ValueNode), Error> {
		scope.add_closure(self.scope.to_owned());
		
		self.set_params(args, scope)?;
		let value = self.body.resolve(scope)?;
		
		self.scope = scope.drop_closure();
		Ok((self, value))
	}
	pub fn match_params_to_args(&self, args: &ArgumentsNode) -> Vec<(Parameter, BoxAST)> {
		let params: &Vec<Parameter> = &self.params.symbols;
		if params.len() == args.0.len() {
			params
				.iter()
				.enumerate()
				.map(|(i, p)| ((*p).to_owned(), args.0[i].to_owned()))
				.collect()
		} else {
			panic!("wrong number of args")
		}
	}
	pub fn set_params(&self, args:  &ArgumentsNode, scope: &mut LexicalScope) -> Result<(), Error> {
		for (param, arg) in self.match_params_to_args(args) {
			let declaration = DeclarationNode {
				type_hint: param.type_hint.to_owned(),
				symbol: param.symbol.to_owned(),
				value: arg.to_owned(),
				meta: Meta::new(vec![], None, vec![]),
				mutable: true,
			};
			declaration.resolve(scope)?;
		}
		Ok(())
	}
	pub fn set_self_symbol(&self, value: ValueNode, scope: &mut LexicalScope) -> Result<(), Error> {
		if let Some(_) = &self.params.self_symbol {
			create_self_symbol(&self, value, scope)?;
		}
		Ok(())
	}
}
