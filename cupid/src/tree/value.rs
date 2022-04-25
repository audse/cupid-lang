use std::fmt::{Display, Formatter, Result};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::{TypeKind, Symbol, Expression, Error, Token};
use std::ops::{Add, Sub, Mul, Neg, Div, Rem, BitAnd, BitOr};
use std::cmp::Ordering;


#[derive(Debug, Clone)]
pub enum Value {
	Integer(i32),
	Decimal(i32, u32),
	Char(char),
	Array(Vec<Box<Value>>),
	String(String),
	Boolean(bool),
	FunctionBody(Vec<(Value, Symbol)>, Box<Expression>),
	Error(Error),
	Type(TypeKind),
	Break(Box<Value>),
	Return(Box<Value>),
	Map(HashMap<Value, (usize, Value)>),
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
			(Value::Map(mut x), Value::Map(y)) => {
				y.into_iter().for_each(|(entry, (_, value))| {
					x.insert(entry, (x.len(), value));
				});
				Value::Map(x)
			},
			(Value::Array(mut x), Value::Array(mut y)) => {
				x.append(&mut y);
				Value::Array(x)
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

impl Rem for Value {
	type Output = Self;
	fn rem(self, rhs: Self) -> Self::Output {
    	match (self, rhs) {
			(Value::Integer(x), Value::Integer(y)) => Value::Integer(x % y),
			(Value::Decimal(x, y), Value::Decimal(a, b)) => float_to_dec(dec_to_float(x, y) % dec_to_float(a, b)),
			_ => Value::None
		}
	}
}

impl BitAnd for Value {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self::Output {
		let left_truthy = self.is_truthy();
		let right_truthy = rhs.is_truthy();
		if left_truthy && right_truthy {
			rhs
		} else {
			self
		}
	}
}

impl BitOr for Value {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self::Output {
		let left_truthy = self.is_truthy();
		if left_truthy {
			self
		} else {
			rhs
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
			(Value::Char(x), Value::Char(y)) => x == y,
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
			Value::Char(x) => x.hash(state),
			Value::Array(x) => x.hash(state),
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
			Value::Map(x) => for entry in x.iter() {
				entry.hash(state)
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
	pub fn is_poisoned(&self) -> bool {
		matches!(self, Value::Error(_))
	}
	pub fn is_truthy(&self) -> bool {
		match self {
			Value::Integer(x) => *x > 0,
			Value::Decimal(x, y) => dec_to_float(*x, *y) > 0.0,
			Value::Map(x) => x.len() > 0,
			Value::Array(x) => x.len() > 0,
			Value::Boolean(x) => *x,
			_ => false
		}
	}
	pub fn pow(&self, right: &Self, operator: &Token) -> Self {
		match (self, right) {
			(Value::Integer(x), Value::Integer(y)) => {
				if let Some(z) = x.checked_pow(*y as u32) {
					Value::Integer(z)
				} else {
					Value::error(operator, format!("Overflow raising {} to the power of {}", x, y), String::new())
				}
			},
			(Value::Decimal(a, b), Value::Decimal(x, y)) => float_to_dec(dec_to_float(*a, *b).powf(dec_to_float(*x, *y))),
			(x, y) => Value::error(
				operator, 
				format!("Unable to raise {} ({}) to the power of {} ({})", x, TypeKind::infer(x), y, TypeKind::infer(y)), 
				String::new()
			)
		}
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			Self::Integer(v) => write!(f, "{}", v),
			Self::Char(x) => write!(f, "{}", x),
			Self::Decimal(v, w) => write!(f, "{}.{}", v, w),
			Self::Error(e) => write!(f, "{}", e), // TODO change this
			Self::String(s) => write!(f, "{}", s),
			Self::Array(array) => {
				let entries: Vec<String> = array
					.iter()
					.map(|item| format!("{}", *item))
					.collect();
				_ = write!(f, "[{}]", entries.join(", "));
				Ok(())
			},
			Self::Map(map) => {
				let entries: Vec<String> = map
					.iter()
					.map(|(key, (_, value))| format!("{key}: {value}"))
					.collect();
				_ = write!(f, "[{}]", entries.join(", "));
				Ok(())
			},
			Self::FunctionBody(params, _) => {
				let params: Vec<String> = params
					.iter()
					.map(|p| format!("{} {}", p.0, p.1.identifier))
					.collect();
				_ = write!(f, "fun ({})", params.join(","));
				Ok(())
			},
			Self::Type(type_kind) => write!(f, "{}", type_kind),
			v => write!(f, "{:?}", v)
		}
	}
}