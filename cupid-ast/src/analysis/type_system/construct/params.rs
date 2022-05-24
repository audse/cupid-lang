use crate::*;

#[derive(Debug, Clone, Default)]
pub struct GenericParam(pub Option<Str>, pub Option<Ident>);

impl PartialEq for GenericParam {
	fn eq(&self, other: &Self) -> bool {
		if let (Some(name), Some(other_name)) = (&self.0, &other.0) {
			name == other_name
		} else if let (None, None) = (&self.0, &other.0) {
			self.1 == other.1
		} else {
			true
		}
	}
}

impl Eq for GenericParam {}

impl GenericParam {
	pub const fn new(name: &'static str) -> Self { 
		Self(Some(Cow::Borrowed(name)), None) 
	}
}

impl Hash for GenericParam {
	fn hash<H: Hasher>(&self, state: &mut H) {
    	self.0.hash(state);
		self.1.hash(state);
	}
}