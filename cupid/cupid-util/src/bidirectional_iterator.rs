#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct BiDirectionalIterator<T> {
	index: usize,
	items: Vec<T>,
}

impl<T: Clone> BiDirectionalIterator<T> {
	pub fn new(items: Vec<T>) -> Self {
		Self { 
			index: 0,
			items,
		}
	}
}

impl<T: Clone> Iterator for BiDirectionalIterator<T> {
	type Item = T;
	fn next(&mut self) -> Option<T> {
		if !self.at_end() {
			let val = self.items[self.index].clone();
			self.index += 1;
			Some(val)
		} else {
			None
		}
	}
}

impl<T: Clone> BiDirectionalIterator<T> {
	pub fn at_end(&self) -> bool {
		self.index >= self.items.len()
	}
	pub fn peek_back(&self, amount: usize) -> Option<&T> {
		self.items.get(self.index - amount)
	}
	pub fn index(&self) -> usize {
		self.index
	}
	pub fn goto(&mut self, pos: usize) {
		self.index = pos;
	}
	pub fn peek(&self, amount: usize) -> Option<&T> {
		return self.items.get(self.index + amount)
	}
}