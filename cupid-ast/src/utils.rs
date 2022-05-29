mod get_methods;
pub use get_methods::*;

mod type_of;
pub use type_of::*;

mod typed_untyped;
pub use typed_untyped::*;
pub use Typed::{Typed as IsTyped, Untyped};