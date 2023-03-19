use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct BinOpSource<'src> {
    pub left_src: SourceId,
    pub right_src: SourceId,
    pub op: Token<'src>,
}

impl<'src> HasToken<'src> for BinOpSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.op == token
    }
}

impl<'src> From<BinOpSource<'src>> for ExprSource<'src> {
    fn from(value: BinOpSource<'src>) -> Self {
        ExprSource::BinOp(value)
    }
}
