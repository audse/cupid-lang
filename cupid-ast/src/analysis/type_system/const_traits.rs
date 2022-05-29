use crate::*;

type S = &'static str;

const OP_ADD: S = "add!";
const OP_SUBTRACT: S = "subtract!";
const OP_EQUAL: S = "equal!";
const OP_NOT_EQUAL: S = "not_equal!";
const OP_GET: S = "get!";

// Traits
lazy_static! {
	pub static ref ADD: Trait = new_bin_op(OP_ADD);
	pub static ref SUBTRACT: Trait = new_bin_op(OP_SUBTRACT);
	// pub static ref MULTIPLY: Trait = new_bin_op("*");
	// pub static ref DIVIDE: Trait = new_bin_op("/");
	pub static ref EQUAL: Trait = new_bin_op(OP_EQUAL);
	pub static ref NOT_EQUAL: Trait = new_bin_op(OP_NOT_EQUAL);
	pub static ref GET: Trait = new_bin_op(OP_GET);

	pub static ref SQ: Method = new_unary_op_method("sq");
}

pub fn use_type_as_generic_args<T>(trait_val: &mut T, mut type_ident: Ident)
		where T: UseAttributes + Methods + ToOwned<Owned = T> {
	// Apply using type as the first generic argument
	trait_val.attributes().generics.apply_named(vec![(
		type_ident.name.to_owned(), type_ident.to_owned())
	]);
	// Apply other types as they match generic arguments
	trait_val.attributes().generics.apply(
		type_ident.attributes().generics.to_owned()
	);
	// Apply the same generics to each method
	for method in trait_val.methods_mut().iter_mut() {
		method.name.attributes().generics.apply_named(vec![(
			type_ident.name.to_owned(), type_ident.to_owned())
		]);
		method.name.attributes().generics.apply(
			type_ident.attributes().generics.to_owned()
		);
		use_type_as_generic_args(&mut method.signature, type_ident.to_owned());
	}
}

pub fn new_bin_op(name: &'static str) -> Trait {
	// Creates a trait with a single operation method
	// E.g.
	// trait [t] add! = [
	//   fun [t] add = t self, t other => _
	// ]

	let ident = Ident::build()
		.name(name.into())
		.attributes(
			Attributes::build()
				.generics(generics!["t"])
				.build()
		)
		.build();

	let method_signature = Type::build()
		.name(Ident::new("fun", generics!("r": "t")))
		.fields(fields!["t", "t", "t"])
		.base_type(BaseType::Function)
		.build();

	let function_body = Exp::Value(Value { 
		val: Untyped(Val::BuiltinPlaceholder), 
		..Default::default() 
	});

	let param_type_hint = Untyped(Ident::new_name("t"));
	let param_left = Declaration::build()
		.name(Ident::new_name("left"))
		.type_hint(param_type_hint.clone())
		.build();
	let param_right = Declaration::build()
		.name(Ident::new_name("right"))
		.type_hint(param_type_hint)
		.build();

	let method_body = Function::build()
		.body(IsTyped(
			Block::build()
				.body(vec![function_body])
				.build(),
			method_signature.to_owned()
		))
		.params(vec![param_left, param_right])
		.build();

	let method = Method::build()
		.name(ident.to_owned())
		.signature(method_signature)
		.value(Some(method_body))
		.build();

	TraitBuilder::new()
		.name(ident)
		.methods(vec![method])
		.build()
}

pub fn new_unary_op(name: &'static str) -> Trait {
	// Creates a trait with a single operation method
	// E.g.
	// trait [t] negate! = [
	//   fun [t] negate = t self => _
	// ]
	let method = new_unary_op_method(name);

	TraitBuilder::new()
		.name(method.name.to_owned())
		.methods(vec![method])
		.build()
}


pub fn new_unary_op_method(name: &'static str) -> Method {
	// E.g.
	//   fun [t] negate = t self => _

	let ident = Ident::build()
		.name(name.into())
		.attributes(
			Attributes::build()
				.generics(generics!["t"])
				.build()
		)
		.build();

	let method_signature = Type::build()
		.name(Ident::new("fun", generics!("r": "t")))
		.fields(fields!["t", "t"])
		.base_type(BaseType::Function)
		.build();

	let function_body = Exp::Value(Value { 
		val: Untyped(Val::BuiltinPlaceholder), 
		..Default::default() 
	});
	
	let param_left = Declaration::build()
		.name(Ident::new_name("left"))
		.type_hint(Untyped(Ident::new_name("t")))
		.build();

	let method_body = Function::build()
		.body(IsTyped(
			Block::build()
				.body(vec![function_body])
				.build(),
			method_signature.to_owned()
		))
		.params(vec![param_left])
		.build();
	
	Method::build()
		.name(ident)
		.signature(method_signature)
		.value(Some(method_body))
		.build()
}
