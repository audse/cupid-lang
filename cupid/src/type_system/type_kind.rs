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
	Placeholder,
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
			Value::Function(..) => "fun",
			_ => panic!()
		}.to_string()
	}
	pub fn infer_from_scope(value: &ValueNode, scope: &mut LexicalScope) -> Option<Self> {
		// TODO allow for args, e.g. array[<e>]
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
	pub fn get_implementation(&mut self) -> &mut Implementation {
		match self {
			Self::Alias(x) => &mut x.implementation,
			Self::Array(x) => &mut x.implementation,
			Self::Function(x) => &mut x.implementation,
			Self::Map(x) => &mut x.implementation,
			Self::Primitive(x) => &mut x.implementation,
			Self::Struct(x) => &mut x.implementation,
			Self::Sum(x) => &mut x.implementation,
			_ => panic!()
		}
	}
	pub fn apply_args(&mut self, args: Vec<TypeKind>) -> Result<(), &str> {
		match self {
			Self::Array(x) => x.apply_args(args),
			_ => Ok(()) // todo
		}
	}
}

impl Type for TypeKind {
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

impl Display for TypeKind {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		match self {
			Self::Primitive(x) => write!(f, "{:8} {}", x.identifier, x.implementation),
			Self::Array(x) => write!(f, "{:8} {} {}", "array", x.element_type, x.implementation),
			Self::Map(x) => write!(f, "{}", x.to_string()),
			Self::Function(x) => write!(f, "{}", x.to_string()),
			Self::Generic(x) => write!(f, "<{}>", x.identifier),
			Self::Struct(x) => {
				let members: Vec<String> = x.members
					.iter()
					.map(|(symbol, member)| format!("{}: {member}", symbol.0))
					.collect();
				write!(f, "{:8} [{}] {}", "struct", members.join(", "), x.implementation)
			},
			Self::Sum(x) => {
				let members: Vec<String> = x.types
					.iter()
					.map(|member| member.to_string())
					.collect();
				write!(f, "{:8} [{}] {}", "sum", members.join(", "), x.implementation)
			},
			Self::Alias(x) => write!(f, "{:8} {} {}", "alias", x.true_type, x.implementation),
			Self::Type => write!(f, "{:8}", "type kind"),
			Self::Placeholder => write!(f, "{:8}", "placeholder"),
		}
	}
}
