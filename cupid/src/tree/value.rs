use std::fmt::{Display, Formatter, Result};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::{Symbol, Expression, Error, Token, Type, ScopeContext, LexicalScope, TypeSymbol, SymbolFinder, DICTIONARY, LIST, TUPLE};
use std::ops::{Add, Sub, Mul, Neg, Div};
use std::cmp::Ordering;


#[derive(Debug, Clone)]
pub enum Value {
	Integer(i32),
	Decimal(i32, u32),
	String(String),
	Boolean(bool),
	FunctionBody(Vec<(TypeSymbol, Symbol)>, Box<Expression>),
	Dictionary(HashMap<Value, (usize, Value)>),
	List(HashMap<Value, (usize, Value)>),
	Tuple(HashMap<Value, (usize, Value)>),
	MapEntry(usize, Box<Value>),
	Error(Error),
	Type(Type),
	Break(Box<Value>),
	Return(Box<Value>),
	Continue,
	None,
}

impl Add for Value {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
    	match (self, rhs) {
			(Value::Integer(x), Value::Integer(y)) => Value::Integer(x + y),
			(Value::Decimal(x, y), Value::Decimal(a, b)) => float_to_dec(dec_to_float(x, y) + dec_to_float(a, b)),
			(Value::String(mut x), Value::String(y)) => {
				x.push_str(y.as_str());
				Value::String(x)
			},
			(Value::Dictionary(mut x), Value::Dictionary(y)) => {
				y.into_iter().for_each(|(entry, (_, value))| {
					x.insert(entry, (x.len(), value));
				});
				Value::Dictionary(x)
			},
			(Value::List(mut x), Value::List(y)) => {
				y.into_iter().for_each(|(entry, (_, value))| {
					x.insert(entry, (x.len(), value));
				});
				Value::List(x)
			},
			(Value::Tuple(mut x), Value::Tuple(y)) => {
				y.into_iter().for_each(|(entry, (_, value))| {
					x.insert(entry, (x.len(), value));
				});
				Value::Tuple(x)
			},
			_ => Value::None,
		}
	}
}

impl Sub for Value {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Value::Integer(x), Value::Integer(y)) => Value::Integer(x - y),
			(Value::Decimal(x, y), Value::Decimal(a, b)) => float_to_dec(dec_to_float(x, y) - dec_to_float(a, b)),
			_ => Value::None
		}
	}
}

impl Mul for Value {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Value::Integer(x), Value::Integer(y)) => Value::Integer(x * y),
			(Value::Decimal(x, y), Value::Decimal(a, b)) => float_to_dec(dec_to_float(x, y) * dec_to_float(a, b)),
			_ => Value::None
		}
	}
}

impl Div for Value {
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Value::Integer(x), Value::Integer(y)) => Value::Integer(x / y),
			(Value::Decimal(x, y), Value::Decimal(a, b)) => float_to_dec(dec_to_float(x, y) / dec_to_float(a, b)),
			_ => Value::None
		}
	}
}

impl Neg for Value {
	type Output = Self;
	fn neg(self) -> Self::Output {
		match self {
			Value::Integer(x) => Value::Integer(-x),
			Value::Decimal(x, y) => float_to_dec(-dec_to_float(x, y)),
			_ => Value::None
		}
	}
}

impl PartialOrd for Value {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match (self, other) {
			(Value::Integer(x), Value::Integer(y)) => Some(x.cmp(y)),
			(Value::Decimal(x, y), Value::Decimal(a, b)) => {
				let x = dec_to_float(*x, *y);
				let y = dec_to_float(*a, *b);
				if x > y {
					Some(Ordering::Greater)
				} else if x == y {
					Some(Ordering::Equal)
				} else {
					Some(Ordering::Less)
				}
			},
			_ => None
		}
	}
}

impl PartialEq for Value {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Value::Integer(x), Value::Integer(y)) => x == y,
			(Value::String(x), Value::String(y)) => x == y,
			(Value::Decimal(a, b), Value::Decimal(x, y)) => a == x && b == y,
			(Value::Boolean(x), Value::Boolean(y)) => x == y,
			(Value::FunctionBody(x, _), Value::FunctionBody(y, _)) => x == y,
			(Value::Type(x), Value::Type(y)) => x == y,
			(Value::None, Value::None) => true,
			(Value::Error(_), Value::Error(_)) => false,
			(Value::Break(x), Value::Break(y)) => x == y,
			_ => false
		}
	}
}

impl Eq for Value {}

impl Hash for Value {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Value::Integer(x) => x.hash(state),
			Value::String(x) => x.hash(state),
			Value::Boolean(x) => x.hash(state),
			Value::Error(x) => x.hash(state),
			Value::Decimal(x, y) => {
				x.hash(state);
				y.hash(state);
			},
			Value::FunctionBody(x, y) => {
				x.hash(state);
				y.hash(state);
			},
			Value::None => (),
			Value::Dictionary(x)
			| Value::List(x)
			| Value::Tuple(x) => for entry in x.iter() {
				entry.hash(state)
			},
			Value::MapEntry(x, y) => {
				x.hash(state);
				y.hash(state);
			},
			Value::Type(x) => x.hash(state),
			Value::Break(x) => x.hash(state),
			Value::Return(x) => x.hash(state),
			Value::Continue => (),
		}
	}
}


pub fn dec_to_float(front: i32, back: u32) -> f64 {
	format!("{}.{}", front, back).parse::<f64>().unwrap()
}

pub fn float_to_dec(float: f64) -> Value {
	let string = float.to_string();
	let mut parts: Vec<&str> = string.split('.').collect();
	parts.push("0");
	Value::Decimal(parts[0].parse::<i32>().unwrap(), parts[1].parse::<u32>().unwrap())
}

impl Value {
	pub fn error<S>(token: &Token, message: S, context: S) -> Value where S: Into<String> {
		Value::Error(Error::from_token(token, &message.into(), &context.into()))
	}
	pub fn inner_map(&self) -> Option<&HashMap<Value, (usize, Value)>> {
		match &self {
			Self::Dictionary(m)
			| Self::List(m) 
			| Self::Tuple(m) => Some(m),
			_ => None
		}
	}
	pub fn inner_map_clone(&self) -> Option<HashMap<Value, (usize, Value)>> {
		self.inner_map().cloned()
	}
	pub fn wrap_map(map_type: &Type, map: HashMap<Value, (usize, Value)>) -> Self {
		if map_type == &DICTIONARY {
			Value::Dictionary(map)
		} else if map_type == &LIST {
			Value::List(map)
		} else if map_type == &TUPLE {
			Value::Tuple(map)
		} else {
			Value::Dictionary(map)
		}
	}
	pub fn make_scope(&self, scope: &mut LexicalScope, token: Token, mutable: bool) {
		scope.add(ScopeContext::Map);
		let self_symbol = Symbol::new_string("self".to_string(), token.clone());
		scope.create_symbol_of_type(
			&self_symbol, self.clone(), Type::from(self), mutable, mutable
		);
		if let Some(map) = self.inner_map() {
			for (key, (index, value)) in map.iter() {
				let symbol = Symbol { identifier: key.clone(), token: token.clone() };
				let entry_value = Value::MapEntry(*index, Box::new(value.clone()));
				let entry_type = Type::from(&value);
				scope.create_symbol_of_type(&symbol, entry_value, entry_type, mutable, mutable);
			}
		}
	}
	pub fn is_poisoned(&self) -> bool {
		matches!(self, Value::Error(_))
	}
	pub fn sort_by_index(&self) -> Vec<(usize, &Value)> {
		match self {
			Self::List(l)
			| Self::Tuple(l) => {
				let mut vec: Vec<(usize, &Value)> = l
					.iter()
					.map(|(key, (_, value))| {
						if let Value::Integer(i) = key {
							(*i as usize, value)
						} else {
							(0, value)
						}
					})
					.collect();
				vec.sort_by(|a, b| a.0.cmp(&b.0));
				vec
			},
			_ => unreachable!()
		}
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			Self::Integer(v) => write!(f, "{}", v),
			Self::Decimal(v, w) => write!(f, "{}.{}", v, w),
			Self::Error(e) => write!(f, "{}", e), // TODO change this
			Self::String(s) => write!(f, "{}", s),
			Self::Dictionary(d) => {
				_ = write!(f, "[ ");
				for (key, (_, val)) in d.iter() {
					_ = write!(f, "{}: {}, ", key, val);
				}
				_ = write!(f, "]");
				Ok(())
			},
			Self::MapEntry(_, v) => write!(f, "{}", v),
			Self::List(l) => {
				_ = write!(f, "[");
				let vec = self.sort_by_index();
				for (i, val) in vec {
					if i != l.len() - 1 {
						_ = write!(f, "{}, ", val);
					} else {
						_ = write!(f, "{}", val);
					}
				}
				_ = write!(f, "]");
				Ok(())
			},
			Self::FunctionBody(params, _) => {
				_ = write!(f, "(fun => ");
				for param in params.iter() {
					_ = write!(f, "({}) {}, ", param.0.name, param.1.identifier);
				}
				_ = write!(f, ")");
				Ok(())
			}
			v => write!(f, "{:?}", v)
		}
	}
}