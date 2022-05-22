pub use std::{
	vec::Vec,
	fmt::Display,
	fmt::Result as DisplayResult,
	fmt::Formatter,
};

mod bidirectional_iterator;
pub use bidirectional_iterator::*;

mod displays;
pub use displays::*;

mod static_map;

mod strings;
pub use strings::*;