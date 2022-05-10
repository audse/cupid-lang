// use std::hash::{Hash, Hasher};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::collections::HashMap;
// use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeKind<'src> {
	Array(ArrayType<'src>),
	Function(FunctionType<'src>),
	Generic(GenericType<'src>),
	Map(MapType<'src>),
	Primitive(PrimitiveType<'src>),
	Struct(StructType<'src>),
	Sum(SumType<'src>),
	Alias(AliasType<'src>),
	Type,
	Placeholder,
}

impl<'src> TypeKind<'src> {
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
		Self::Function(FunctionType { 
			return_type: Box::new(Self::new_generic("r")),
			param_types: vec![],
			implementation: Implementation::default() 
		})
	}
	pub fn infer(value: &Value) -> Self {
		match value {
			Value::Boolean(_) => Self::new_primitive("bool"),
			Value::Integer(_) =>  Self::new_primitive("int"),
			Value::Char(_) => Self::new_primitive("char"),
			Value::Decimal(_, _) => Self::new_primitive("dec"),
			Value::String(_) => Self::new_primitive("string"),
			Value::None => Self::new_primitive("nothing"),
			Value::Function(_) => Self::new_function(),
			Value::Array(array) => {
				if !array.is_empty() {
					let element_type = TypeKind::infer(&array[0].value);
					Self::new_array(element_type)
				} else {
					Self::new_array(Self::new_generic("e"))
				}
			},
			Value::Map(map) => {
				if let Some((key, (_, value))) = map.iter().next() {
					let key_type = TypeKind::infer(&key.value);
					let value_type = TypeKind::infer(&value.value);
					Self::new_map(key_type, value_type)
				} else {
					Self::new_map(Self::new_generic("k"), Self::new_generic("v"))
				}
			},
			Value::Type(t) => t.to_owned(),
			Value::Values(_) => Self::Placeholder,
			Value::Implementation(_) => Self::Type,
			Value::TypeIdentifier(_) => Self::Type,
			x => {
				println!("Cannot infer type of {:?}", x);
				unreachable!()
			}
		}
	}
	pub fn infer_name(value: &Value) -> &'static str {
		match value {
			Value::Boolean(_) => "bool",
			Value::Integer(_) =>  "int",
			Value::Char(_) => "char",
			Value::Decimal(_, _) => "dec",
			Value::String(_) => "string",
			Value::None => "nothing",
			Value::Array(_) => "array",
			Value::Map(_) => "map",
			Value::Function(..) => "fun",
			_ => panic!()
		}
	}
	pub fn infer_from_scope(value: &ValueNode, scope: &mut LexicalScope) -> Option<Self> {
		// TODO allow for args, e.g. array[<e>]
		let name = Value::String(Self::infer_name(&value.value).into());
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
	
	pub fn most_specific<'a>(a: &'a Self, b: &'a Self) -> &'a Self {
		use TypeKind::*;
		match (a, b) {
			(Array(x), Array(_)) => if matches!(*x.element_type, Generic(_)) { b } else { a },
			(Map(x), Map(_)) => if matches!(*x.key_type, Generic(_)) && matches!(*x.value_type, Generic(_)) { b } else { a },
			_ => a
		}
	}
	
	pub fn get_implementation(&mut self) -> &mut Implementation {
		match self {
			Self::Alias(x) => &mut x.implementation,
			Self::Array(x) => &mut x.implementation,
			Self::Function(x) => &mut x.implementation,
			Self::Map(x) => &mut x.implementation,
			Self::Primitive(x) => &mut x.implementation,
			Self::Struct(x) => &mut x.implementation,
			Self::Sum(x) => &mut x.implementation,
			_ => panic!("cannot get implementation")
		}
	}
	pub fn apply_args(&mut self, args: Vec<TypeKind>) -> Result<(), &str> {
		match self {
			Self::Array(x) => x.apply_args(args),
			_ => Ok(()) // todo
		}
	}
	pub fn get_name(&self) -> String {
		match self {
			Self::Alias(x) => format!("alias [{}]", x.true_type.get_name()),
			Self::Array(x) => format!("array [{}]", x.element_type.get_name()),
			Self::Function(x) => format!("fun [{}]", x.return_type.get_name()),
			Self::Generic(x) => {
				let type_value = if let Some(type_value) = &x.type_value {
					format!(": {}", type_value.get_name())
				} else {
					String::new()
				};
				format!("<{}{}>", x.identifier, type_value)
			},
			Self::Map(x) => format!("map [{}, {}]", x.key_type.get_name(), x.value_type.get_name()),
			Self::Primitive(x) => format!("{}", x.identifier),
			Self::Struct(x) => {
				let members: Vec<String> = x.members
					.iter()
					.map(|(k, v)| format!("{k}: {}", v.get_name()))
					.collect();
				format!("struct [{}]", members.join(", "))
			},
			Self::Sum(x) => {
				let members: Vec<String> = x.types
					.iter()
					.map(|t| format!("{}", t.get_name()))
					.collect();
				format!("sum [{}]", members.join(", "))
			},
			Self::Type => "type".to_string(),
			Self::Placeholder => "placeholder".to_string(),
			// _ => panic!()
		}
	}
}

impl<'src> Type for TypeKind<'src> {
	fn implement(&mut self, functions: HashMap<ValueNode, ValueNode>) {
		self.get_implementation().implement(functions)
	}
	fn implement_trait(&mut self, trait_symbol: SymbolNode, implementation: Implementation) {
		self.get_implementation().implement_trait(trait_symbol, implementation)
	}
	fn get_trait_function(&mut self, symbol: &SymbolNode) -> Option<(&Implementation, &FunctionNode)> { 
		self.get_implementation().get_trait_function(symbol) 
	}
}

pub trait Type {
	fn implement(&mut self, _functions: HashMap<ValueNode, ValueNode>) {}
	fn implement_trait(&mut self, _trait_symbol: SymbolNode, _implement: Implementation) {}
	fn get_function(&mut self, _symbol: &SymbolNode) -> Option<&FunctionNode> { None }
	fn get_trait_function(&mut self, _symbol: &SymbolNode) -> Option<(&Implementation, &FunctionNode)> { None }
	fn apply_args(&mut self, _args: Vec<TypeKind>) -> Result<(), &str> { Ok(()) }
}

impl<'src> Display for TypeKind<'src> {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		if let Self::Generic(_) = self {
			write!(f, "{}", self.get_name())
		} else {
			write!(f, "{} {}", self.get_name(), self.to_owned().get_implementation())
		}
	}
}


// IDK TODO

// impl PartialEq for TypeKind {
// 	fn eq(&self, other: &Self) -> bool {
// 		use TypeKind::*;
//     	match (self, other) {
// 			(Generic(_), _) => true,
// 			(_, Generic(_)) => true,
// 			(_, _) => false
// 		}
// 	}
// }
// 
// impl Eq for TypeKind {}
// 
// impl Hash for TypeKind {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		match self {
// 			Self::Alias(x) => x.hash(state),
// 			Self::Array(x) => x.hash(state),
// 			Self::Function(x) => x.hash(state),
// 			Self::Generic(x) => x.hash(state),
// 			Self::Map(x) => x.hash(state),
// 			Self::Primitive(x) => x.hash(state),
// 			Self::Struct(x) => x.hash(state),
// 			Self::Sum(x) => x.hash(state),
// 			_ => (),
// 		}
// 	}
// }