use crate::{Tree, LexicalScope, Value, SymbolFinder, TypeKind, Symbol, PrimitiveType, ArrayType, MapType, FunctionType, GenericType};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
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

// pub fn use_builtin_types(scope: &mut LexicalScope, symbols: Vec<Expression>) {
// 	let types: Vec<&Symbol> = symbols
// 		.iter()
// 		.filter_map(|t|
// 			if let Expression::Symbol(symbol) = t {
// 				Some(symbol)
// 			} else {
// 				None
// 			}
// 		)
// 		.collect();
// 	
// 	for symbol in types {
// 		let ok = match symbol.get_identifier().as_str() {
// 			"bool" => scope.define_type(symbol, BOOLEAN),
// 			"int" => scope.define_type(symbol, INTEGER),
// 			"dec" => scope.define_type(symbol, DECIMAL),
// 			"char" => scope.define_type(symbol, CHAR),
// 			_ => unreachable!()
// 		};
// 		match ok {
// 			Ok(_) => (),
// 			Err(_) => (),
// 		};
// 	}
// 	scope.pretty_print_definitions()
// }

// Primitives
// pub const BOOLEAN: TypeKind = TypeKind::Primitive(PrimitiveType::new("bool"));
// pub const INTEGER: TypeKind = TypeKind::Primitive(PrimitiveType::new("int"));
// pub const DECIMAL: TypeKind = TypeKind::Primitive(PrimitiveType::new("dec"));
// pub const CHAR: TypeKind = TypeKind::Primitive(PrimitiveType::new("char"));