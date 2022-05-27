pub type ErrCode = usize;

pub const ERR_CANNOT_INFER: usize = 100;
pub const ERR_TYPE_MISMATCH: usize = 400;
pub const ERR_NOT_FOUND: usize = 404;
pub const ERR_BAD_ACCESS: usize = 405;
pub const ERR_ALREADY_DEFINED: usize = 406;
pub const ERR_EXPECTED_TYPE: usize = 417;
pub const ERR_UNEXPECTED_TYPE: usize = 418;
pub const ERR_EXPECTED_FUNCTION: usize = 419;
