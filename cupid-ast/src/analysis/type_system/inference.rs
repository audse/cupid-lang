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
		_ => Err(ERR_CANNOT_INFER)
	}
}

fn infer_array(array: &[Val]) -> Result<Type, ErrCode> {
	Ok(if let Some(first_element) = array.first() {
		array_type(infer_type(first_element)?.into_ident())
	} else {
		(*ARRAY).to_owned()
	})
}

fn infer_tuple(tuple: &[Val]) -> Result<Type, ErrCode> {
	let types: Result<Vec<Ident>, ErrCode> = tuple
		.iter()
		.map(|t| Ok(infer_type(t)?.into_ident()))
		.collect();
	Ok(tuple_type(types?))
}