use crate::*;
use OperationFlag::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FunctionFlag {
	Operation(OperationFlag),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OperationFlag {
	Add,
	Subtract,
	Multiply,
	Divide,
	Modulus,
	Power,
	Equal,
	NotEqual,
	Less,
	LessEqual,
	Greater,
	GreaterEqual,
	And,
	Or,
	As,
	IsType,

	Get,
	// Set,
}

impl From<OperationFlag> for FunctionFlag {
	fn from(flag: OperationFlag) -> Self {
		Self::Operation(flag)
	}
}

impl From<FunctionFlag> for OperationFlag {
	fn from(flag: FunctionFlag) -> Self {
		if let FunctionFlag::Operation(flag) = flag {
			flag
		} else {
			panic!("expected operation")
		}
	}
}

pub const OPERATIONS: &[(&str, &str, OperationFlag); 17] = &[
	("+", "add", Add),
	("-", "subtract", Subtract),
	("*", "multiply", Multiply),
	("/", "divide", Divide),
	("%", "modulus", Modulus),
	("^", "power", Power),
	("is", "equal", Equal),
	("not", "not_equal", NotEqual),
	("<", "less", Less),
	("<=", "less_equal", LessEqual),
	(">", "greater", Greater),
	(">=", "greater_equal", GreaterEqual),
	("and", "logic_and", And),
	("or", "logic_or", Or),
	("as", "cast", As),
	("istype", "is_type", IsType),
	(".", "get", Get),
];

pub fn get_operation(op_symbol: &str) -> (&'static str, OperationFlag) {
	match OPERATIONS.iter().find(|(op, ..)| *op == op_symbol) {
		Some((_, function_name, flag)) => (*function_name, *flag),
		None => panic!("unrecognized operation")
	}
}

pub fn do_operation(op: OperationFlag, left: Value, right: Value) -> Result<Value, String> {
	use Value::*;
	match op {
		Add => left + right,
		Subtract => left - right,
		Multiply => left * right,
		Divide => left / right,
		Modulus => left % right,
		Power => left.pow(&right),
		Equal => Ok(Boolean(left == right)),
		NotEqual => Ok(Boolean(left != right)),
		Less => left.compare(right, "<"),
		LessEqual => left.compare(right, "<="),
		Greater => left.compare(right, ">"),
		GreaterEqual => left.compare(right, ">="),
		And => left & right,
		Or => left | right,
		As => left.cast(right),
		_ => Err(format!("unrecognized operation: {:?}", op)),
	}
}