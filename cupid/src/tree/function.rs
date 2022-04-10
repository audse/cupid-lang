use crate::{Symbol, Scope, Value, Expression, Tree};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Function {
	pub params: Vec<Symbol>,
	pub body: Box<Expression>,
}

impl Tree for Function {
	fn resolve(&self, _scope: &mut Scope) -> Value {
		Value::FunctionBody(self.params.clone(), self.body.clone())
	}
}

#[derive(Debug, Hash, Clone)]
pub struct FunctionCall {
	pub fun: Symbol,
	pub args: Vec<Expression>
}

impl Tree for FunctionCall {
	fn resolve(&self, scope: &mut Scope) -> Value {
		let mut inner_scope = scope.make_sub_scope();
		let args = self.resolve_args(scope);
		if let Some(fun) = scope.get_symbol(&self.fun) {
			let (params, body) = match fun {
				Value::FunctionBody(params, body) => (params, body),
				_ => panic!("Not a function")
			};
			FunctionCall::set_scope(&mut inner_scope, params, args);
			return body.resolve(&mut inner_scope);
		}
		Value::None
	}
}

impl FunctionCall {
	fn resolve_args(&self, scope: &mut Scope) -> Vec<Value> {
		(&self.args).iter().map(|arg| arg.resolve(scope)).collect()
	}
	fn set_scope(inner_scope: &mut Scope, params: &[Symbol], args: Vec<Value>) {
		for (index, param) in params.iter().enumerate() {
			let arg = &args[index];
			inner_scope.create_symbol(param, arg.clone(), true, true);
		}
	}
}

impl PartialEq for FunctionCall {
	fn eq(&self, _other: &Self) -> bool { false } // TODO
}
impl Eq for FunctionCall {}
