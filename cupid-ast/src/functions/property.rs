use crate::*;

pub enum Object {
	Array(Vec<Value>),
	Instance(Value),
	Type(Type),
}

pub enum Property {
	Method {
		name: Str, 
		args: Vec<Value>
	},
	Field(Str),
	Index(usize),
}

pub struct PropertyAccess {
	pub object: BoxAST,
	pub property: Property,
}