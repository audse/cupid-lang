use crate::{
	Env,
	Source,
};

mod display;
pub use display::*;

mod error_codes;
pub use error_codes::*;

mod error_context;
pub use error_context::*;

pub fn err_from_code(src: Source, code: ErrCode, scope: &mut Env) -> String {
	// println!("{scope}");
	
	println!("{} \n{}", scope.fmt_current(), scope.closures[scope.current_closure].1.as_table());
	let source_node = &scope.source_data[src];
	(match code {
		ERR_CANNOT_INFER => format!("{code}: Cannot infer type"),
		ERR_TYPE_MISMATCH => format!("{code}: Type mismatch"),
		ERR_NOT_FOUND => format!("{code}: Symbol not found in scope"),
		ERR_BAD_ACCESS => format!("{code}: Cannot access object with given property type"),
		ERR_ALREADY_DEFINED => format!("{code}: Attempted to set already defined symbol"),
		ERR_EXPECTED_TYPE => format!("{code}: Expected type"),
		ERR_EXPECTED_FUNCTION => format!("{code}: Expected function"),
		_ => format!("{code}")
	}) + &format!("\nsource: {source_node}")
}