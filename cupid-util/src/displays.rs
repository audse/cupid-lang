use crate::*;

pub struct DisplayVec<T>(pub Vec<T>, pub bool) where T: Display;

impl<T: Display + Clone> DisplayVec<T> {
	pub fn new(items: &Vec<T>, indent: bool) -> Self {
		Self(items.to_vec(), indent)
	}
}

impl<T: Display> Display for DisplayVec<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		let t = self.0.iter().map(|t| t.to_string());
		if self.1 {
			f.debug_list()
				.entries(t)
				.finish()
		} else {
			write!(f, "[{}]", t.collect::<Vec<String>>().join(", ").replace("\n", ""))
		}
	}
}

pub struct DisplayMap<K, V>(pub Vec<(K, V)>, pub bool) where K: Display, V: Display;

impl<K: Display + Clone, V: Display + Clone> DisplayMap<K, V> {
	pub fn new(items: &Vec<(K, V)>, indent: bool) -> Self {
		Self(items.to_vec(), indent)
	}
}

impl<K: Display + Clone, V: Display + Clone> Display for DisplayMap<K, V> {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		let t = self.0.iter().map(|(k, v)| format!("{k}: {v}"));
		if self.1 {
			f.debug_list()
				.entries(t)
				.finish()
		} else {
			write!(f, "[{}]", t.collect::<Vec<String>>().join(", ").replace("\n", ""))
		}
	}
}