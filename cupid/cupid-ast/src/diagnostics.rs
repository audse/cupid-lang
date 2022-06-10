use crate::*;

mod context;
pub use context::*;

mod display;
pub use display::*;

mod trace;
pub use trace::*;

pub fn err_from_code(code: ErrCode) -> String {
	match code {
		ERR_CANNOT_INFER => format!("{code}: Cannot infer type"),
		ERR_TYPE_MISMATCH => format!("{code}: Type mismatch"),
		ERR_NOT_FOUND => format!("{code}: Symbol not found in scope"),
		ERR_BAD_ACCESS => format!("{code}: Cannot access object with given property type"),
		ERR_ALREADY_DEFINED => format!("{code}: Attempted to set already defined symbol"),
		ERR_EXPECTED_TYPE => format!("{code}: Expected type"),
		ERR_EXPECTED_FUNCTION => format!("{code}: Expected function"),
		_ => format!("{code}")
	}
}

pub trait ToError: UseAttributes {
	fn to_err<T>(&self, code: usize) -> ASTResult<T> {
		Err(self.as_err(code))
	}
	fn to_err_mut<T>(&mut self, code: usize) -> ASTResult<T> {
		Err(self.as_err(code))
	}
	fn as_err(&self, code: usize) -> crate::ASTErr;
}