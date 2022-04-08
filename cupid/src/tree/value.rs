use std::any::Any;
use std::fmt::{
	Display,
	Formatter,
	Result,
};
use crate::{
	Operator,
	CupidExpression,
	CupidSymbol,
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum CupidValue {
	Integer(i32),
	Decimal{ front: i32, back: u32 }, // converted to int
	String(String),
	Boolean(bool),
	FunctionBody(FunctionBody),
	None,
}

#[derive(Debug, Hash, Clone)]
pub struct FunctionBody(pub Vec<CupidSymbol>, pub Box<CupidExpression>);
impl PartialEq for FunctionBody {
	fn eq(&self, other: &Self) -> bool {
		return false;
	}
}
impl Eq for FunctionBody {}

pub fn dec_to_float(front: i32, back: u32) -> f64 {
	format!("{}.{}", front, back).parse::<f64>().unwrap()
}

pub fn float_to_dec(float: f64) -> CupidValue {
	let string = float.to_string();
	let mut parts: Vec<&str> = string.split(".").collect();
	// add a trailing 0
	if parts.len() == 1 {
		parts.push("0");
	}
	return CupidValue::Decimal { 
		front: parts[0].parse::<i32>().unwrap(), 
		back: parts[1].parse::<u32>().unwrap()
	};
}

impl CupidValue {
	
	pub fn is_equal(self, expected: Box<dyn Any>) -> bool {
		match self {
			CupidValue::Integer(i) => {
				if let Ok(expected_val) = expected.downcast::<i32>() {
					return i == *expected_val;
				}
				return false;
			},
			CupidValue::Decimal { front, back } => {
				if let Ok(expected_val) = expected.downcast::<f64>() {
					let float = dec_to_float(front, back);
					return (float - *expected_val as f64) < 0.00001
				}
				return false;
			},
			CupidValue::String(s) => {
				if let Ok(expected_val) = expected.downcast::<String>() {
					return s == *expected_val;
				}
				return false;
			},
			CupidValue::Boolean(b) => {
				if let Ok(expected_val) = expected.downcast::<bool>() {
					return b == *expected_val;
				}
				return false;
			},
			CupidValue::None => {
				if let Ok(expected_val) = expected.downcast::<Option<bool>>() {
					return expected_val.is_none();
				}
				return false;
			},
			_ => return false, // TODO compare functions
		}
	}
	
	pub fn op(op: Operator, left: &Self, right: &Self) -> Self {
		match (&left, &right) {
			// integer
			(Self::Integer(x), Self::Integer(y)) => Self::op_int(op, *x, *y),
			
			// negative integer
			(Self::None, Self::Integer(y)) => match op {
				Operator::Subtract => Self::op_int(Operator::Multiply, *y, -1),
				_ => panic!("Cannot perform operation {} on {} and {}", op, left, right)
			},
			
			// decimal
			(Self::Decimal { front: xf, back: xb }, Self::Decimal { front: yf, back: yb }) => Self::op_dec(op, *xf, *xb, *yf, *yb),
			
			// negative decimal
			(Self::None, Self::Decimal { front: yf, back: yb }) => match op {
				Operator::Subtract => Self::op_dec(Operator::Multiply, *yf, *yb, -1, 1),
				_ => panic!("Cannot perform operation {} on {} and {}", op, left, right)
			},
			
			(Self::String(x), Self::String(y)) => Self::op_str(op, x, y),
			(Self::Boolean(x), Self::Boolean(y)) => Self::op_bool(op, *x, *y),
			_ => panic!("Cannot perform operation {} on {} and {}", op, left, right)
		}
	}
	
	pub fn op_int(op: Operator, x: i32, y: i32) -> CupidValue {
		match op {
			Operator::Add => CupidValue::Integer(x + y),
			Operator::Subtract => CupidValue::Integer(x - y),
			Operator::Multiply => CupidValue::Integer(x * y),
			Operator::Divide => CupidValue::Integer(x / y),
			Operator::Equal => CupidValue::Boolean(x == y),
			Operator::NotEqual => CupidValue::Boolean(x != y),
			Operator::Greater => CupidValue::Boolean(x > y),
			Operator::GreaterEqual => CupidValue::Boolean(x >= y),
			Operator::Less => CupidValue::Boolean(x < y),
			Operator::LessEqual => CupidValue::Boolean(x <= y),
			Operator::And => CupidValue::Integer(if x > 0 && y > 0 { y } else if x > 0 { x } else { 0 }),
			Operator::Or => CupidValue::Integer(if x > 0 { x } else if y > 0 { y } else { 0 }),
			p => panic!("Cannot perform operation ({}) on an integer", p)
		}
	}
	
	pub fn op_dec(op: Operator, xf: i32, xb: u32, yf: i32, yb: u32) -> CupidValue {
		let x = dec_to_float(xf, xb);
		let y = dec_to_float(yf, yb);
		match op {
			Operator::Add => float_to_dec(x + y),
			Operator::Subtract => float_to_dec(x - y),
			Operator::Multiply => float_to_dec(x * y),
			Operator::Divide => float_to_dec(x / y),
			Operator::Equal => CupidValue::Boolean(x == y),
			Operator::NotEqual => CupidValue::Boolean(x != y),
			Operator::Greater => CupidValue::Boolean(x > y),
			Operator::GreaterEqual => CupidValue::Boolean(x >= y),
			Operator::Less => CupidValue::Boolean(x < y),
			Operator::LessEqual => CupidValue::Boolean(x <= y),
			Operator::And => float_to_dec(if x > 0.0 && y > 0.0 { y } else if x > 0.0 { x } else { 0.0 }),
			Operator::Or => float_to_dec(if x > 0.0 { x } else if y > 0.0 { y } else { 0.0 }),
			p => panic!("Cannot perform operation ({}) on a decimal", p)
		}
	}
	
	pub fn op_str(op: Operator, x: &String, y: &String) -> CupidValue {
		match op {
			Operator::Add => {
				let mut string = x.clone();
				string.push_str(&y);
				CupidValue::String(string)
			},
			Operator::Equal => CupidValue::Boolean(x == y),
			Operator::NotEqual => CupidValue::Boolean(x != y),
			z => panic!("Cannot perform operation ({}) on string", z)
		}
	}
	
	pub fn op_bool(op: Operator, x: bool, y: bool) -> CupidValue {
		match op {
			Operator::Equal => CupidValue::Boolean(x == y),
			Operator::NotEqual => CupidValue::Boolean(x != y),
			Operator::And => CupidValue::Boolean(x && y),
			Operator::Or => CupidValue::Boolean(x || y),
			p => panic!("Cannot perform operation ({}) on a boolean", p)
		}
	}
	
}


impl Display for CupidValue {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			CupidValue::Integer(i) => write!(f, "Integer {}", i),
			CupidValue::Decimal { front, back } => write!(f, "Decimal {}.{}", front, back),
			CupidValue::String(s) => write!(f, "String {}", s),
			CupidValue::Boolean(b) => write!(f, "Boolean {}", b),
			CupidValue::None => write!(f, "None"),
			_ => write!(f, ""),
		}
	}
}

impl CupidValue {
	pub fn clone(&self) -> Self {
		match self {
			CupidValue::Integer(i) => CupidValue::Integer(*i),
			CupidValue::Decimal { front, back } => CupidValue::Decimal { front: *front, back: *back },
			CupidValue::String(s) => CupidValue::String(s.clone()),
			CupidValue::Boolean(b) => CupidValue::Boolean(*b),
			_ => CupidValue::None
		}
	}
}

impl Clone for CupidValue {
	fn clone(&self) -> Self {
		match self {
			CupidValue::Integer(i) => CupidValue::Integer(*i),
			CupidValue::Decimal { front, back } => CupidValue::Decimal { front: *front, back: *back },
			CupidValue::String(s) => CupidValue::String(s.clone()),
			CupidValue::Boolean(b) => CupidValue::Boolean(*b),
			// CupidValue::FunctionBody(f) => CupidValue::FunctionBody(*f),
			_ => CupidValue::None
		}
	}
}