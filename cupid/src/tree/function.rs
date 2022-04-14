use std::hash::{Hash, Hasher};
use crate::{Symbol, LexicalScope, Value, Expression, Tree, Token};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Function {
	pub params: Vec<Symbol>,
	pub body: Box<Expression>,
}

impl Tree for Function {
	fn resolve(&self, _scope: &mut LexicalScope) -> Value {
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
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		_ = scope.add();
		
		let mut args: Vec<Value> = vec![];
		for arg in &self.args.0 {
			let val = arg.resolve(scope);
			if let Value::Error(e) = val {
				return Value::Error(e);
			}
			args.push(val);
		}
		
		if let Some(fun) = scope.get_symbol(&self.fun) {
			let (params, body) = match fun {
				Value::FunctionBody(params, body) => (params, body),
				_ => return Value::error(&self.fun.token, format!("`{}` is not a function", self.fun.get_identifier()))
			};
			FunctionCall::set_scope(scope, &params, args);
			let val = body.resolve(scope);
			scope.pop();
			val
		} else {
			Value::error(&self.fun.token, format!("function `{}` is not defined", self.fun.get_identifier()))
		}
	}
}

impl FunctionCall {
	fn set_scope(inner_scope: &mut LexicalScope, params: &[Symbol], args: Vec<Value>) {
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
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		// TODO fix so that identifier dicts don't resolve 
		let log_type = self.0.source.as_str();
		
		let mut strings = vec![];
		for arg in &self.1.0 {
			strings.push(format!("{}", arg.resolve(scope)))
		}
		
		let format = |arg: String| match log_type {
			"logs" | "logs_line" => format!("{} ", arg),
			_ => format!("{}", arg),
		};
		
		let mut string = String::new();
    	strings.iter().for_each(|arg| string.push_str(format(arg.clone()).as_str()));
		match log_type {
			"log" | "logs" => println!("{}", string),
			_ => print!("{}", string)
		};
		Value::None
	}
}

