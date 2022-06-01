use crate::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Tabled)]
pub struct FieldSet(
	#[tabled(display_with="fmt_field_set")]
	pub Vec<(Option<Str>, Typed<Ident>)>
);

fn fmt_field_set(field_set: &[(Option<Str>, Typed<Ident>)]) -> String {
	let fields = field_set.iter().map(|(s, i)| quick_fmt!(fmt_option!(s), i)).collect::<Vec<String>>();
	fmt_list!(fields, ", ")
}

impl std::ops::Deref for FieldSet {
	type Target = Vec<(Option<Str>, Typed<Ident>)>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for FieldSet {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}