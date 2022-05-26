use std::{
	fmt::Display,
	fmt::Formatter,
	fmt::Result as DisplayResult,
	fmt::Debug,
	borrow::Cow,
};
use serde::{
	Serialize,
	Deserialize
};
pub use colored::*;

mod errors;
pub use errors::*;

mod token;
pub use token::*;

mod tokenizer;
pub use tokenizer::*;