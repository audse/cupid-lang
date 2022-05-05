use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionNode {
	pub params: ParametersNode,
	pub body: BlockNode,
}

impl From<&mut ParseNode> for FunctionNode {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			params: ParametersNode::from(&mut node.children[0]),
			body: BlockNode::from(&mut node.children[1]),
		}
	}
}

impl AST for FunctionNode {
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		Ok(ValueNode::from_value(Value::FuncBody(self.to_owned())))
	}
}

impl FunctionNode {
	fn infer_return_type(&self) -> TypeKind { todo!() }
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
			.find(|t| t.source.as_str() == "mut")
			.is_some();
		let use_self = node.has("self");
		let symbols = node.map_mut(&|n: &mut ParseNode| match n.name.as_str() {
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
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		for Parameter { type_hint, symbol, .. } in self.symbols.iter() {
			let type_hint = if let Some(t) = type_hint {
				t.resolve_to_type_kind(scope)?
			} else {
				TypeKind::Type
			};
			let symbol_value = RSymbolValue::Declaration { 
				type_hint, 
				mutable: false, 
				value: ValueNode::new_none() 
			};
			scope.set_symbol(symbol, &symbol_value)?;
		}
		Ok(ValueNode::new_none())
	}
}
