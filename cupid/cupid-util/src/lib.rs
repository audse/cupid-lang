pub mod bidirectional_iterator;
pub use bidirectional_iterator::*;

pub mod builder;
pub use builder::*;

pub mod error_codes;
pub use error_codes::*;

pub mod fmt;
pub use fmt::*;

pub mod strings;
pub use strings::*;

pub fn invert<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    x.map_or(Ok(None), |v| v.map(Some))
}

pub trait InvertOption<T, E> {
	fn invert(self) -> Result<Option<T>, E>;
}

impl<T, E> InvertOption<T, E> for Option<Result<T, E>> {
	fn invert(self) -> Result<Option<T>, E> {
		self.map_or(Ok(None), |v| v.map(Some))
	}
}

pub trait MapMut<'a, T: 'a, R: 'a> {
	fn map_mut<F: FnOnce(&'a mut T) -> R>(&'a mut self, f: F) -> Option<R>;
}

impl<'a, T: 'a, R: 'a> MapMut<'a, T, R> for Option<T> {
	fn map_mut<F: FnOnce(&'a mut T) -> R>(&'a mut self, f: F) -> Option<R> {
		self.as_mut().map(f)
	}
}