use crate::{ LexicalScope, Type, SymbolFinder, TypeSymbol };

pub fn use_builtin_types(scope: &mut LexicalScope) {
	_ = scope.define_type(&BOOLEAN.get_symbol(), BOOLEAN);
	_ = scope.define_type(&INTEGER.get_symbol(), INTEGER);
	_ = scope.define_type(&DECIMAL.get_symbol(), DECIMAL);
	_ = scope.define_type(&STRING.get_symbol(), STRING);
	_ = scope.define_type(&CHAR.get_symbol(), CHAR);
	_ = scope.define_type(&NONE.get_symbol(), NONE);
	
	// _ = scope.define_type(&ARRAY.get_symbol(), ARRAY);
	_ = scope.define_type(
		&ARRAY.get_symbol(), 
		Type::new_const_product("array", vec![(GENERIC, None)])
	);
	_ = scope.define_type(
		&MAP.get_symbol(), 
		Type::new_const_product("map", vec![(GENERIC, None), (GENERIC, None)])
	);
	
	_ = scope.define_type(&FUNCTION.get_symbol(), FUNCTION);
	_ = scope.define_type(&LIST.get_symbol(), LIST);
	_ = scope.define_type(&DICTIONARY.get_symbol(), DICTIONARY);
	_ = scope.define_type(&TUPLE.get_symbol(), TUPLE);
}

/* Built-in types */
pub const BOOLEAN: Type = Type::new_const_product("bool", vec![]);
pub const INTEGER: Type = Type::new_const_product("int", vec![]);
pub const DECIMAL: Type = Type::new_const_product("dec", vec![]);
pub const STRING: Type = Type::new_const_product("string", vec![]);
pub const CHAR: Type = Type::new_const_product("char", vec![]);
pub const NONE: Type = Type::new_const_product("none", vec![]);
pub const ERROR: Type = Type::new_const_product("error", vec![]);

pub const GENERIC: TypeSymbol = TypeSymbol::new_const_generic("T");

pub const ARRAY: Type = Type::new_const_product("array", vec![]);
pub const MAP: Type = Type::new_const_product("map", vec![]);

pub const FUNCTION: Type = Type::new_const_product("fun", vec![]);
pub const LIST: Type = Type::new_const_product("list", vec![]);
pub const DICTIONARY: Type = Type::new_const_product("dict", vec![]);
pub const TUPLE: Type = Type::new_const_product("tuple", vec![]);
pub const MAP_ENTRY: Type = Type::new_const_product("map_entry", vec![]);