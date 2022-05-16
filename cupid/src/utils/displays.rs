use crate::*;

use std::vec::Vec;

pub struct DisplayVec<T>(pub Vec<T>, pub bool) where T: Display;

impl<T: Display + Clone> DisplayVec<T> {
	pub fn new(items: &Vec<T>, indent: bool) -> Self {
		Self(items.to_vec(), indent)
	}
}

impl<T: Display> Display for DisplayVec<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		let t: Vec<String> = self.0.iter().map(|t| t.to_string()).collect();
		if self.1 {
			f.debug_list()
				.entries(t)
				.finish()
		} else {
			write!(f, "[{}]", t.join(", ").replace("\n", ""))
		}
	}
}