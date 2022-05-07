// use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::*;

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
	Type,
}

impl TypeKind {
	pub fn new_primitive(identifier: &str) -> Self {
		Self::Primitive(PrimitiveType::new(identifier))
	}
	pub fn new_array(element_type: Self) -> Self {
		Self::Array(ArrayType { element_type: Box::new(element_type), implementation: Implementation::default() })
	}
	pub fn new_map(key_type: Self, value_type: Self) -> Self {
		Self::Map(MapType { key_type: Box::new(key_type), value_type: Box::new(value_type), implementation: Implementation::default() })
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
			// Value::FunctionBody(..) => Self::new_function(),
			Value::Function(_) => Self::new_function(),
			Value::Array(array) => {
				if !array.is_empty() {
					let element_type = TypeKind::infer(&array[0]);
					Self::new_array(element_type)
				} else {
					Self::new_array(Self::new_generic("e"))
				}
			},
			Value::Map(map) => {
				if let Some((key, (_, value))) = map.iter().next() {
					let key_type = TypeKind::infer(key);
					let value_type = TypeKind::infer(value);
					Self::new_map(key_type, value_type)
				} else {
					Self::new_map(Self::new_generic("k"), Self::new_generic("v"))
				}
			},
			Value::Type(t) => t.clone(),
			Value::Values(_) => Self::Type,
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
			Value::Array(_) => "array",
			Value::Map(_) => "map",
			// Value::FunctionBody(..) => "fun",
			Value::Function(..) => "fun",
			_ => panic!()
		}.to_string()
	}
	pub fn infer_from_scope(value: &ValueNode, scope: &mut LexicalScope) -> Option<Self> {
		let name = Value::String(Self::infer_name(&value.value));
		let symbol_value = ValueNode::new(name, Meta::with_tokens(value.meta.tokens.to_owned()));
		let symbol = SymbolNode(symbol_value);
		if let Ok(type_kind) = scope.get_symbol(&symbol) {
			if let Value::Type(type_kind) = type_kind.value {
				Some(type_kind)
			} else {
				None
			}
		} else {
			None
		}
	}
	
	pub fn replace_generic(generic: &TypeKind, with: &GenericType) -> Option<Box<TypeKind>> {
		match generic {
			TypeKind::Generic(GenericType { identifier, type_value: _ }) => {
				if identifier == &with.identifier {
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
	pub fn is_function(&self) -> bool {
		matches!(self, Self::Function(_))
	}
	pub fn get_implementation(&mut self) -> &mut Implementation {
		match self {
			Self::Alias(x) => &mut x.implementation,
			Self::Array(x) => &mut x.implementation,
			Self::Map(x) => &mut x.implementation,
			Self::Primitive(x) => &mut x.implementation,
			Self::Struct(x) => &mut x.implementation,
			Self::Sum(x) => &mut x.implementation,
			_ => panic!()
		}
	}
	pub fn get_name(&self) -> String {
		match self {
			Self::Primitive(x) => x.identifier.to_string(),
			Self::Array(x) => format!("array [{}]", x.element_type.get_name()),
			Self::Function(x) => format!("fun [{}]", x.return_type.get_name()),
			Self::Generic(_) => "generic".to_string(),
			Self::Struct(_) => "struct".to_string(),
			Self::Map(x) => format!("map [{}, {}]", x.key_type.get_name(), x.value_type.get_name()),
			Self::Alias(x) => format!("alias of {}", x.true_type.get_name()),
			Self::Sum(_) => "sum".to_string(),
			Self::Type => "type kind".to_string(),
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
			_ => panic!(),
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
			_ => panic!(),
		}
	}
	fn implement(&mut self, functions: HashMap<ValueNode, ValueNode>) {
		self.get_implementation().implement(functions)
	}
	fn implement_trait(&mut self, trait_symbol: SymbolNode, functions: HashMap<ValueNode, ValueNode>) {
		self.get_implementation().implement_trait(trait_symbol, Implementation { functions, traits: HashMap::new() })
	}
	fn find_function(&mut self, symbol: &SymbolNode, scope: &mut LexicalScope) -> Option<ValueNode> {
		self.get_implementation().find_function(symbol, scope)
	}
	fn find_function_value(&mut self, symbol: &SymbolNode, scope: &mut LexicalScope) -> Option<FunctionNode> {
		self.get_implementation().find_function_value(symbol, scope)
	}
}

pub trait Type {
	fn apply_arguments(&mut self, _arguments: &[GenericType]) -> Result<(), String> { Ok(()) }
	fn convert_primitives_to_generics(&mut self, _generics: &[GenericType]) {}
	fn implement(&mut self, _functions: HashMap<ValueNode, ValueNode>) {}
	fn implement_trait(&mut self, _trait_symbol: SymbolNode, _implement: HashMap<ValueNode, ValueNode>) {}
	fn find_function(&mut self, _symbol: &SymbolNode, _scope: &mut LexicalScope) -> Option<ValueNode> { None }
	fn find_function_value(&mut self, _symbol: &SymbolNode, _scope: &mut LexicalScope) -> Option<FunctionNode> { None }
}

impl Display for TypeKind {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		match self {
			Self::Primitive(x) => write!(f, "{} [{}]", x.identifier, x.implementation),
			Self::Array(x) => write!(f, "array [{}]", x.element_type),
			Self::Map(x) => write!(f, "map [{}, {}]", x.key_type, x.value_type),
			Self::Function(x) => write!(f, "fun [{}]", x.return_type),
			Self::Generic(x) => write!(f, "<{}>", x.identifier),
			Self::Struct(x) => {
				let members: Vec<String> = x.members
					.iter()
					.map(|(symbol, member)| format!("{}: {member}", symbol.0))
					.collect();
				write!(f, "[{}] [{}]", members.join(", "), x.implementation)
			},
			Self::Sum(x) => {
				let members: Vec<String> = x.types
					.iter()
					.map(|member| member.to_string())
					.collect();
				write!(f, "one of [{}] [{}]", members.join(", "), x.implementation)
			},
			Self::Alias(x) => write!(f, "alias of {} [{}]", x.true_type, x.implementation),
			Self::Type => write!(f, "type kind"),
		}
	}
}
