use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Unwrap)]
pub enum Value {
	VArray(Vec<Value>, Attributes),
	VBoolean(bool, Attributes),
	VChar(char, Attributes),
	VDecimal(i32, u32, Attributes),
	VFunction(Box<crate::Function>),
	VInteger(i32, Attributes),
	VNone(Attributes),
	VString(Cow<'static, str>, Attributes),
	VTuple(Vec<Value>, Attributes),
	VType(crate::Type),
	VTrait(crate::Trait),
	VBuiltinPlaceholder(Attributes),
}

pub use Value::*;

impl Default for Value {
	fn default() -> Self { Self::VNone(Attributes::default()) }
}

impl UseAttributes for Value {
	fn attributes(&self) -> &Attributes {
		match self {
			VArray(_, a) => a,
			VBoolean(_, a) => a,
			VChar(_, a) => a,
			VDecimal(_, _, a) => a,
			VFunction(function) => function.attributes(),
			VInteger(_, a) => a,
			VNone(a) => a,
			VString(_, a) => a,
			VTuple(_, a) => a,
			VType(type_val) => type_val.attributes(),
			VTrait(trait_val) => trait_val.attributes(),
			VBuiltinPlaceholder(a) => a
		}
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		match self {
			VArray(_, a) => a,
			VBoolean(_, a) => a,
			VChar(_, a) => a,
			VDecimal(_, _, a) => a,
			VFunction(function) => function.attributes_mut(),
			VInteger(_, a) => a,
			VNone(a) => a,
			VString(_, a) => a,
			VTuple(_, a) => a,
			VType(type_val) => type_val.attributes_mut(),
			VTrait(trait_val) => trait_val.attributes_mut(),
			VBuiltinPlaceholder(a) => a
		}
	}
}