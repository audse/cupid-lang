use serde::{Serialize, Deserialize};
use crate::{Tree, LexicalScope, Value, SymbolFinder, TypeKind, Symbol, PrimitiveType, ArrayType, MapType, FunctionType, GenericType};

#[derive(Debug, Hash, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuiltInType {
	pub symbol: Symbol
}

impl Tree for BuiltInType {
	fn resolve(&self, scope: &mut LexicalScope) -> Value {
		let name = self.symbol.get_identifier();
		let type_value = match name.as_str() {
			"bool"
			| "char"
			| "int"
			| "dec"
			| "nothing"
			| "string" => TypeKind::Primitive(PrimitiveType::new(&name)),
			"array" => {
				let generic = TypeKind::Generic(GenericType::new("e", None));
				TypeKind::Array(ArrayType { element_type: Box::new(generic) })
			},
			"map" => {
				let key_generic = TypeKind::Generic(GenericType::new("k", None));
				let value_generic = TypeKind::Generic(GenericType::new("v", None));
				TypeKind::Map(MapType { 
					key_type: Box::new(key_generic),
					value_type: Box::new(value_generic) 
				})
			},
			"fun" => {
				let generic = TypeKind::Generic(GenericType::new("r", None));
				TypeKind::Function(FunctionType { return_type: Box::new(generic) })
			},
			_ => unreachable!()
		};
		if let Some(new_type) = scope.define_type(&self.symbol, type_value.clone()) {
			new_type
		} else {
			self.symbol.error_unable_to_assign(&Value::Type(type_value))
		}
	}
}
