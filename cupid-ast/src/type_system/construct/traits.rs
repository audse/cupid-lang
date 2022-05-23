use crate::*;

#[derive(Debug, Clone, Default)]
pub struct Trait {
	pub name: Str,
	pub generics: Vec<GenericParam>,
	pub methods: Vec<Type>,
	pub bounds: Vec<Ident>,
}

impl Trait {
	pub fn new_bin_op(name: &'static str) -> Self {
		// Creates a trait with a single operation method
		// E.g.
		// trait [t] add! = [
		//   fun [t] add = t self, t other => _
		// ]
		let generic = GenericParam::new("t");
		Trait {
			name: Cow::Borrowed(name),
			generics: vec![generic.to_owned()],
			methods: vec![Type {
				name: None,
				generics: vec![generic.to_owned()],
				fields: FieldSet::Unnamed(vec![
					Type::primitive("t").into_ident(), // left
					Type::primitive("t").into_ident(), // right
					Type::primitive("t").into_ident(), // return type
				]),
				methods: vec![],
				traits: vec![],
			}],
			bounds: vec![],
		}
	}	
	pub fn into_ident(&self) -> Ident {
		Ident { name: self.name.to_owned(), generics: self.generics.to_owned() }
	}
}