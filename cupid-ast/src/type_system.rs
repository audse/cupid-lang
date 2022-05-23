pub use lazy_static::lazy_static;

mod const_traits;
pub use const_traits::*;

mod const_types;
pub use const_types::*;

mod construct;
pub use construct::*;

mod inference;
pub use inference::*;

pub type ErrCode = usize;

pub const ERR_CANNOT_INFER: usize = 1;
// pub const ERR_