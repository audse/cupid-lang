use crate::*;

pub fn function_signature(mut generics: GenericParams, mut params: Vec<Ident>, return_type: Ident, scope: &mut Env) -> Type {
	let fun_type = scope.get_type(&FUNCTION.to_ident()).unwrap();
	params.push(return_type.to_owned());
	generics.0.insert(0, GenericParam(None, Some(return_type)));
	
	TypeBuilder::from(fun_type)
		.generics(generics)
		.unnamed_fields(params)
		.build()
}

pub fn last_field(fields: &FieldSet) -> Option<&Ident> {
	match fields {
		FieldSet::Named(fields) => fields.last().map(|f| &f.1),
		FieldSet::Unnamed(fields) => fields.last(),
		FieldSet::Empty => None
	}
}