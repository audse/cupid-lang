pub use lazy_static::lazy_static;

mod const_traits;
pub use const_traits::*;

mod const_types;
pub use const_types::*;

mod construct;
pub use construct::*;

mod inference;
pub use inference::*;

mod typed_expression;
pub use typed_expression::*;

#[allow(unused)]
pub type ErrCode = usize;

pub const ERR_CANNOT_INFER: usize = 1;