use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionNode {
	pub params: ParametersNode,
	pub body: BlockNode,
	pub meta: Meta<()>,
}

impl From<&mut ParseNode> for FunctionNode {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			params: ParametersNode::from(&mut node.children[0]),
			body: BlockNode::from(&mut node.children[1]),
			meta: Meta::with_tokens(node.tokens.to_owned())
		}
	}
}

impl AST for FunctionNode {
	fn resolve(&self, _scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let meta = Meta::<Flag>::from(&self.meta);
		Ok(ValueNode::from((
			Value::Function(self.to_owned()), 
			&meta
		)))
	}
}

impl FunctionNode {
	// fn infer_return_type(&self) -> TypeKind { todo!() }
	
	pub fn call_function(&self, args: &ArgumentsNode, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		scope.add(Context::Function);
		self.set_params(args, scope)?;
		let value = self.body.resolve(scope);
		scope.pop();
		value
	}
	pub fn match_params_to_args(&self, args: &ArgumentsNode) -> Vec<(Parameter, BoxAST)> {
		let params: Vec<&Parameter> = self.params.symbols
			.iter()
			.filter(|p| p.type_hint.is_some())
			.collect();
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
		for (mut param, arg) in self.match_params_to_args(args) {
			
			let type_hint = if let Some(ref mut type_hint) = param.type_hint {
				type_hint.to_owned()
			} else {
				panic!("all params should have types ..")
			};
			
			let declaration = DeclarationNode {
				type_hint,
				symbol: param.symbol.to_owned(),
				value: arg.to_owned(),
				meta: Meta::new(vec![], None, vec![]),
				mutable: false,
			};
			declaration.resolve(scope)?;
		}
		Ok(())
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Parameter {
	pub type_hint: Option<TypeHintNode>,
	pub symbol: SymbolNode,
	pub default: OptionAST,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParametersNode {
	pub symbols: Vec<Parameter>,
	pub use_self: bool,
	pub mut_self: bool,
}

impl From<&mut ParseNode> for ParametersNode {
	fn from(node: &mut ParseNode) -> Self {
		let mut_self = node.tokens
			.iter_mut()
			.any(|t| &*t.source == "mut");
		let use_self = node.has("self");
		let symbols = node.map_mut(&|n: &mut ParseNode| match &*n.name {
				"annotated_parameter" => Parameter {
					type_hint: Some(TypeHintNode::from(&mut n.children[0])), 
					symbol: SymbolNode::from(&mut n.children[1]), 
					default: OptionAST::None // TODO default vals
				},
				"self" => {
					Parameter {
						type_hint: None, 
						symbol: SymbolNode::from(n), 
						default: OptionAST::None
					}
				},
				_ => panic!("unexpected params")
			});
		Self {
			symbols,
			use_self,
			mut_self
		}
	}
}

impl AST for ParametersNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		for Parameter { type_hint, symbol, .. } in self.symbols.iter() {
			
			// only set symbols with type hints (meaning not `self`)
			if let Some(type_hint) = type_hint {
				let symbol_value = SymbolValue::Declaration { 
					type_hint:  Some(type_hint.to_owned()), 
					mutable: false, 
					value: ValueNode::new_none() 
				};
				scope.set_symbol(symbol, symbol_value)?;
			}
		}
		Ok(ValueNode::new_none())
	}
}