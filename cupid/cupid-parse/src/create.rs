use crate::*;

pub mod nodes;
pub use nodes::*;

pub mod utils;
pub use utils::*;

pub mod value;
pub use value::*;

pub trait CreateAST: Sized {
	fn create_ast(node: &mut node::ParseNode, scope: &mut Env) -> Result<Self, ErrCode>;
}

pub fn create_file_ast(node: &mut node::ParseNode, scope: &mut Env) -> Result<Vec<Exp>, ErrCode> {
	node.children
		.iter_mut()
		.map(|c| Exp::create_ast(c, scope))
		.collect::<Result<Vec<Exp>, ErrCode>>()
}