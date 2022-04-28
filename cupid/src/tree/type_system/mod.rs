mod alias_type;
pub use alias_type::*;

mod array_type;
pub use array_type::*;

mod builtin;
pub use builtin::*;

mod function_type;
pub use function_type::*;

mod generic_type;
pub use generic_type::*;

mod map_type;
pub use map_type::*;

mod primitive_type;
pub use primitive_type::*;

mod struct_type;
pub use struct_type::*;

mod sum_type;
pub use sum_type::*;

mod types;
pub use types::*;

// use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use serde::{Serialize, Deserialize};
use crate::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeKind {
	Array(ArrayType),
	Function(FunctionType),
	Generic(GenericType),
	Map(MapType),
	Primitive(PrimitiveType),
	Struct(StructType),
	Sum(SumType),
	Alias(AliasType),
}

impl TypeKind {
	pub fn new_primitive(identifier: &str) -> Self {
		Self::Primitive(PrimitiveType::new(identifier))
	}
	pub fn new_array(element_type: Self) -> Self {
		Self::Array(ArrayType { element_type: Box::new(element_type) })
	}
	pub fn new_map(key_type: Self, value_type: Self) -> Self {
		Self::Map(MapType { key_type: Box::new(key_type), value_type: Box::new(value_type) })
	}
	pub fn new_generic(identifier: &str) -> Self {
		Self::Generic(GenericType::new(identifier, None))
	}
	pub fn new_function() -> Self {
		Self::Function(FunctionType { return_type: Box::new(Self::new_generic("r")) })
	}
	pub fn infer(value: &Value) -> Self {
		match value {
			Value::Boolean(_) => Self::new_primitive("bool"),
			Value::Integer(_) =>  Self::new_primitive("int"),
			Value::Char(_) => Self::new_primitive("char"),
			Value::Decimal(_, _) => Self::new_primitive("dec"),
			Value::String(_) => Self::new_primitive("string"),
			Value::None => Self::new_primitive("nothing"),
			Value::FunctionBody(_, _) => Self::new_function(),
			Value::Array(array) => {
				if array.len() > 0 {
					let element_type = TypeKind::infer(&array[0]);
					Self::new_array(element_type)
				} else {
					Self::new_array(Self::new_generic("e"))
				}
			},
			Value::Map(map) => {
				if let Some((key, (_, value))) = map.iter().nth(0) {
					let key_type = TypeKind::infer(key);
					let value_type = TypeKind::infer(value);
					Self::new_map(key_type, value_type)
				} else {
					Self::new_map(Self::new_generic("k"), Self::new_generic("v"))
				}
			},
			Value::Type(t) => t.clone(),
			x => {
				println!("Cannot infer type of {:?}", x);
				unreachable!()
			}
		}
	}
	pub fn infer_name(value: &Value) -> String {
		match value {
			Value::Boolean(_) => "bool",
			Value::Integer(_) =>  "int",
			Value::Char(_) => "char",
			Value::Decimal(_, _) => "dec",
			Value::String(_) => "string",
			Value::None => "nothing",
			_ => panic!()
		}.to_string()
	}
	
	fn replace_generic(generic: &TypeKind, with: &GenericType) -> Option<Box<TypeKind>> {
		match generic {
			TypeKind::Generic(GenericType { identifier, type_value: _ }) => {
				if identifier.to_string() == with.identifier.to_string() {
					with.type_value.clone()
				} else {
					None
				}
			},
			_ => None
		}
	}
	
	pub fn is_equal(&self, other: &Value) -> bool {
		match (self, TypeKind::infer(other)) {
			(_, Self::Alias(y)) => y.true_type.is_equal(other),
			(Self::Alias(x), _) => x.true_type.is_equal(other),
			(_, Self::Sum(y)) => y.contains(other),
			(Self::Sum(x), _) => x.contains(other),
			(Self::Struct(x), Self::Map(_)) => x.is_map_equal(other),
			(Self::Map(_), Self::Struct(y)) => y.is_map_equal(other),
			(_, Self::Generic(_)) => true,
			(Self::Generic(_), _) => true,
			(x, y) => x == &y,
		}
	}
	
	pub fn get_implemented_symbol(&self, symbol: &Value) -> Option<Value> {
		match self {
			Self::Primitive(x) => x.implement.get(symbol).cloned(),
			Self::Alias(x) => x.implement.get(symbol).cloned(),
			Self::Struct(x) => x.implement.get(symbol).cloned(),
			Self::Sum(x) => x.implement.get(symbol).cloned(),
			_ => None
		}
	}
}

impl Type for TypeKind {
	fn apply_arguments(&mut self, arguments: &[GenericType]) -> Result<(), String> {
		match self {
			Self::Primitive(x) => x.apply_arguments(arguments),
			Self::Array(x) => x.apply_arguments(arguments),
			Self::Function(x) => x.apply_arguments(arguments),
			Self::Generic(x) => x.apply_arguments(arguments),
			Self::Struct(x) => x.apply_arguments(arguments),
			Self::Map(x) => x.apply_arguments(arguments),
			Self::Alias(x) => x.apply_arguments(arguments),
			Self::Sum(x) => x.apply_arguments(arguments),
		}
	}
	fn convert_primitives_to_generics(&mut self, generics: &[GenericType]) {
		match self {
			Self::Primitive(x) => x.convert_primitives_to_generics(generics),
			Self::Array(x) => x.convert_primitives_to_generics(generics),
			Self::Function(x) => x.convert_primitives_to_generics(generics),
			Self::Generic(x) => x.convert_primitives_to_generics(generics),
			Self::Struct(x) => x.convert_primitives_to_generics(generics),
			Self::Map(x) => x.convert_primitives_to_generics(generics),
			Self::Alias(x) => x.convert_primitives_to_generics(generics),
			Self::Sum(x) => x.convert_primitives_to_generics(generics),
		}
	}
}

pub trait Type {
	fn apply_arguments(&mut self, _arguments: &[GenericType]) -> Result<(), String> {
		Ok(())
	}
	fn convert_primitives_to_generics(&mut self, _generics: &[GenericType]) {}
}

impl Display for TypeKind {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		match self {
			Self::Primitive(x) => {
				let implement: Vec<String> = x.implement.iter().map(|(k, v)| format!("{}: {v}", k)).collect();
				write!(f, "{}: use [{}]", x.identifier, implement.join(", "))
			},
			Self::Array(x) => write!(f, "array [{}]", x.element_type),
			Self::Map(x) => write!(f, "map [{}, {}]", x.key_type, x.value_type),
			Self::Function(x) => write!(f, "fun [{}]", x.return_type),
			Self::Generic(x) => write!(f, "<{}>", x.identifier),
			Self::Struct(x) => {
				let implement: Vec<String> = x.implement.iter().map(|(k, v)| format!("{}: {v}", k)).collect();
				let members: Vec<String> = x.members
					.iter()
					.map(|(symbol, member)| format!("{}: {member}", symbol.identifier))
					.collect();
				write!(f, "[{}]: use: [{}]", members.join(", "), implement.join(", "))
			},
			Self::Sum(x) => {
				let implement: Vec<String> = x.implement.iter().map(|(k, v)| format!("{}: {v}", k)).collect();
				let members: Vec<String> = x.types
					.iter()
					.map(|member| member.to_string())
					.collect();
				write!(f, "one of [{}]: use [{}]", members.join(", "), implement.join(", "))
			},
			Self::Alias(x) => {
				let implement: Vec<String> = x.implement.iter().map(|(k, v)| format!("{}: {v}", k)).collect();
				write!(f, "alias of {}: use [{}]", x.true_type, implement.join(", "))
			},
		}
	}
}
// 
// impl Eq for TypeKind {}
// 
// impl Hash for TypeKind {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		match self {
// 			Self::Primitive(x) => x.hash(state),
// 			Self::Array(x) => x.hash(state),
// 			Self::Function(x) => x.hash(state),
// 			Self::Generic(x) => x.hash(state),
// 			Self::Struct(x) => x.hash(state),
// 			Self::Map(x) => x.hash(state),
// 			Self::Alias(x) => x.hash(state),
// 			Self::Sum(x) => x.hash(state),
// 		}
// 	}
// }