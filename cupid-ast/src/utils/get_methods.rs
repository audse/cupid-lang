use crate::{
	Method,
	Type,
	Trait
};

pub trait Methods {
	fn methods(&self) -> &[Method];
	fn methods_mut(&mut self) -> &mut [Method];
}

impl Methods for Type {
	fn methods(&self) -> &[Method] {
		&self.methods
	}
	fn methods_mut(&mut self) -> &mut [Method] {
		&mut self.methods
	}
}

impl Methods for Trait {
	fn methods(&self) -> &[Method] {
		&self.methods
	}
	fn methods_mut(&mut self) -> &mut [Method] {
		&mut self.methods
	}
}