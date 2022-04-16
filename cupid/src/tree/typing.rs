use std::fmt::{Display, Formatter, Result};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::{TypeSymbol, Token, Value, Symbol, Tree, LexicalScope};

#[derive(Debug, Clone)]
pub struct Type {
	pub symbol: TypeSymbol,
	pub fields: Vec<(Type, Symbol)>
}

impl PartialEq for Type {
	fn eq(&self, other: &Self) -> bool {
		self.symbol.name == other.symbol.name
	}
}

impl Eq for Type {}

impl Hash for Type {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.symbol.hash(state);
		self.fields.iter().for_each(|(_, s)| s.hash(state));
	}
}

pub fn use_builtin_types(scope: &mut LexicalScope) {
	_ = scope.define_type(&BOOLEAN.symbol, BOOLEAN);
	_ = scope.define_type(&INTEGER.symbol, INTEGER);
	_ = scope.define_type(&DECIMAL.symbol, DECIMAL);
	_ = scope.define_type(&STRING.symbol, STRING);
	_ = scope.define_type(&FUNCTION.symbol, FUNCTION);
	_ = scope.define_type(&LIST.symbol, LIST);
	_ = scope.define_type(&DICTIONARY.symbol, DICTIONARY);
	_ = scope.define_type(&TUPLE.symbol, TUPLE);
	_ = scope.define_type(&NONE.symbol, NONE);
}

/* Built-in types */
pub const BOOLEAN: Type = Type::new_const("bool");
pub const INTEGER: Type = Type::new_const("int");
pub const DECIMAL: Type = Type::new_const("dec");
pub const STRING: Type = Type::new_const("string");
pub const FUNCTION: Type = Type::new_const("fun");
pub const LIST: Type = Type::new_const("list");
pub const DICTIONARY: Type = Type::new_const("dict");
pub const TUPLE: Type = Type::new_const("tuple");
pub const NONE: Type = Type::new_const("none");
pub const ERROR: Type = Type::new_const("error");
pub const MAP_ENTRY: Type = Type::new_const("map_entry");

impl Type {
	pub const fn new_const(name: &'static str) -> Self {
		Self {
			symbol: TypeSymbol::new_const(name),
			fields: vec![]
		}
	}
	pub fn from(value: &Value) -> Type {
		match value {
			Value::Boolean(_) => BOOLEAN,
			Value::Integer(_) => INTEGER,
			Value::Decimal(_, _) => DECIMAL,
			Value::String(_) => STRING,
			Value::FunctionBody(_, _) => FUNCTION,
			Value::Dictionary(_) => DICTIONARY,
			Value::List(_) => LIST,
			Value::Tuple(_) => TUPLE,
			Value::MapEntry(_, _) => MAP_ENTRY,
			Value::Error(_) => ERROR,
			_ => NONE
		}
	}
	pub fn is_builtin(&self) -> bool {
		vec![&BOOLEAN, &INTEGER, &DECIMAL, &STRING, &FUNCTION, &DICTIONARY, &LIST, &TUPLE, &NONE, &ERROR].contains(&self)
	}
	pub fn get_name(&self) -> String {
		self.symbol.name.to_string()
	}
	pub fn is(&self, name: &str) -> bool {
		self.symbol.name == name
	}
	// is either a builtin map type or a struct type
	pub fn is_map(&self) -> bool {
		if [DICTIONARY, LIST, TUPLE].contains(self) {
			true
		} else {
			!self.is_builtin()
		}
	}
}

impl Display for Type {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.symbol)
	}
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DefineType {
	pub token: Token,
	pub type_value: Type,
}

impl Tree for DefineType {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
    	match scope.define_type(&self.type_value.symbol, self.type_value.clone()) {
			Ok(new_type) => Value::Type(new_type),
			Err(error) => error
		}
	}
}

type DictRef<'a> = &'a HashMap<Value, (usize, Value)>;

pub fn is_type(value: &Value, custom_type: &Type) -> bool {
	let value_type = Type::from(value);
	if &value_type == custom_type {
		true
	} else {
		let should_be_builtin = custom_type.is_builtin();
		if should_be_builtin {
			false
		} else if let Value::Dictionary(dict) = value {
			is_dict_typeof(dict, custom_type)
		} else {
			false
		}
	}
}

pub fn is_dict_typeof(dict: DictRef, custom_type: &Type) -> bool {
	let fields = &custom_type.fields;
	if dict.len() != fields.len() {
		return false;
	}
	for (field_type, field_symbol) in fields {
		if !dict_has_field(dict, field_type, field_symbol) {
			return false;
		}
	}
	true
}

pub fn dict_has_field(dict: DictRef, field_type: &Type, field_symbol: &Symbol) -> bool {
	let field_identifier = &field_symbol.identifier;
	// check that property exists
	if let Some((_, dict_property)) = dict.get(field_identifier) {
		// check that property is correct type
		is_type(dict_property, field_type)
	} else {
		false
	}
}