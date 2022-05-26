use crate::*;

lazy_static! {
	pub static ref BOOLEAN: Type = primitive("bool");
	pub static ref INTEGER: Type = primitive("int");
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

pub fn array_type(arg: Ident) -> Type {
	TypeBuilder::from(&*ARRAY)
		.unnamed_fields(vec![arg.to_owned()])
		.generic_arg(0, arg)
		.build()
}

pub fn tuple_type(args: Vec<Ident>) -> Type {
	TypeBuilder::from(&*TUPLE)
		.unnamed_fields(args)
		.build()
}

#[macro_export]
macro_rules! generics {
	($($g:tt),*) => { GenericParams::from(vec![$($g),*]) }
}

#[macro_export]
macro_rules! fields {
	($($f:tt),*) => { 
		FieldSet::Unnamed(vec![ $( primitive($f).into_ident() ),* ])
	};
	($($name:tt: $f:tt),*) => {
		FieldSet::Named(vec![ $( 
			(Cow::Borrowed($f), primitive($f).into_ident()) 
		),* ])
	};
}

#[macro_export]
macro_rules! traits {
	($($t:ident),*) => { vec![$($t.into_ident()),*] }
}