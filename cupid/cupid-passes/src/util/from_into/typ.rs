use cupid_util::ERR_EXPECTED_TYPE;
use crate::{Ident, Type, BaseType, Value, PassResult, PassErr, AsNode};

impl From<Ident> for Type {
    fn from(name: Ident) -> Self {
        Self {
            attr: name.attr,
            name,
            base: BaseType::Primitive,
            fields: vec![],
        }
    }
}

impl TryFrom<Value> for Type {
	type Error = PassErr;
	fn try_from(value: Value) -> PassResult<Type> {
		match value {
			Value::VType(typ) => Ok(typ),
			_ => Err((value.address(), ERR_EXPECTED_TYPE))
		}
	}
}

impl<'val> TryFrom<&'val Value> for &'val Type {
	type Error = PassErr;
	fn try_from(value: &'val Value) -> PassResult<&'val Type> {
		match &value {
			Value::VType(typ) => Ok(typ),
			_ => Err((value.address(), ERR_EXPECTED_TYPE))
		}
	}
}