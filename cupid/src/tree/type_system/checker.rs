use std::collections::HashMap;
use crate::{Value, Type, Symbol, TypeSymbol, SymbolFinder};

type DictRef<'a> = &'a HashMap<Value, (usize, Value)>;

pub trait TypeChecker: SymbolFinder {
	fn is_type(&self, value: &Value, custom_type_symbol: &TypeSymbol) -> bool {
		let value_type = Type::from(value);
		if let Some(custom_type) = self.get_definition(custom_type_symbol) {
			if value_type == custom_type {
				true
			} else {
				let should_be_builtin = custom_type.is_builtin();
				if should_be_builtin {
					false
				} else if let Value::Dictionary(dict) = value {
					self.is_dict_typeof(dict, custom_type_symbol)
				} else {
					false
				}
			}
		} else {
			false
		}
	}
	
	fn is_dict_typeof(&self, dict: DictRef, custom_type_symbol: &TypeSymbol) -> bool {
		if let Some(custom_type) = self.get_definition(custom_type_symbol) {
			let fields = &custom_type.fields;
			if dict.len() != fields.len() {
				return false;
			}
			for (field_type, field_symbol) in fields {
				if !self.dict_has_field(dict, field_type, field_symbol) {
					return false;
				}
			}
			true
		} else {
			false
		}
	}
	
	fn dict_has_field(&self, dict: DictRef, field_type_symbol: &TypeSymbol, field_symbol: &Symbol) -> bool {
		let field_identifier = &field_symbol.identifier;
		// check that property exists
		if let Some((_, dict_property)) = dict.get(field_identifier) {
			// check that property is correct type
			self.is_type(dict_property, field_type_symbol)
		} else {
			false
		}
	}
}
// 
// pub fn is_type<T>(value: &Value, custom_type_symbol: &TypeSymbol, scope: &T) -> bool where T: SymbolFinder {
// 	let value_type = Type::from(value);
// 	if let Some(custom_type) = scope.get_definition(custom_type_symbol) {
// 		if value_type == custom_type {
// 			true
// 		} else {
// 			let should_be_builtin = custom_type.is_builtin();
// 			if should_be_builtin {
// 				false
// 			} else if let Value::Dictionary(dict) = value {
// 				is_dict_typeof(dict, custom_type_symbol, scope)
// 			} else {
// 				false
// 			}
// 		}
// 	} else {
// 		false
// 	}
// }
// 
// pub fn is_dict_typeof<T>(dict: DictRef, custom_type_symbol: &TypeSymbol, scope: &T) -> bool where T: SymbolFinder {
// 	if let Some(custom_type) = scope.get_definition(custom_type_symbol) {
// 		let fields = &custom_type.fields;
// 		if dict.len() != fields.len() {
// 			return false;
// 		}
// 		for (field_type, field_symbol) in fields {
// 			if !dict_has_field(dict, field_type, field_symbol, scope) {
// 				return false;
// 			}
// 		}
// 		true
// 	} else {
// 		false
// 	}
// }
// 
// pub fn dict_has_field<T>(dict: DictRef, field_type_symbol: &TypeSymbol, field_symbol: &Symbol, scope: &T) -> bool where T: SymbolFinder {
// 	let field_identifier = &field_symbol.identifier;
// 	// check that property exists
// 	if let Some((_, dict_property)) = dict.get(field_identifier) {
// 		// check that property is correct type
// 		is_type(dict_property, field_type_symbol, scope)
// 	} else {
// 		false
// 	}
// }