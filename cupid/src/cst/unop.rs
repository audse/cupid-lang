use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct UnOpSource<'src> {
    pub op: Token<'src>,
    pub expr_src: SourceId,
}

impl<'src> HasToken<'src> for UnOpSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.op == token
    }
}

impl<'src> From<UnOpSource<'src>> for ExprSource<'src> {
    fn from(value: UnOpSource<'src>) -> Self {
        ExprSource::UnOp(value.into())
    }
}
