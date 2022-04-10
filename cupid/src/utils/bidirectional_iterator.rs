#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BiDirectionalIterator<T> where T: Clone + PartialEq {
	index: usize,
	items: Vec<T>,
}

impl<T> BiDirectionalIterator<T> where T: Clone + PartialEq {
	pub fn new(items: Vec<T>) -> Self {
		Self { 
			index: 0,
			items,
		}
	}
}

impl<T> Iterator for BiDirectionalIterator<T> where T: Clone + PartialEq {
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

impl<T> BiDirectionalIterator<T> where T: Clone + PartialEq {
	pub fn at_end(&self) -> bool {
		self.index >= self.items.len() - 1
	}
	pub fn back(&mut self, amount: usize) -> Option<&T> {
		self.index -= amount;
		self.items.get(self.index)
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
	pub fn expect(&mut self, val: T) -> Option<T> {
		let next = self.peek_expect(0, val);
		if next {
			return self.next();
		}
		None
	}
	pub fn peek(&self, amount: usize) -> Option<&T> {
		if !self.at_end() {
			return self.items.get(self.index + amount);
		}
		None
	}
	pub fn peek_expect(&self, amount: usize, val: T) -> bool {
		if let Some(item) = self.peek(amount) {
			if item.clone() == val {
				return true;
			}
		}
		false
	}
}