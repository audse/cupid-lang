pub use lazy_static::lazy_static;

mod const_traits;
pub use const_traits::*;

mod const_types;
pub use const_types::*;

mod definition;
pub use definition::*;

mod inference;
pub use inference::*;

mod type_builder;
pub use type_builder::*;

mod typed_expression;
pub use typed_expression::*;
pub use Typed::{Typed as IsTyped, Untyped};