use crate::{
	lazy_static,
	Trait,
};

lazy_static! {
	pub static ref ADD: Trait = Trait::new_bin_op("add!");
	pub static ref SUBTRACT: Trait = Trait::new_bin_op("subtract!");
	pub static ref MULTIPLY: Trait = Trait::new_bin_op("multiply!");
	pub static ref DIVIDE: Trait = Trait::new_bin_op("divide!");
	pub static ref EQUAL: Trait = Trait::new_bin_op("equal!");
	pub static ref NOT_EQUAL: Trait = Trait::new_bin_op("not_equal!");
	pub static ref GET: Trait = Trait::new_bin_op("get!");
}