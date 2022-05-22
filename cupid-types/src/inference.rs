use crate::*;

#[derive(Debug, Clone)]
pub enum Value {
	Array(Vec<Value>),
	Boolean(bool),
	Char(char),
	Decimal(i32, u32),
	Integer(i32),
	None,
	String(Cow<'static, str>),
	Tuple(Vec<Value>),
	Type(Type),
}

pub fn infer_type(value: &Value) -> Result<Type, ErrCode> {
	use Value::*;
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

fn infer_array(array: &Vec<Value>) -> Result<Type, ErrCode> {
	Ok(if let Some(first_element) = array.first() {
		array_type(infer_type(first_element)?)
	} else {
		(*ARRAY).to_owned()
	})
}

fn infer_tuple(tuple: &Vec<Value>) -> Result<Type, ErrCode> {
	let types: Result<Vec<Type>, ErrCode> = tuple.iter().map(|t| infer_type(t)).collect();
	Ok(tuple_type(types?))
}