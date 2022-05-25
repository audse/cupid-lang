use crate::*;

#[derive(Debug, Clone, Default)]
pub struct Trait {
	pub name: Ident,
	pub methods: Vec<Type>,
	pub bounds: Vec<Ident>,
}

impl PartialEq for Trait {
	fn eq(&self, other: &Self) -> bool {
    	self.name == other.name
	}
}

impl Eq for Trait {}

impl Hash for Trait {
	fn hash<H: Hasher>(&self, state: &mut H) {
    	self.name.hash(state);
	}
}


impl Trait {
	pub fn new_bin_op(name: &'static str) -> Self {
		// Creates a trait with a single operation method
		// E.g.
		// trait [t] add! = [
		//   fun [t] add = t self, t other => _
		// ]
		let generic = GenericParam::new("t");
		let name = Ident::new(name, vec![generic]);
		Trait {
			name: name.to_owned(),
			methods: vec![Type {
				name,
				fields: FieldSet::Unnamed(vec![
					Type::primitive("t").into_ident(), // left
					Type::primitive("t").into_ident(), // right
					Type::primitive("t").into_ident(), // return type
				]),
				methods: vec![],
				traits: vec![],
				inherits: BaseType::Function,
			}],
			bounds: vec![],
		}
	}	
	pub fn into_ident(&self) -> Ident {
		self.name.to_owned()
	}
}

impl UseAttributes for Trait {
	fn attributes(&mut self) -> &mut Attributes {
    	self.name.attributes()
	}
}

impl Analyze for Trait {} // TODO

impl ToIdent for Trait {
	fn to_ident(&self) -> Ident {
    	self.name.to_owned()
	}
}

impl From<Trait> for Val {
	fn from(t: Trait) -> Val {
		Val::Trait(t)
	}
}