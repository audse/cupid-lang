use std::fmt::{Display, Formatter, Result};
use crate::{Symbol, LexicalScope, Value, Expression, Tree, Token, ErrorHandler, FUNCTION};
use crate::utils::{pluralize, pluralize_word};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionCall {
	pub fun: Symbol,
	pub args: Args,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Args(pub Vec<Expression>);

impl Display for Args {
	fn fmt(&self, f: &mut Formatter) -> Result {
		_ = write!(f, "args ");
		for arg in self.0.iter() {
			_ = write!(f, "`{}`, ", arg)
		}
		Ok(())
	}
}

impl Tree for FunctionCall {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		scope.add();
		
		let mut args: Vec<Value> = vec![];
		for arg in &self.args.0 {
			let val = arg.resolve(scope);
			crate::abort_on_error!(val, { scope.pop(); });
			args.push(val);
		}
		
		if let Some(fun) = scope.get_symbol(&self.fun) {
			let (params, body) = match fun {
				Value::FunctionBody(params, body) => (params, body),
				Value::MapEntry(_, function) => match *function {
					Value::FunctionBody(params, body) => (params, body),
					_ => {
						scope.pop();
						return self.type_error(&function, FUNCTION);
					}
				} 
				_ => {
					scope.pop();
					return self.type_error(&fun, FUNCTION);
				}
			};
			if args.len() != params.len() {
				return self.num_arguments_error(params.len(), args.len());
			}
			FunctionCall::set_scope(scope, &params, args);
			let val = body.resolve(scope);
			scope.pop();
			val
		} else {
			scope.pop();
			self.undefined_error(self.fun.get_identifier())
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
	fn num_arguments_error(&self, num_params: usize, num_args: usize) -> Value {
		self.error(format!(
			"this function takes {} {}, but {} {} supplied",
			num_params,
			pluralize(num_params, "argument"),
			num_args,
			pluralize_word(num_args, "was")
		))
	}
}

impl ErrorHandler for FunctionCall {
	fn get_token(&self) -> &Token {
		&self.fun.token
	}
	fn get_context(&self) -> String {
		format!(
			"attempting to call function {} with {}", 
			self.fun.identifier, 
			self.args
		)
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Logger(pub Token, pub Args);

impl Tree for Logger {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		// TODO fix so that identifier dicts don't resolve 
		let log_type = self.0.source.as_str();
		
		let mut strings = vec![];
		for arg in &self.1.0 {
			let val = arg.resolve(scope);
			crate::abort_on_error!(val);
			strings.push(val.to_string());
		}
		
		let format = |arg: String| match log_type {
			"logs" | "logs_line" => format!("{} ", arg),
			_ => arg
		};
		
		let mut string = String::new();
    	strings.iter().for_each(|arg| string.push_str(format(arg.to_string()).as_str()));
		match log_type {
			"log" | "logs" => println!("{}", string),
			_ => print!("{}", string)
		};
		Value::None
	}
}


