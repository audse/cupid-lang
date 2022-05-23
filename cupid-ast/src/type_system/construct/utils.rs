use crate::*;

pub fn make_function_signature(generics: Vec<GenericParam>, mut params: Vec<Ident>, return_type: Ident, scope: &mut Env) -> Type {
	let fun_type_ident = &FUNCTION.to_owned().into_ident();
	let mut fun_type = scope.get_type(&fun_type_ident).unwrap();
	fun_type.generics = generics;
	params.push(return_type);
	fun_type.fields = FieldSet::Unnamed(params);
	fun_type
}