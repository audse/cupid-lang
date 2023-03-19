use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct ReturnSource<'src> {
    pub return_kw: Token<'src>,
    pub value_src: Option<SourceId>,
}

impl<'src> HasToken<'src> for ReturnSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.return_kw == token
    }
}

impl<'src> From<ReturnSource<'src>> for ExprSource<'src> {
    fn from(value: ReturnSource<'src>) -> Self {
        ExprSource::Return(value.into())
    }
}
