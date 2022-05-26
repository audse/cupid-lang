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
	pub fn apply_named(&mut self, arg: &mut TypedIdent) {
		// If the generic name matches the arg's name, sets the generic's value to the arg's value
		if let Some(param_name) = &mut self.0 {
			if param_name == &mut arg.0 { 
				self.1 = Some(arg.1.to_owned());
			}
		}
	}
	pub fn apply_unnamed(&mut self, arg: Ident) {
		// Sets the generic's value to the arg's value
		self.1 = Some(arg);
	}
	pub fn apply(&mut self, arg: &mut GenericParam) {
		// If the generic name matches the arg's name, or either is unnamed,
		// sets the generic's value to the arg's value
		if let (Some(param_name), Some(arg_name)) = (&mut self.0, &mut arg.0) {
			if param_name == arg_name {
				self.1 = arg.1.to_owned();
			}
		} else {
			self.1 = arg.1.to_owned();
		}
	}
}

impl Hash for GenericParam {
	fn hash<H: Hasher>(&self, _state: &mut H) {}
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GenericParams(pub Vec<GenericParam>);

impl GenericParams {
	pub fn apply(&mut self, args: GenericParams) {
		// Matches and applies generic params to arguments
		for (i, mut arg) in args.0.into_iter().enumerate() {
			if arg.0.is_some() {
				self.0.iter_mut().for_each(|param| param.apply(&mut arg));
			} else {
				self.0[i].apply(&mut arg);
			}
		}
	}
	pub fn apply_named(&mut self, args: Vec<TypedIdent>) {
		// Matches and applies type identifiers to generic params based on name
		for mut arg in args.into_iter() {
			self.0.iter_mut().for_each(|param| param.apply_named(&mut arg));
		}
	}
	pub fn apply_unnamed(&mut self, args: Vec<Ident>) {
		// Matches and applies type identifiers to generic params based on position
		for (i, arg) in args.into_iter().enumerate() {
			self.0[i].apply_unnamed(arg);
		}
	}
}

impl From<Vec<&'static str>> for GenericParams {
	fn from(names: Vec<&'static str>) -> Self {
    	Self(names.into_iter().map(GenericParam::new).collect::<Vec<GenericParam>>())
	}
}