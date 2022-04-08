use crate::{
	CupidSymbol,
	CupidScope,
	CupidValue,
	CupidExpression,
	FunctionBody,
	Tree,
};

#[derive(Debug, Hash, Clone)]
pub struct CupidFunction {
	pub params: Vec<CupidSymbol>,
	pub body: Box<CupidExpression>,
}

impl Tree for CupidFunction {
	fn resolve(&self, _scope: &mut CupidScope) -> CupidValue {
		let param_list = self.params.iter().map(|p| p.clone()).collect();
		return CupidValue::FunctionBody(FunctionBody(param_list, self.body.clone()));
	}
}

impl PartialEq for CupidFunction {
	fn eq(&self, _other: &Self) -> bool { return false; } // TODO
}
impl Eq for CupidFunction {}

#[derive(Debug, Hash, Clone)]
pub struct CupidFunctionCall {
	pub fun: CupidSymbol,
	pub args: Vec<CupidExpression>
}

impl Tree for CupidFunctionCall {
	fn resolve(&self, scope: &mut CupidScope) -> CupidValue {
		let mut inner_scope = scope.make_sub_scope();
		let args = self.resolve_args(scope);
		
		if let Some(fun) = scope.get_symbol(&self.fun) {
			let (params, body) = match fun {
				CupidValue::FunctionBody(FunctionBody(params, body)) => (params, body),
				_ => panic!("Not a function")
			};
			
			CupidFunctionCall::set_scope(&mut inner_scope, params, args);
			return body.resolve(&mut inner_scope);
		}
		return CupidValue::None;
	}
}

impl CupidFunctionCall {
	
	fn resolve_args(&self, scope: &mut CupidScope) -> Vec<CupidValue> {
		return (&self.args).iter().map(|arg| arg.resolve(scope)).collect();
	}
	
	fn set_scope(inner_scope: &mut CupidScope, params: &Vec<CupidSymbol>, args: Vec<CupidValue>) {
		let mut index = 0;
		for param in params {
			let arg = &args[index];
			inner_scope.set_symbol(&param, arg.clone());
			index += 1;
		}
	}
}

impl PartialEq for CupidFunctionCall {
	fn eq(&self, _other: &Self) -> bool { return false; } // TODO
}
impl Eq for CupidFunctionCall {}
