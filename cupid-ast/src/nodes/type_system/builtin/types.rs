use crate::*;

lazy_static! {
	pub static ref TYPE: Type = primitive("type");
	pub static ref TRAIT: Type = primitive("trait");
	pub static ref BOOLEAN: Type = primitive("bool");
	pub static ref INTEGER: Type = Type::build()
		.name_str("int")
		.methods(vec![SQ.to_owned()])
		.traits(traits!(ADD, SUBTRACT, EQUAL, NOT_EQUAL))
		.base_primitive("int")
		.build();
	
	pub static ref DECIMAL: Type = primitive("dec");
	pub static ref CHARACTER: Type = primitive("char");
	pub static ref STRING: Type = primitive("string");
	pub static ref NOTHING: Type = primitive("nothing");
	
	pub static ref ARRAY: Type = TypeBuilder::new()
		.name_str("array")
		.generics(generics!["e"])
		.fields(fields!["e"])
		.traits(traits!(EQUAL, NOT_EQUAL))
		.base_type(BaseType::Array)
		.build();
	
	pub static ref TUPLE: Type = TypeBuilder::new()
		.name_str("tuple")
		.traits(traits!(EQUAL, NOT_EQUAL))
		.base_type(BaseType::Array)
		.build();
	
	pub static ref MAYBE: Type = TypeBuilder::new()
		.name_str("maybe")
		.generics(generics!["t"])
		.fields(fields!["yes": "t", "n": "nothing"])
		.traits(traits!(EQUAL, NOT_EQUAL))
		.base_type(BaseType::Sum)
		.build();
	
	pub static ref FUNCTION: Type = TypeBuilder::new()
		.name_str("fun")
		.generics(generics!["r"])
		.fields(fields!["r"])
		.base_type(BaseType::Function)
		.build();
}

pub fn primitive(name: &'static str) -> Type {
	TypeBuilder::primitive(name)
}

pub fn array_type(arg: Type) -> Type {
	let mut new_type = ARRAY.to_owned();
	let element_type = IsTyped(arg.to_ident(), arg);
	new_type.unify_with(&[element_type]).unwrap();
	new_type
}

pub fn tuple_type(args: Vec<Typed<Ident>>) -> Type {
	let mut new_type = TUPLE.to_owned();
	new_type.unify_with(&args).unwrap();
	new_type
}
