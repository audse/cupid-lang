use crate::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Tabled)]
pub struct GenericList(
	#[tabled(display_with = "fmt_vec")]
	pub Vec<Typed<Ident>>
);

impl GenericList {
	pub fn set_symbols(&self, scope: &mut Env) {
		for generic in (*self).iter() {
			if scope.get_type(generic).is_err() {
				// TODO is this right
				scope.set_symbol(generic, SymbolValue { 
					value: Some(Value::build()
						.value(Untyped(Type::build()
							.name(generic.to_owned().into_inner())
							.build()
						))
						.attributes(generic.attributes.to_owned())
						.build()),
					type_hint: TYPE.to_ident(), 
					mutable: false 
				})
			}
		}
	}
}

impl From<Vec<&'static str>> for GenericList {
	fn from(names: Vec<&'static str>) -> Self {
    	Self(names.into_iter().map(|n| Untyped(Ident::new_name(n)) ).collect::<Vec<Typed<Ident>>>())
	}
}

impl std::ops::Deref for GenericList {
	type Target = Vec<Typed<Ident>>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for GenericList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}