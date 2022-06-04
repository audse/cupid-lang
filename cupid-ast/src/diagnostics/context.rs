use crate::*;

mod nodes;
pub use nodes::*;

pub trait ErrorContext: UseAttributes + ToError {
	fn source_node<'env>(&'env self, scope: &'env mut Env) -> &'env ParseNode {
		scope.get_source_node(self.source())
	}
	fn fmt_source(&self, scope: &mut Env) -> String {
		quick_fmt!(@pretty=true self.source_node(scope))
	}
	fn context(&self, scope: &mut Env, source: &str) -> String;
	fn message(&self, code: ErrCode) -> String {
		err_from_code(code)
	}
}

pub trait Report: ErrorContext {
	fn report(&self, code: ErrCode, extra_context: String) -> String;
}