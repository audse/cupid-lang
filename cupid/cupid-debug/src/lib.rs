use cupid_lex::node::ParseNode;
use cupid_util::*;
use cupid_ast::*;

pub mod context;
pub use context::*;

pub mod display;
pub use display::*;

pub mod to_err;
pub use to_err::*;

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