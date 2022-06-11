use crate::*;

/// Checks equality between types; either identical, or structurally equivalent
/// 
/// ## Example
/// ```
/// use cupid_analysis::TypeEq;
/// 
/// let base_array_type = cupid_builder::array!();
/// let aliased_array_type = cupid_builder::array!("my_alias");
/// 
/// assert_eq!(base_array_type.type_eq(&aliased_array_type), false);
/// assert_eq!(base_array_type.type_structure_eq(&aliased_array_type), true);
/// ```
pub trait TypeEq {
	fn type_eq(&self, other: &Self) -> bool;
	fn type_structure_eq(&self, other: &Self) -> bool;
}

fn find_match<'ty, T: TypeEq>(current: &'ty T, other: &'ty[T]) -> Option<&'ty T> {
	other.iter().find(|o| current.type_eq(o))
}

fn find_structure_match<'ty, T: TypeEq>(current: &'ty T, other: &'ty[T]) -> Option<&'ty T> {
	other.iter().find(|o| current.type_structure_eq(o))
}

fn next_is_structure_match<'ty, T: TypeEq>(current: &'ty T, mut other: impl Iterator<Item = &'ty T>) -> bool {
	other.next().as_ref().map(|o| current.type_structure_eq(o)).unwrap_or(false)
}

impl TypeEq for Type {
	fn type_eq(&self, other: &Self) -> bool {
		self.name.type_eq(&other.name)
			&& self.base_type == other.base_type
			&& self.fields.type_eq(&other.fields)
	}
	fn type_structure_eq(&self, other: &Self) -> bool {
		self.name.type_structure_eq(&other.name)
			&& self.base_type == other.base_type
			&& self.fields.type_structure_eq(&other.fields)
	}
}

impl TypeEq for FieldSet {
	fn type_eq(&self, other: &Self) -> bool {
		let mut other_fields = other.iter();
		self.len() == other.len()
			&& self.iter().all(|field| 
				find_match(field, other).is_some() || next_is_structure_match(field, &mut other_fields)
			)
	}
	fn type_structure_eq(&self, other: &Self) -> bool {
		self.len() == other.len()
			&& self.iter().all(|field| find_structure_match(field, other).is_some())
	}
}

impl TypeEq for Field {
	fn type_eq(&self, other: &Self) -> bool {
		self.name.type_eq(&other.name)
			&& match (&self.type_hint, &other.type_hint) {
				(Some(self_type), Some(other_type)) => self_type.type_eq(other_type),
				(None, None) => true,
				_ => false,
			}
	}
	fn type_structure_eq(&self, other: &Self) -> bool {
		self.name.type_structure_eq(&other.name)
			&& match (&self.type_hint, &other.type_hint) {
				(Some(self_type), Some(other_type)) => self_type.type_structure_eq(other_type),
				(None, None) => true,
				_ => false,
			}
	}
}

impl TypeEq for Ident {
	fn type_eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.attributes.generics.type_eq(&other.attributes.generics)
	}
	fn type_structure_eq(&self, other: &Self) -> bool {
		self.attributes.generics.type_structure_eq(&other.attributes.generics)
	}
}

impl TypeEq for Typed<Ident> {
	fn type_eq(&self, other: &Self) -> bool {
		self.inner().type_eq(other.inner())
			&& match (self, other) {
				(IsTyped(_, self_type), IsTyped(_, other_type)) => self_type.type_eq(other_type),
				_ => true
			}
	}
	fn type_structure_eq(&self, other: &Self) -> bool {
		self.inner().type_structure_eq(other.inner())
			&& match (self, other) {
				(IsTyped(_, self_type), IsTyped(_, other_type)) => self_type.type_structure_eq(other_type),
				_ => true
			}
	}
}

impl TypeEq for GenericList {
	fn type_eq(&self, other: &Self) -> bool {
		let mut other_generics = other.iter();
		self.len() == other.len() 
			&& self.iter().all(|generic| 
				find_match(generic, other).is_some() || next_is_structure_match(generic, &mut other_generics)
			)
	}
	fn type_structure_eq(&self, other: &Self) -> bool {
		self.len() == other.len()
			&& self.iter().all(|generic| find_structure_match(generic, other).is_some())
	}
}

// TODO trait equality