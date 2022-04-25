use std::collections::HashMap;
use crate::{Expression, Value, Tree, LexicalScope, Token};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Map {
	pub items: Vec<(Expression, Expression)>,
	pub token: Token,
}

impl Tree for Map {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let items = self.items
			.iter()
			.enumerate()
			.map(|(i, (k, v))| {
				let key = if let Expression::Symbol(symbol) = k {
					symbol.identifier.clone()
				} else {
					k.resolve(scope)
				};
				let value = v.resolve(scope);
				(key, (i, value)) 
			});
		Value::Map(HashMap::from_iter(items))
	}
}
// 
// #[derive(Debug, Hash, Clone, PartialEq, Eq)]
// pub struct MapAccess {
// 	pub map: Box<Expression>,
// 	pub term: Box<Expression>,
// 	pub token: Token,
// }
// 
// impl Tree for MapAccess {
// 	fn resolve(&self, scope: &mut LexicalScope) -> Value {
// 		let map = crate::resolve_or_abort!(self.map, scope);
// 		let term = crate::resolve_or_abort!(self.term, scope);
// 		match (&term, &map) {
// 			(_, Value::Map(m)) => {
// 				m.get(&term).unwrap_or(&(0, self.no_property_error(&map, &term))).1.clone()
// 			},
// 			(_, m) => self.not_map_error(&m, &term),
// 		}
// 	}
// }
// 
// impl ErrorHandler for MapAccess {
// 	fn get_token(&self) -> &crate::Token {
// 		&self.token
// 	}
// 	fn get_context(&self) -> String {
// 		format!("Accessing property {} of map {}", self.term, self.map)
// 	}
// }
// 
// impl MapAccess {
// 	fn not_map_error(&self, map: &Value, accessor: &Value) -> Value {
// 		self.error(format!(
// 			"type mismatch: {} ({}) is not a map, and cannot be accessed by {} ({})",
// 			map,
// 			TypeKind::infer(map),
// 			accessor,
// 			TypeKind::infer(accessor)
// 		))
// 	}
// 	fn no_property_error(&self, map: &Value, accessor: &Value) -> Value {
// 		self.error(format!(
// 			"undefined: map {} doesn't have property {}",
// 			map,
// 			accessor
// 		))
// 	}
// }