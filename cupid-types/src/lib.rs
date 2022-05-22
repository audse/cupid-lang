
// Stdlib
pub use std::borrow::Cow;

// External
pub use lazy_static::lazy_static;

mod construct;
pub use construct::*;

mod inference;
pub use inference::*;

pub type ErrCode = usize;

pub const ERR_CANNOT_INFER: usize = 1;
// pub const ERR_