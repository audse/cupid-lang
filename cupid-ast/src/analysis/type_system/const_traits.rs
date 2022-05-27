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

    pub static ref SQ: Method = new_no_params("sq");
}

pub fn new_bin_op(name: &'static str) -> Trait {
	// Creates a trait with a single operation method
	// E.g.
	// trait [t] add! = [
	//   fun [t] add = t self, t other => _
	// ]
	let method = builtin(name);
	TraitBuilder::new()
		.name(method.signature.name.to_owned())
		.methods(vec![method])
		.build()
}

fn new_no_params(name: &'static str) -> Method {
    let signature = TypeBuilder::new()
        .name_str(name)
        .fields(fields!["return": "int"])
        .base_type(BaseType::Function)
        .build();
    MethodBuilder::new()
        .value(Some(builtin_method_value(signature.to_owned())))
        .signature(signature)
        .build()
}

fn builtin(name: &'static str) -> Method {
    let signature = TypeBuilder::new()
        .name_str(name)
        .bin_op("t")
        .build();
    MethodBuilder::new()
        .value(Some(builtin_method_value(signature.to_owned())))
        .signature(signature)
        .build()
}

fn builtin_method_value(signature: Type) -> Function {
    let params = signature.fields
        .unwrap_named()
        .into_iter()
        .map(|(name, param)| builtin_param(name, param))
        .collect::<Vec<Declaration>>();
    FunctionBuilder::new()
        .body(Untyped(Block::default()))
        .params(params)
        .build()
}

fn builtin_param(name: Str, type_hint: Ident) -> Declaration {
    let (value, value_type) = (
        Exp::Value(ValueBuilder::new().builtin().build()), 
        NOTHING.to_owned()
    );
    DeclarationBuilder::new()
        .name(Ident { name, ..Default::default() })
        .type_hint(Untyped(type_hint))
        .value(IsTyped(Box::new(value), value_type))
        .build()
}