use crate::*;

pub fn infer_type_from_scope(value: &Val, scope: &mut Env) -> Result<Type, ErrCode> {
	use Val::*;
	match &value {
		Array(array) => infer_array_from_scope(array, scope),
		Boolean(_) => get_primitive("bool", scope),
		Char(_) => get_primitive("char", scope),
		Decimal(..) => get_primitive("dec", scope),
		Integer(_) => get_primitive("int", scope),
		String(_) => get_primitive("string", scope),
		Tuple(tuple) => infer_tuple_from_scope(tuple, scope),
		Function(function) => infer_function_from_scope(function, scope),
		None | BuiltinPlaceholder => get_primitive("nothing", scope),
		_ => Err(ERR_CANNOT_INFER)
	}
}

fn get_primitive(primitive: &'static str, scope: &mut Env) -> Result<Type, ErrCode> {
	let ident = Ident::new_name(primitive);
	scope.get_type(&ident).map_err(|(_, e)| e)
}

fn infer_array_from_scope(array: &[Val], scope: &mut Env) -> Result<Type, ErrCode> {
	let mut ident = Ident::build().name("array".into());

	if let Some(first_element) = array.first() {
		let first_element = infer_type_from_scope(first_element, scope)?;
		ident = ident.one_generic(IsTyped(first_element.to_ident(), first_element));
	}

	get_type_and_unify(ident.build(), scope)
}

fn infer_tuple_from_scope(tuple: &[Val], scope: &mut Env) -> Result<Type, ErrCode> {
	let mut ident = Ident::new_name("tuple");

	let types: Result<Vec<Typed<Ident>>, ErrCode> = tuple
		.iter()
		.map(|t| Ok(IsTyped(Ident::default(), infer_type_from_scope(t, scope)?)))
		.collect();
	let types = types?;

	ident.attributes.generics = GenericList(types);
	
	get_type_and_unify(ident, scope)
}

fn infer_function_from_scope(function: &Function, scope: &mut Env) -> Result<Type, ErrCode> {
	let mut ident = Ident::new_name("fun");
	let mut param_types = function.params.iter().map(|p| p.type_hint.to_owned()).collect::<Vec<Typed<Ident>>>();
	let return_type = function.body.type_of(scope).map_err(|e| e.1)?;
	param_types.push(IsTyped(return_type.to_ident(), return_type)); // return type
	ident.attributes.generics = GenericList(param_types);
	get_type_and_unify(ident, scope)
}

fn get_type_and_unify(ident: Ident, scope: &mut Env) -> Result<Type, ErrCode> {
	let mut type_value = scope.get_type(&ident).map_err(|e| e.1)?;
	type_value.unify_with(&*ident.attributes.generics).map_err(|e| e.1)?;
	Ok(type_value)
}

pub fn infer_type(value: &Val) -> Result<Type, ErrCode> {
	use Val::*;
	match &value {
		Array(array) => infer_array(array),
		Boolean(_) => Ok((*BOOLEAN).to_owned()),
		Char(_) => Ok((*CHARACTER).to_owned()),
		Decimal(..) => Ok((*DECIMAL).to_owned()),
		Integer(_) => Ok((*INTEGER).to_owned()),
		String(_) => Ok((*STRING).to_owned()),
		Tuple(tuple) => infer_tuple(tuple),
		Function(function) => infer_function(function),
		None => Ok((*NOTHING).to_owned()),
		BuiltinPlaceholder => Ok((*NOTHING).to_owned()),
		_ => Err(ERR_CANNOT_INFER)
	}
}

fn infer_array(array: &[Val]) -> Result<Type, ErrCode> {
	Ok(if let Some(first_element) = array.first() {
		array_type(infer_type(first_element)?)
	} else {
		(*ARRAY).to_owned()
	})
}

fn infer_tuple(tuple: &[Val]) -> Result<Type, ErrCode> {
	let types: Result<Vec<Typed<Ident>>, ErrCode> = tuple
		.iter()
		.map(|t| Ok(IsTyped(Ident::default(), infer_type(t)?)))
		.collect();
	Ok(tuple_type(types?))
}

fn infer_function(function: &Function) -> Result<Type, ErrCode> {
	let mut function_type = (*FUNCTION).to_owned();
	if let Typed::Typed(_, t) = &function.body {
		function_type.unify(t).map_err(|(_, e)| e)?;
	}
	Ok(function_type)
}