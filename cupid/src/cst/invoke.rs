use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct InvokeSource<'src> {
    pub receiver: SourceId,
    pub dot: Token<'src>,
    pub callee: Token<'src>,
    pub open_paren: Token<'src>,
    pub close_paren: Token<'src>,
    pub commas: Vec<Token<'src>>,
    pub args: Vec<SourceId>,
}

impl<'src> HasToken<'src> for InvokeSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.callee == token
            || self.dot == token
            || self.open_paren == token
            || self.close_paren == token
            || self.commas.contains(&token)
    }
}

impl<'src> From<InvokeSource<'src>> for ExprSource<'src> {
    fn from(value: InvokeSource<'src>) -> Self {
        ExprSource::Invoke(value)
    }
}
