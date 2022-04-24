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

mod types;
pub use types::*;

use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeKind {
	Array(ArrayType),
	Function(FunctionType),
	Generic(GenericType),
	Map(MapType),
	Primitive(PrimitiveType),
	Struct(StructType),
	Alias(AliasType),
}

impl TypeKind {
	pub fn from_value(value: &Value) -> &str {
		match value {
			Value::Boolean(_) => "bool",
			Value::Integer(_) => "int",
			Value::Char(_) => "char",
			Value::Decimal(_, _) => "dec",
			
			_ => unreachable!()
		}
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
			Self::Primitive(x) => write!(f, "{}", x.identifier),
			Self::Array(x) => write!(f, "array [{}]", x.element_type),
			Self::Map(x) => write!(f, "map [{}, {}]", x.key_type, x.value_type),
			Self::Function(x) => write!(f, "fun [{}]", x.return_type),
			Self::Generic(x) => write!(f, "<{}>", x.identifier),
			Self::Struct(x) => {
				let members: Vec<String> = x.members
					.iter()
					.map(|(symbol, member)| format!("{symbol}: {member}"))
					.collect();
				write!(f, "type [{:#?}]", members.join(", "))
			},
			Self::Alias(x) => write!(f, "type [{}]", x.true_type),
		}
	}
}

// TODO

// #[derive(Debug, Clone)]
// pub struct SumType {
// 	pub members: Vec<dyn Type>,
// }
// 
// impl Type for SumType {}
// 