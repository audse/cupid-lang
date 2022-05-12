use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub, Mul, Neg, Div, Rem, BitAnd, BitOr};
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
	Array(Vec<ValueNode>),
	Boolean(bool),
	Char(char),
	Decimal(i32, u32),
	Function(FunctionNode),
	Implementation(Implementation),
	Integer(i32),
	Log(Box<Value>),
	Map(HashMap<ValueNode, (usize, ValueNode)>),
	None,
	String(Cow<'static, str>),
	Type(TypeKind),
	Values(Vec<ValueNode>),
	TypeHint(TypeHintNode),
}

impl Add for Value {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
    	match (self, rhs) {
			(Value::Integer(x), Value::Integer(y)) => Value::Integer(x + y),
			(Value::Decimal(x, y), Value::Decimal(a, b)) => float_to_dec(dec_to_float(x, y) + dec_to_float(a, b)),
			(Value::String(x), Value::String(y)) => {
				Value::String(x.to_owned() + y)
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
			(Value::Type(x), Value::Type(y)) => x == y,
			(Value::None, Value::None) => true,
			(Value::TypeHint(x), Value::TypeHint(y)) => x == y,
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
			Value::Decimal(x, y) => {
				x.hash(state);
				y.hash(state);
			},
			Value::None => (),
			Value::Map(x) => for entry in x.iter() {
				entry.hash(state)
			},
			Value::Type(x) => x.hash(state),
			Value::Log(x) => x.hash(state),
			Value::Implementation(x) => x.hash(state),
			Value::Values(v) => v.iter().for_each(|v| v.hash(state)),
			Value::TypeHint(x) => x.hash(state),
			_ => ()
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
	pub fn is_truthy(&self) -> bool {
		match self {
			Value::Integer(x) => *x >= 0,
			Value::Decimal(x, y) => dec_to_float(*x, *y) >= 0.0,
			Value::Map(x) => !x.is_empty(),
			Value::Array(x) => !x.is_empty(),
			Value::Boolean(x) => *x,
			Value::Char(x) => *x != '\0',
			_ => true
		}
	}
	pub fn pow(&self, right: &Self, _operator: &Token) -> Result<Self, String> {
		match (self, right) {
			(Value::Integer(x), Value::Integer(y)) => {
				if let Some(z) = x.checked_pow(*y as u32) {
					Ok(Value::Integer(z))
				} else {
					Err(format!("Overflow raising {x} to the power of {y}"))
				}
			},
			(Value::Decimal(a, b), Value::Decimal(x, y)) => Ok(float_to_dec(dec_to_float(*a, *b).powf(dec_to_float(*x, *y)))),
			(x, y) => Err(format!("Unable to raise {x} to the power of {y}"))
		}
	}
	
	pub fn cast(&self, type_kind: Value) -> Value {
		match type_kind {
			Value::Type(TypeKind::Primitive(t)) => match t.identifier.into_owned().as_str() {
				"int" => self.as_int(),
				"dec" => self.as_dec(),
				"bool" => self.as_bool(),
				"string" => self.as_string(),
				_ => panic!("unable to cast")
			},
			_ => panic!("unable to cast")
		}
	}
	pub fn as_int(&self) -> Value {
		match self {
			Value::Integer(y) => Value::Integer(*y),
			Value::Decimal(a, b) => Value::Integer(dec_to_float(*a, *b) as i32),
			Value::Boolean(y) => match y {
				true => Value::Integer(1),
				false => Value::Integer(-1),
			},
			Value::String(x) => Value::Integer(x.parse::<i32>().unwrap_or_else(|_| panic!())),
			_ => panic!()
		}
	}
	pub fn as_bool(&self) -> Value {
		Value::Boolean(self.is_truthy())
	}
	pub fn as_dec(&self) -> Value {
		match self {
			Value::Decimal(a, b) => Value::Decimal(*a, *b),
			Value::Integer(x) => float_to_dec(*x as f64),
			Value::Boolean(y) => match y {
				true => Value::Decimal(1, 0),
				false => Value::Decimal(-1, 0),
			},
			Value::String(x) => float_to_dec(x.parse::<f64>().unwrap_or_else(|_| panic!())),
			_ => panic!()
		}
	}
	pub fn as_string(&self) -> Value {
		Value::String(self.to_string().into())
	}
	pub fn get_property(&self, property: &ValueNode) -> Result<ValueNode, String> {
		match self {
			Value::Map(map) => if let Some((_, value)) = map.get(property) {
				Ok(value.to_owned())
			} else {
				Err(format!("map has no property {property}"))
			},
			Value::Array(array) => match &property.value {
				Value::Integer(i) => Ok(array[*i as usize].to_owned()),
				x => Err(format!("arrays can only be accessed with integers (not {x})"))
			},
			x => Err(format!("cannot access properties of {x}"))
		}
	}
	pub fn map_to_vec(self) -> Vec<(ValueNode, ValueNode)> {
		if let Value::Map(map) = self {
			let mut map_vec: Vec<(ValueNode, (usize, ValueNode))> = map.into_iter().collect();
			map_vec.sort_by(|(_, (a, _)), (_, (z, _))| a.cmp(z));
			map_vec.iter().map(|(k, (_, v))| (
					(*k).to_owned(), 
					v.to_owned()
				)).collect()
		} else {
			panic!("expected map");
		}
	}
	pub fn array_to_vec(self) -> Vec<(ValueNode, ValueNode)> {
		if let Value::Array(a) = self {
			a.into_iter()
				.enumerate()
				.map(|(i, element)| (
					ValueNode::from((Value::Integer(i as i32), &element.meta)),
					element
				))
				.collect()
		} else {
			panic!("expected array")
		}
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		match self {
			Self::Boolean(b) => write!(f, "{b}"),
			Self::Integer(v) => write!(f, "{v}"),
			Self::Char(x) => write!(f, "{x}"),
			Self::Decimal(v, w) => write!(f, "{v}.{w}"),
			Self::String(s) => write!(f, "{s}"),
			Self::Array(array) => {
				let entries: Vec<String> = array
					.iter()
					.map(|item| format!("{item}"))
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
			Self::Type(type_kind) => write!(f, "{type_kind}"),
			Self::Log(log) => write!(f, "{log}"),
			Self::Implementation(trait_map) => write!(f, "trait {trait_map}"),
			Self::Values(values) => {
				let values: Vec<String> = values
					.iter()
					.map(|v| v.to_string())
					.collect();
				write!(f, "{}", values.join(", "))
			},
			Self::Function(function) => {
				let params: Vec<String> = function.params.symbols
					.iter()
					.map(|p|p.symbol.0.to_string())
					.collect();
				write!(f, "{}", params.join(", "))
			},
			Self::TypeHint(id) => write!(f, "{id}"),
			v => write!(f, "{:?}", v)
		}
	}
}