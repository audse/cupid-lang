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
	pub fn new_primitive(identifier: Cow<'static, str>) -> Self {
		Self::Primitive(PrimitiveType::new(identifier))
	}
	pub fn new_array(element: Option<&ValueNode>) -> Self {
		Self::Array(ArrayType { 
			element_type: if let Some(element) = element {
				if let Some(t) = TypeKind::infer_id(&element) {
					t
				} else {
					generic("e")
				}
			} else {
				generic("e")
			}, 
			implementation: Implementation::default() 
		})
	}
	pub fn new_map(pair: Option<(&ValueNode, &(usize, ValueNode))>) -> Self {
		let (key, value) = if let Some((key, (_, value))) = pair {
			if let (Some(k), Some(v)) = (TypeKind::infer_id(&key), TypeKind::infer_id(&value)) {
				(k, v)
			} else {
				(generic("k"), generic("v"))
			}
		} else {
			(generic("k"), generic("v"))
		};
		Self::Map(MapType { 
			key_type: key,
			value_type: value,
			implementation: Implementation::default() 
		})
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
	pub fn infer_id(value: &ValueNode) -> Option<TypeHintNode> {
		use Value::*;
		use TypeFlag::{Primitive, Inferred, Array as TArray, Map as TMap, Function as TFunction};
		let tokens = value.meta.tokens.to_owned();
		if let Some((name, flag, args)) = match &value.value {
			Boolean(_) => Some(("bool", Primitive, vec![])),
			Char(_) => Some(("char", Primitive, vec![])),
			Integer(_) => Some(("int", Primitive, vec![])),
			Decimal(_, _) => Some(("dec", Primitive, vec![])),
			None => Some(("nothing", Primitive, vec![])),
			String(_) => Some(("string", Primitive, vec![])),
			Array(a) => Some(("array", TArray, if !a.is_empty() {
				if let Some(t) = Self::infer_id(&a[0]) {
					vec![t]
				} else {
					vec![generic("e")]
				}
			} else {
				vec![generic("e")]
			})),
			Map(m) => Some(("map", TMap, if let Some((k, (_, v))) = &m.iter().next() {
				if let (Some(k), Some(v)) = (Self::infer_id(&k), Self::infer_id(&v)) {
					vec![k, v]
				} else {
					vec![generic("k"), generic("v")]
				}
			} else {
				vec![generic("k"), generic("v")]
			})),
			Function(_) => Some(("fun", TFunction, vec![generic("r")])),
			Values(_) => Option::None,
			_ => Option::None,
		} {
			Some(TypeHintNode::new(name.into(), vec![flag, Inferred], args, tokens))
		} else {
			Option::None
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
	pub fn get_name(&self) -> Cow<'static, str> {
		match self {
			Self::Alias(x) => format!("alias [{}]", x.true_type),
			Self::Array(x) => format!("array [{}]", x.element_type),
			Self::Function(x) => format!("fun [{}]", x.return_type),
			Self::Generic(x) => {
				let type_value = if let Some(type_value) = &x.type_value {
					format!(": {}", type_value)
				} else {
					String::new()
				};
				format!("<{}{}>", x.identifier, type_value)
			},
			Self::Map(x) => format!("map [{}, {}]", x.key_type, x.value_type),
			Self::Primitive(x) => format!("{}", x.identifier),
			Self::Struct(x) => {
				let members: Vec<String> = x.members
					.iter()
					.map(|(k, v)| format!("{k}: {v}"))
					.collect();
				format!("struct [{}]", members.join(", "))
			},
			Self::Sum(x) => {
				let members: Vec<String> = x.types
					.iter()
					.map(|t| format!("{t}"))
					.collect();
				format!("sum [{}]", members.join(", "))
			},
			Self::Type => "type".to_string(),
			Self::Placeholder => "placeholder".to_string(),
		}.into()
	}
}

impl Type for TypeKind {
	fn implement(&mut self, functions: HashMap<ValueNode, ValueNode>) {
		self.get_implementation().implement(functions)
	}
	fn implement_trait(&mut self, trait_symbol: TypeHintNode, implementation: Implementation) {
		self.get_implementation().implement_trait(trait_symbol, implementation)
	}
	fn get_trait_function(&mut self, symbol: &SymbolNode, scope: &mut LexicalScope) -> Option<(Implementation, FunctionNode)> { 
		self.get_implementation().get_trait_function(symbol, scope) 
	}
}

pub trait Type {
	fn implement(&mut self, _functions: HashMap<ValueNode, ValueNode>) {}
	fn implement_trait(&mut self, _trait_symbol: TypeHintNode, _implement: Implementation) {}
	fn get_function(&mut self, _symbol: &SymbolNode) -> Option<FunctionNode> { None }
	fn get_trait_function(&mut self, _symbol: &SymbolNode, _scope: &mut LexicalScope) -> Option<(Implementation, FunctionNode)> { None }
	fn apply_args(&mut self, _args: Vec<TypeKind>) -> Result<(), &str> { Ok(()) }
}

impl Display for TypeKind {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		if let Self::Generic(_) = self {
			write!(f, "{}", self.get_name())
		} else {
			write!(f, "{} {}", self.get_name(), self.to_owned().get_implementation())
		}
	}
}

fn generic(name: &'static str) -> TypeHintNode {
	TypeHintNode::generic(name.into(), vec![])
}
