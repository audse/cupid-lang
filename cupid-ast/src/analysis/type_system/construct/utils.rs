use crate::*;

pub fn function_signature(generics: Vec<GenericParam>, mut params: Vec<Ident>, return_type: Ident, scope: &mut Env) -> Type {
	let fun_type_ident = &FUNCTION.to_owned().into_ident();
	let mut fun_type = scope.get_type(fun_type_ident).unwrap();
	fun_type.name.attributes.generics = generics;
	params.push(return_type);
	fun_type.fields = FieldSet::Unnamed(params);
	fun_type
}

pub fn last_field(fields: &FieldSet) -> Option<&Ident> {
	match fields {
		FieldSet::Named(fields) => fields.last().map(|f| &f.1),
		FieldSet::Unnamed(fields) => fields.last(),
		FieldSet::Sum(fields) => last_field(fields),
		FieldSet::Empty => None
	}
}