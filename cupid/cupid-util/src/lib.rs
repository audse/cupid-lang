pub mod bidirectional_iterator;
pub use bidirectional_iterator::*;

pub mod builder;
pub use builder::*;

pub mod error_codes;
pub use error_codes::*;

pub mod fmt;
pub use fmt::*;

pub mod invert_option;
pub use invert_option::*;

pub mod map_mut;
pub use map_mut::*;

pub mod strings;
pub use strings::*;

/// Shorthand `Box::new(...)` as an associated trait. Implemented
/// for any type that can be boxed.
/// 
/// # Examples
/// ```
/// use cupid_util::Bx;
/// 
/// let word = "hello";
/// assert_eq!(word.bx(), Box::new(word));
/// ```
pub trait Bx where Self: Sized {
	fn bx(self) -> Box<Self> { Box::new(self) }
}

impl<T: Sized> Bx for T {}

pub trait BxOption<T> where Self: Sized {
	fn bx_option(self) -> Option<Box<T>>;
}

impl<T: Sized> BxOption<T> for Option<T> {
	fn bx_option(self) -> Option<Box<T>> {
		self.map(|t| Box::new(t))
	}
}

/// Suppresses compiler warnings inside the given block. Works similar
/// to `todo!()` macro, but for items rather than expressions.
#[macro_export]
macro_rules! ignore_errors {
    ($($_:item)*) => {}
}

/// Does literally nothing, a more semantic version of `todo!()` for some cases
#[macro_export]
macro_rules! placeholder {
	(...) => {}
}