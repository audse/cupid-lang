use crate::{Expression, Value, LexicalScope, Tree, Token, Block, Symbol};

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
			let condition = self.condition.resolve(scope);
			if let Value::Boolean(condition_value) = condition {
				if !condition_value {
					break;
				}
				value = self.body.resolve(scope);
				if let Value::Error(_) = value {
					break;
				}
			} else {
				return Value::error(&self.token, "a while loop condition must evaluate to a boolean");
			}
		}
		value
	}
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ForInLoop {
	pub map: Box<Expression>,
	pub body: Block,
	pub token: Token,
	pub params: Vec<Symbol>,
}

impl Tree for ForInLoop {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let map = match self.map.resolve(scope) {
			Value::Dictionary(x)
			| Value::List(x)
			| Value::Tuple(x) => x,
			x => return Value::error(&self.token, format!("expected a dictionary, list, or tuple, not {}", x))
		};
		
		let mut iter: Vec<(Value, (usize, Value))> = map.into_iter().collect();
		iter.sort_by(|(_, (a_index, _)), (_, (b_index, _))| a_index.cmp(b_index));
				
		let num_params = self.params.len();
		let mut result = Value::None;
		for (key, (index, value)) in iter.iter() {
			_ = scope.add();
			
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
			result = self.body.resolve(scope);
			scope.pop();
		}
		result
	}
}