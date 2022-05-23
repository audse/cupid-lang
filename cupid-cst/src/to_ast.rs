pub struct ParseNode;
pub struct Source;

pub trait ToAST {
	fn to_ast(node: &mut ParseNode, meta: &mut Vec<Source>) -> Self;
}

