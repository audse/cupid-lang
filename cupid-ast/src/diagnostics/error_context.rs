use crate::*;

pub trait ErrorContext: UseAttributes {
	fn get_context(&self) -> String;
	fn get_message(&self, code: usize) -> String;
}

pub trait Report: ErrorContext {
	fn report(&self, code: usize, extra_context: String) -> String;
}