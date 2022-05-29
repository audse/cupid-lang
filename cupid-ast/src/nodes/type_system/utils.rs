use crate::*;

pub fn function_signature(mut generics: GenericList, mut params: Vec<Ident>, return_type: Ident, scope: &mut Env) -> Type {
	let fun_type = scope.get_type(&FUNCTION.to_ident()).unwrap();
	params.push(return_type.to_owned());
	generics.0.insert(0, Generic { ident: None, arg: Some(return_type) });
	
	TypeBuilder::from(fun_type)
		.generics(generics)
		.unnamed_fields(params)
		.build()
}