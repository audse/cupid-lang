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
		self.create_environment(None, args, scope)?;
		let value = self.body.resolve(scope)?;
		self.drop_environment(scope)?;
		Ok((self, value))
	}
	pub fn call_function_with(&mut self, self_value: ValueNode, args: &ArgumentsNode, scope: &mut LexicalScope) -> Result<(ValueNode, ValueNode), Error> {
		self.create_environment(Some(self_value), args, scope)?;
		let value = self.body.resolve(scope)?;
		let self_symbol = self.get_self_symbol(scope).unwrap();
		Ok((self_symbol, value))
	}
	pub fn create_environment(&mut self, self_value: Option<ValueNode>, args: &ArgumentsNode, scope: &mut LexicalScope) -> Result<(), Error> {
		if let (Some(self_symbol), Some(self_value)) = (&self.params.self_symbol, self_value) {
			self.scope.set_symbol(&self_symbol, self_value.into_declaration(self.params.mut_self))?;
		}
		scope.add_closure(self.scope.to_owned());
		self.set_params(args, scope)?;
		Ok(())
	}
	pub fn drop_environment(&mut self, scope: &mut LexicalScope) -> Result<(), Error> {
		self.scope = scope.drop_closure();
		Ok(())
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
			panic!("wrong number of args: expected {}, found {}", self.params.symbols.len(), args.0.len())
		}
	}
	pub fn set_params(&self, args:  &ArgumentsNode, scope: &mut LexicalScope) -> Result<(), Error> {
		for (param, arg) in self.match_params_to_args(args) {
			if let Some(type_hint) = &param.type_hint {
				let type_hint: TypeHintNode = type_hint.resolve_to(scope)?;
				
				let declaration = DeclarationNode {
					type_hint,
					symbol: param.symbol.to_owned(),
					value: arg.to_owned(),
					meta: Meta::new(vec![], None, vec![]),
					mutable: true,
				};
				declaration.resolve(scope)?;
			};
		}
		Ok(())
	}
	pub fn get_self_symbol(&self, scope: &mut LexicalScope) -> Option<ValueNode> {
		if let Some(symbol) = &self.params.self_symbol {
			return match scope.get_symbol(&symbol) {
				Ok(val) => {
					Some(val)
				},
				Err(_) => None
			}
		}
		None
	}
	pub fn empty(&self) -> bool {
		self.body.expressions.is_empty()
	}
}
