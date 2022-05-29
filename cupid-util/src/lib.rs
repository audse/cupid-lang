use std::{
	vec::Vec,
	fmt::Display,
	fmt::Result as DisplayResult,
	fmt::Formatter,
};

mod bidirectional_iterator;
pub use bidirectional_iterator::*;

mod builder;
pub use builder::*;

mod displays;
pub use displays::*;

mod fmt;
pub use fmt::*;

mod static_map;

mod strings;
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

pub trait MapMut<T, R> {
	fn map_mut<F: FnOnce(&mut T) -> R>(&mut self, f: F) -> Option<R>;
}

impl<T, R> MapMut<T, R> for Option<T> {
	fn map_mut<F: FnOnce(&mut T) -> R>(&mut self, f: F) -> Option<R> {
		self.as_mut().map(f)
	}
}