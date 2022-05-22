use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionNode {
	pub params: ParametersNode,
	pub body: BlockNode,
	pub meta: Meta<()>,
	pub scope: SingleScope,
}

impl FromParse for Result<FunctionNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		let tokens: Vec<Token> = node.collect_tokens();
		Ok(FunctionNode {
			params: Result::<ParametersNode, Error>::from_parse(&mut node.children[0])?,
			body: Result::<BlockNode, Error>::from_parse(&mut node.children[1])?,
			meta: Meta::with_tokens(tokens),
			scope: SingleScope::new(Context::Closure),
		})
	}
}

impl AST for FunctionNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		
		let mut meta = Meta::<Flag>::from(&self.meta);
		meta.set_token_store(scope);
		
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
	
	pub fn create_environment(&mut self, self_value: Option<ValueNode>, args: &ArgumentsNode, scope: &mut LexicalScope) -> Result<(), Error> {
		if let (Some(self_symbol), Some(self_value)) = (&self.params.self_symbol, self_value) {
			scope.to_result(self.scope.set_symbol(&self_symbol, self_value.into_declaration(self.params.mut_self)))?;
		}
		scope.add_closure(self.scope.to_owned());
		self.set_params(args, scope)?;
		Ok(())
	}
	pub fn drop_environment(&mut self, scope: &mut LexicalScope) -> Result<(), Error> {
		self.scope = scope.drop_closure();
		Ok(())
	}
	pub fn drop_and_modify(&mut self, symbol: &SymbolNode, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		self.drop_environment(scope)?;
		self.modify_closure(symbol, scope)
	}
	pub fn modify_closure(&self, symbol: &SymbolNode, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		scope.modify_symbol(symbol, &|symbol_value| match symbol_value {
			SymbolValue::Declaration { value, .. } => {
				if let Value::Function(_) = &mut value.value {
					value.value = Value::Function(self.to_owned())
				}
			}
			_ => (),
		})
	}
	pub fn match_params_to_args(&self, args: &ArgumentsNode, scope: &mut LexicalScope) -> Result<Vec<(Parameter, BoxAST)>, Error> {
		let params: &Vec<Parameter> = &self.params.symbols;
		if params.len() == args.0.len() {
			Ok(params
				.iter()
				.enumerate()
				.map(|(i, p)| ((*p).to_owned(), args.0[i].to_owned()))
				.collect())
		} else {
			Err(self.error_context(
				format!("wrong number of args: expected {}, found {}", self.params.symbols.len(), args.0.len()),
				format!("attempting to call function {self} with args ({args})"),
				scope
			))
		}
	}
	pub fn set_params(&self, args:  &ArgumentsNode, scope: &mut LexicalScope) -> Result<(), Error> {
		for (param, arg) in self.match_params_to_args(args, scope)? {
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
	pub fn get_self_symbol(&mut self) -> Option<ValueNode> {
		if let Some(symbol) = &self.params.self_symbol {
			return match self.scope.get_symbol(&symbol) {
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

impl ErrorHandler for FunctionNode {
	fn get_token<'a>(&'a self, scope: &'a mut LexicalScope) -> &'a Token {
		if self.meta.tokens.len() > 0 {
    		&self.meta.tokens[0]
		} else if let Some(token_store) = &self.meta.token_store {
			&scope.tokens[*token_store][0]
		} else {
			panic!("error occurred for {self:?} but there is no positional information")
		}
	}
	fn get_context(&self) -> String {
    	format!("accessing function ({:?})", self.params)
	}
}

impl Display for FunctionNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		let params: Vec<String> = self.params.symbols
			.iter()
			.map(|p| {
				let type_hint = if let Some(type_hint) = &p.type_hint {
					format!("{type_hint} ")
				} else {
					String::new()
				};
				format!("{}{}", type_hint, p.symbol.0)
			})
			.collect();
		write!(f, "fun ({})", params.join(", "))
	}
}