use std::result::Result as R;
use std::hash::{Hash, Hasher};
use crate::{Symbol, Scope, Value, Expression, Tree, Token};

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

#[derive(Debug, Clone)]
pub struct FunctionCall {
	pub fun: Symbol,
	pub args: Args,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Args(pub Vec<Expression>);

impl Tree for FunctionCall {
	fn resolve(&self, scope: &mut Scope) -> Value {
		let mut inner_scope = scope.make_sub_scope();
		
		match self.resolve_args(scope) {
			Err(e) => e,
			Ok(args) => {
				if let Some(fun) = scope.get_symbol(&self.fun) {
					let (params, body) = match fun {
						Value::FunctionBody(params, body) => (params, body),
						_ => return Value::error(&self.fun.1[0], format!("`{}` is not a function", self.fun.get_identifier()))
					};
					FunctionCall::set_scope(&mut inner_scope, params, args);
					body.resolve(&mut inner_scope)
				} else {
					Value::error(&self.fun.1[0], format!("function `{}` is not defined", self.fun.get_identifier()))
				}
			}
		}
	}
}

impl FunctionCall {
	fn resolve_args(&self, scope: &mut Scope) -> R<Vec<Value>, Value> {
		self.args.0.iter().map(|arg| {
			let val = arg.resolve(scope);
			if val.is_poisoned() {
				Err(val)
			} else {
				Ok(val)
			}
		}).collect()
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

impl Hash for FunctionCall {
	fn hash<H: Hasher>(&self, _: &mut H) {}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Logger(pub Token, pub Args);

impl Tree for Logger {
	fn resolve(&self, scope: &mut Scope) -> Value {
		let log_type = self.0.source.as_str();
		let mut format = |arg: &Expression| match log_type {
			"logs" | "logs_line" => format!("{} ", arg.resolve(scope)),
			"log" | "log_line" | _ => format!("{}", arg.resolve(scope)),
		};
		let mut string = String::new();
    	self.1.0.iter().for_each(|arg| string.push_str(format(arg).as_str()));
		match log_type {
			"log" | "logs" => println!("{}", string),
			_ => print!("{}", string)
		};
		Value::None
	}
}

