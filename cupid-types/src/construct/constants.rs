use crate::{
	Type,
	FieldSet,
	lazy_static,
	GenericParam,
	Cow,
};

lazy_static! {
	pub static ref BOOLEAN: Type = Type::primitive("bool");
	pub static ref INTEGER: Type = Type::primitive("int");
	pub static ref DECIMAL: Type = Type::primitive("dec");
	pub static ref FLOAT: Type = Type::primitive("float");
	pub static ref CHARACTER: Type = Type::primitive("char");
	pub static ref STRING: Type = Type::primitive("string");
	pub static ref NOTHING: Type = Type::primitive("nothing");
	
	pub static ref ARRAY: Type = Type {
		name: Some(Cow::Borrowed("array")),
		params: vec![GenericParam::new("e")],
		fields: FieldSet::unnamed(vec![Type::primitive("e")]),
		traits: vec![],
		methods: vec![],
	};
	
	pub static ref TUPLE: Type = Type {
		name: Some(Cow::Borrowed("tuple")),
		params: vec![],
		fields: FieldSet::Empty,
		traits: vec![],
		methods: vec![],
	};
	
	pub static ref MAYBE_GENERICS: Vec<Type> = vec![Type::primitive("y"), Type::primitive("n")];
	pub static ref MAYBE: Type = Type {
		name: Some(Cow::Borrowed("maybe")),
		params: vec![GenericParam::new("y"), GenericParam::new("n")],
		fields: FieldSet::unnamed(vec![
			Type::primitive("y"), 
			Type::primitive("n")
		]),
		traits: vec![],
		methods: vec![],
	};
}

pub fn array_type(arg: Type) -> Type {
	let mut array = (*ARRAY).to_owned();
	array.fields = FieldSet::Unnamed(vec![arg]);
	array.params = vec![];
	array
}

pub fn tuple_type(args: Vec<Type>) -> Type {
	let mut tuple = (*TUPLE).to_owned();
	tuple.fields = FieldSet::Unnamed(args);
	tuple
}