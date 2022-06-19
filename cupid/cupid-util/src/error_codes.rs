pub type ErrCode = usize;

pub const ERR_CANNOT_INFER: ErrCode = 100;
pub const ERR_TYPE_MISMATCH: ErrCode = 400;
pub const ERR_NOT_IN_SCOPE: ErrCode = 403;
pub const ERR_NOT_FOUND: ErrCode = 404;
pub const ERR_BAD_ACCESS: ErrCode = 405;
pub const ERR_ALREADY_DEFINED: ErrCode = 406;
pub const ERR_CANNOT_UNIFY: ErrCode = 409;
pub const ERR_EXPECTED_TYPE: ErrCode = 417;
pub const ERR_UNEXPECTED_TYPE: ErrCode = 418;
pub const ERR_EXPECTED_FUNCTION: ErrCode = 419;
pub const ERR_EXPECTED_TRAIT: ErrCode = 420;
pub const ERR_EXPECTED_EXPRESSION: ErrCode = 421;
pub const ERR_UNCLOSED_DELIMITER: ErrCode = 422;

pub const ERR_UNREACHABLE: ErrCode = 500;

#[macro_export]
macro_rules! exhaustive {
	($node:tt) => { unreachable!("No other error types for {} nodes", $node) }
}