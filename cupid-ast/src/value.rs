use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
	Array(Vec<Value>),
	Boolean(bool),
	Char(char),
	Decimal(i32, u32),
	Integer(i32),
	None,
	String(Cow<'static, str>),
	Tuple(Vec<Value>),
	Type(Type),
	Function(Function)
}