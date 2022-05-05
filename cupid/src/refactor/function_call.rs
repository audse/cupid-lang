use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallNode {
	pub function: SymbolNode,
	pub args: ArgumentsNode
}

impl From<&mut ParseNode> for FunctionCallNode {
	fn from(node: &mut ParseNode) -> Self {
		Self {
			function: SymbolNode::from(&mut node.children[0]),
			args: ArgumentsNode::from(&mut node.children[1]),
		}
	}
}

impl AST for FunctionCallNode {
	fn resolve(&self, scope: &mut RLexicalScope) -> Result<ValueNode, Error> {
		// let function = self.function.resolve(scope)?;
		// scope.add(Context::Function);
		// function.params.resolve(scope)?;
		// let value = self.body.resolve(scope);
		// scope.pop();
		// value
		todo!()
	}
}

impl FunctionCallNode {
	fn set_params(&self, function: FunctionNode, scope: &mut RLexicalScope) {
		function.params.symbols.iter().enumerate().map(|(i, param)| {
			if param.type_hint.is_some() {
				let declaration = DeclarationNode {
					type_hint: param.type_hint.unwrap(),
					symbol: param.symbol,
					value: Box::new(*self.args.0[i]),
					meta: Meta::new(vec![], None, vec![]),
					mutable: false,
				};
				declaration.resolve(scope);
			}
		});
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
		todo!()
	}
}