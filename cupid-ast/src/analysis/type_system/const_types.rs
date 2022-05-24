use crate::*;

lazy_static! {
	pub static ref BOOLEAN: Type = Type::primitive("bool");
	pub static ref INTEGER: Type = Type::primitive("int");
	pub static ref DECIMAL: Type = Type::primitive("dec");
	pub static ref FLOAT: Type = Type::primitive("float");
	pub static ref CHARACTER: Type = Type::primitive("char");
	pub static ref STRING: Type = Type::primitive("string");
	pub static ref NOTHING: Type = Type::primitive("nothing");
	
	pub static ref ARRAY: Type = Type {
		name: Ident::new("array", vec![GenericParam::new("e")]),
		fields: FieldSet::unnamed(vec![Type::primitive("e").into_ident()]),
		traits: vec![EQUAL.into_ident(), NOT_EQUAL.into_ident()],
		methods: vec![],
	};
	
	pub static ref TUPLE: Type = Type {
		name: Ident::new("array", vec![]),
		fields: FieldSet::Empty,
		traits: vec![EQUAL.into_ident(), NOT_EQUAL.into_ident()],
		methods: vec![],
	};
	
	pub static ref MAYBE: Type = Type {
		name: Ident::new("maybe", vec![GenericParam::new("y"), GenericParam::new("n")]),
		fields: FieldSet::sum_unnamed(vec![
			Type::primitive("y").into_ident(), 
			Type::primitive("n").into_ident()
		]),
		traits: vec![EQUAL.into_ident(), NOT_EQUAL.into_ident()],
		methods: vec![],
	};
	
	pub static ref FUNCTION: Type = Type {
		name: Ident::new("fun", vec![GenericParam::new("r")]),
		fields: FieldSet::unnamed(vec![Type::primitive("r").into_ident()]),
		traits: vec![],
		methods: vec![],
	};
}

pub fn array_type(arg: Ident) -> Type {
	let mut array = (*ARRAY).to_owned();
	array.fields = FieldSet::Unnamed(vec![arg.to_owned()]);
	if let Some(g) = array.name.attributes.generics.get_mut(0) {
		g.1 = Some(arg)
	}
	array
}

pub fn tuple_type(args: Vec<Ident>) -> Type {
	let mut tuple = (*TUPLE).to_owned();
	tuple.fields = FieldSet::Unnamed(args);
	tuple
}