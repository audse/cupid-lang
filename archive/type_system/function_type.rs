use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionType {
	pub return_type: Box<TypeKind>,
	pub param_types: Vec<TypeKind>, // TODO
	pub implementation: Implementation,
}

impl Type for FunctionType {}

impl PartialEq for FunctionType {
	fn eq(&self, other: &Self) -> bool {
		// TODO params as well?
		match (&*self.return_type, &*other.return_type) {
			(TypeKind::Generic(GenericType	{ identifier: _, type_value: _ }), _) => true,
			(_, TypeKind::Generic(GenericType	{ identifier: _, type_value: _ })) => true,
			_ => self.return_type == other.return_type
		}
	}
}

impl Eq for FunctionType {}

impl Hash for FunctionType {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.return_type.hash(state);
	}
}

impl Display for FunctionType {
	fn fmt(&self, f: &mut Formatter) -> DisplayResult {
		let param_types: Vec<String> = self.param_types.iter().map(|p| p.to_string()).collect();
		write!(f, "fun [{}: {}] {}", self.return_type, param_types.join(", "), self.implementation)
	}
}