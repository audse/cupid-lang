use crate::{ LexicalScope, Type, SymbolFinder };

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