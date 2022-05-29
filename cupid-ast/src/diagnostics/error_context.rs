use crate::*;

pub trait ErrorContext: UseAttributes {
	fn get_source(&self) -> &ParseNode;
	fn get_context(&self) -> String {
		quick_fmt!("Accessing node \nSource:", @pretty=true self.get_source())
	}
	fn get_message(&self, code: usize) -> String;
}

pub trait Report: ErrorContext {
	fn report(&self, code: usize, extra_context: String) -> String;
}