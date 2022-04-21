use std::collections::HashMap;
use crate::{Expression, Value, LexicalScope, ScopeContext, Tree, Token, Block, Symbol, ErrorHandler, MapErrorHandler, Type, SymbolFinder};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct WhileLoop {
	pub condition: Box<Expression>,
	pub body: Block,
	pub token: Token
}

impl Tree for WhileLoop {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let mut value = Value::None;
		loop {
			scope.add(ScopeContext::Loop);
			let condition = crate::resolve_or_abort!(self.condition, scope, { scope.pop(); });
			if let Value::Boolean(condition_value) = condition {
				if !condition_value {
					break;
				}
				value = crate::resolve_or_abort!(self.body, scope, { scope.pop(); });
				if let Value::Break(break_value) = value {
					value = *break_value;
					break;
				}
			} else {
				scope.pop();
				return self.error(format!(
					"a while loop condition must evaluate to a boolean, not {} ({})",
					condition,
					Type::from(&condition)
				))
			}
			scope.pop();
		}
		scope.pop();
		value
	}
}
impl ErrorHandler for WhileLoop {
	fn get_token(&self) -> &Token {
		&self.token
	}
	fn get_context(&self) -> String {
		String::from("attempting to construct a while loop")
	}
}
impl MapErrorHandler for WhileLoop {}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ForInLoop {
	pub map: Box<Expression>,
	pub body: Block,
	pub token: Token,
	pub params: Vec<Symbol>,
}

impl Tree for ForInLoop {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		
		let map = crate::resolve_or_abort!(self.map, scope);
		
		let map_type = Type::from(&map);
		if !map_type.is_map() {
			return self.not_map_error(&map);
		}
		
		let inner_map: HashMap<Value, (usize, Value)> = map.inner_map_clone().unwrap();
		
		let mut iter: Vec<(Value, (usize, Value))> = inner_map.into_iter().collect();
		iter.sort_by(|(_, (a_index, _)), (_, (b_index, _))| a_index.cmp(b_index));
				
		let num_params = self.params.len();
		let mut result = Value::None;
		for (key, (index, value)) in iter.iter() {
			_ = scope.add(ScopeContext::Loop);
			
			let args = if num_params == 1 {
				vec![(self.params[0].clone(), value.clone())]
			} else if num_params == 2 {
				vec![
					(self.params[0].clone(), key.clone()), 
					(self.params[1].clone(), value.clone())
				]
			} else if num_params == 3 {
				vec![
					(self.params[0].clone(), Value::Integer(*index as i32)), 
					(self.params[1].clone(), key.clone()), 
					(self.params[2].clone(), value.clone())
				]
			} else {
				vec![]
			};
			
			for (symbol, value) in args {
				scope.create_symbol(&symbol, value.clone(), true, true);
			}
			
			result = crate::resolve_or_abort!(self.body, scope, { scope.pop(); });
			if let Value::Break(break_result) = result {
				result = *break_result;
				scope.pop();
				break
			}
			scope.pop();
		}
		result
	}
}

impl ErrorHandler for ForInLoop {
	fn get_token(&self) -> &Token {
		&self.token
	}
	fn get_context(&self) -> String {
		let params: Vec<String> = self.params.iter().map(|p| format!("`{}`", p.get_identifier())).collect();
		format!("attempting to construct a for..in loop with params {}", params.join(", "))
	}
}

impl MapErrorHandler for ForInLoop {}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Break {
	pub token: Token,
	pub value: Box<Expression>,
}

impl Tree for Break {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		Value::Break(Box::new(self.value.resolve(scope)))
	}
}

impl ErrorHandler for Break {
	fn get_token(&self) -> &Token {
		&self.token
	}
	fn get_context(&self) -> String {
		format!("breaking from loop with value {}", self.value)
	}
}