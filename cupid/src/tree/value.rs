use std::fmt::{Display, Formatter, Result};
use crate::{Symbol, Expression, Error, Token};


#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Type {
	Integer,
	Decimal,
	Boolean,
	String,
	Function,
	Error,
	None,
}

impl Type {
	pub fn from_value(val: &Value) -> Self {
		match val {
			Value::Integer(_) => Self::Integer,
			Value::Decimal(_, _) => Self::Decimal,
			Value::String(_) => Self::String,
			Value::Boolean(_) => Self::Boolean,
			Value::FunctionBody(_, _) => Self::Function,
			Value::Error(_) => Self::Error,
			_ => Self::None
		}
	}
}

impl Display for Type {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    	match self {
			Type::Integer => write!(f, "integer"),
			Type::Decimal => write!(f, "decimal"),
			Type::Boolean => write!(f, "boolean"),
			Type::String => write!(f, "string"),
			Type::Function => write!(f, "function"),
			Type::None => write!(f, "none"),
			_ => write!(f, "unknown type"),
		}
	}
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Value {
	Integer(i32),
	Decimal(i32, u32),
	String(String),
	Boolean(bool),
	FunctionBody(Vec<Symbol>, Box<Expression>),
	Error(Error),
	None,
}

macro_rules! op {
	($left:tt $op:tt $right:tt) => { $left $op $right };
	($left:tt $op:tt $right:tt ?) => { $left $op $right };

	($left_front:tt $left_back:tt $op:tt $right_front:tt $right_back:tt) => {
		dec_to_float(*$left_front, *$left_back) $op dec_to_float($right_front, $right_back)
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
	pub fn error<S>(token: &Token, message: S) -> Value where S: Into<String> {
		Value::Error(Error::from_token(token, &message.into()))
	}
	pub fn is_poisoned(&self) -> bool {
		match self {
			Value::Error(_) => true,
			_ => false
		}
	}
	pub fn add(&self, other: Self, operator: &Token) -> Value {
		match (self, other) {
			(Self::Integer(x), Self::Integer(y)) => Self::Integer(op!(x + y)),
			(Self::Decimal(a, b), Self::Decimal(x, y)) => float_to_dec(op!(a b + x y)),
			(Self::String(x), Self::String(y)) => {
				let x = x.to_owned();
				let y = y.as_str();
				Self::String(op!(x + y))
			},
			(x, y) => Value::error(operator, format!("Cannot add {:?} to {:?}", y, x))
		}
	}
	pub fn subtract(&self, other: Self, operator: &Token) -> Value {
		match (self, other) {
			(Self::Integer(x), Self::Integer(y)) => Self::Integer(op!(x - y)),
			(Self::Decimal(a, b), Self::Decimal(x, y)) => float_to_dec(op!(a b - x y)),
			(x, y) => Value::error(operator, format!("Cannot subtract {:?} from {:?}", y, x))
		}
	}
	pub fn multiply(&self, other: Self, operator: &Token) -> Value {
		match (self, other) {
			(Self::Integer(x), Self::Integer(y)) => Self::Integer(op!(x * y)),
			(Self::Decimal(a, b), Self::Decimal(x, y)) => float_to_dec(op!(a b * x y)),
			(x, y) => Value::error(operator, format!("Cannot multiply {:?} and {:?}", x, y))
		}
	}
	pub fn divide(&self, other: Self, operator: &Token) -> Value {
		match (self, other) {
			(Self::Integer(x), Self::Integer(y)) => Self::Integer(op!(x / y)),
			(Self::Decimal(a, b), Self::Decimal(x, y)) => float_to_dec(op!(a b / x y)),
			(x, y) => Value::error(operator, format!("Cannot divide {:?} by {:?}", x, y))
		}
	}
	pub fn negative(&self, operator: &Token) -> Value {
		let n = -1;
		match self {
			Self::Integer(x) => Self::Integer(op!(x * n)),
			Self::Decimal(a, b) => float_to_dec(op!(a b * n 0)),
			x => Value::error(operator, format!("Cannot make {:?} negative", x))
		}
	}
	pub fn is_equal(&self, other: &Self) -> bool {
		op!(self == other)
	}
	pub fn equal(&self, other: &Self) -> Self {
		Self::Boolean(op!(self == other))
	}
	pub fn not_equal(&self, other: &Self) -> Self {
		Self::Boolean(op!(self != other))
	}
	pub fn greater(&self, other: Self, operator: &Token) -> Self {
		match (self, other) {
			(Self::Integer(x), Self::Integer(y)) => {
				let y = &y;
				Self::Boolean(op!(x > y))
			},
			(Self::Decimal(a, b), Self::Decimal(x, y)) => Self::Boolean(op!(a b > x y)),
			(x, y) => Value::error(operator, format!("Cannot compare {:?} and {:?}", x, y))
		}
	}
	pub fn greater_equal(&self, other: Self, operator: &Token) -> Self {
		match (self, other) {
			(Self::Integer(x), Self::Integer(y)) => {
				let y = &y;
				Self::Boolean(op!(x >= y))
			},
			(Self::Decimal(a, b), Self::Decimal(x, y)) => Self::Boolean(op!(a b >= x y)),
			(x, y) => Value::error(operator, format!("Cannot compare {:?} and {:?}", x, y))
		}
	}
	pub fn less(&self, other: Self, operator: &Token) -> Self {
		match (self, other) {
			(Self::Integer(x), Self::Integer(y)) => {
				let y = &y;
				Self::Boolean(op!(x < y))
			},
			(Self::Decimal(a, b), Self::Decimal(x, y)) => Self::Boolean(op!(a b < x y)),
			(x, y) => Value::error(operator, format!("Cannot compare {:?} and {:?}", x, y))
		}
	}
	pub fn less_equal(&self, other: Self, operator: &Token) -> Self {
		match (self, other) {
			(Self::Integer(x), Self::Integer(y)) => {
				let y = &y;
				Self::Boolean(op!(x <= y))
			},
			(Self::Decimal(a, b), Self::Decimal(x, y)) => Self::Boolean(op!(a b <= x y)),
			(x, y) => Value::error(operator, format!("Cannot compare {:?} and {:?}", x, y))
		}
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			Self::Integer(v) => write!(f, "{}", v),
			Self::Decimal(v, w) => write!(f, "{}.{}", v, w),
			Self::Error(e) => write!(f, "{}", e),
			Self::String(s) => write!(f, "{}", s),
			v => write!(f, "{:?}", v)
		}
	}
}