use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericType {
	pub identifier: Cow<'static, str>,
	pub type_value: Option<TypeHintNode>
}

impl GenericType {
	pub const fn new_const(identifier: &'static str) -> Self {
		Self { identifier: Cow::Borrowed(identifier), type_value: None }
	}
	pub fn new(identifier: &str, type_value: Option<TypeHintNode>) -> Self {
		Self { identifier: Cow::Owned(identifier.to_string()), type_value }
	}
}

impl From<&SymbolNode> for GenericType {
	fn from(symbol: &SymbolNode) -> Self {
		if let Value::String(string) = &symbol.0.value {
			Self {
				identifier: string.to_owned(),
				type_value: None
			}
		} else {
			Self::new("t", None)
		}
	}
}

impl From<&'static str> for GenericType {
	fn from(string: &'static str) -> Self {
		Self { identifier: string.into(), type_value: None }
	}
}

impl Into<TypeKind> for GenericType {
	fn into(self) -> TypeKind { TypeKind::Generic(self) }
}
impl Into<Value> for GenericType {
	fn into(self) -> Value { Value::Type(self.into()) }
}

impl Type for GenericType {}

impl PartialEq for GenericType {
	fn eq(&self, other: &Self) -> bool {
		self.identifier == other.identifier
		&& match (&self.type_value, &other.type_value) {
			(Some(x), Some(y)) => x == y,
			_ => true,
		}
	}
}

impl Eq for GenericType {}

impl Hash for GenericType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.identifier.hash(state);
		self.type_value.hash(state);
	}
}

impl Display for GenericType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let type_value = if let Some(type_value) = &self.type_value {
			format!(": {}", type_value)
		} else {
			String::new()
		};
		write!(f, "<{}{}>", self.identifier, type_value)
	}
}