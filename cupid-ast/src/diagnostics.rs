use crate::{
	Env,
	Source,
};

mod error_context;
pub use error_context::*;

pub type ErrCode = usize;

pub const ERR_CANNOT_INFER: usize = 100;
pub const ERR_TYPE_MISMATCH: usize = 400;
pub const ERR_NOT_FOUND: usize = 404;
pub const ERR_BAD_ACCESS: usize = 405;
pub const ERR_ALREADY_DEFINED: usize = 406;
pub const ERR_EXPECTED_TYPE: usize = 417;
pub const ERR_EXPECTED_FUNCTION: usize = 418;

pub fn err_from_code(src: Source, code: ErrCode, scope: &mut Env) -> String {
	// println!("{scope:#?}");
	// let source_ast_node = scope.debug_find_by_source(src);
	let source_node = &scope.source_data[src];
	(match code {
		ERR_CANNOT_INFER => format!("100: Cannot infer type"),
		ERR_TYPE_MISMATCH => format!("400: Type mismatch"),
		ERR_NOT_FOUND => format!("404: Symbol not found in scope"),
		ERR_BAD_ACCESS => format!("405: Cannot access object with given property type"),
		ERR_ALREADY_DEFINED => format!("406: Attempted to set already defined symbol"),
		ERR_EXPECTED_TYPE => format!("417: Expected type"),
		ERR_EXPECTED_FUNCTION => format!("418: Expected function"),
		_ => format!("{code}")
	}) + &format!("\nsource: {source_node}")
}