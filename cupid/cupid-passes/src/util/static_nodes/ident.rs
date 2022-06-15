cupid_util::node_builder! {
	#[derive(Debug, Default, Clone)]
	pub IdentBuilder => pub Ident {
		pub generics: Vec<Ident>,
		pub name: cupid_util::Str,
		pub namespace: Box<Ident>,
	}
}

impl std::hash::Hash for Ident {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
}

impl PartialEq for Ident {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name 
			&& self.generics.len() == other.generics.len()
	}
}

impl Eq for Ident {}