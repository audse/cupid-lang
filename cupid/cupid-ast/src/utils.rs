pub mod get_methods;
pub use get_methods::*;

pub mod typed_untyped;
pub use typed_untyped::*;
pub use Typed::{Typed as IsTyped, Untyped};