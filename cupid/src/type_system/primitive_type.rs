use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimitiveType {
	pub identifier: Cow<'static, str>,
	pub implementation: Implementation,
}

impl PrimitiveType {
	pub fn new(identifier: Cow<'static, str>) -> Self {
		Self { 
			identifier,
			implementation: Implementation::default()
		}
	}
}

impl Type for PrimitiveType {}

impl PartialEq for PrimitiveType {
	fn eq(&self, other: &Self) -> bool {
    	self.identifier == other.identifier
	}
}

impl Eq for PrimitiveType {}

impl Hash for PrimitiveType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.identifier.hash(state);
	}
}

impl Display for PrimitiveType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		write!(f, "{}", self.identifier)
	}
}