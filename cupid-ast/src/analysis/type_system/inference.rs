use crate::*;

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