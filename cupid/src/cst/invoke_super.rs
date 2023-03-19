use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct InvokeSuperSource<'src> {
    pub name: Token<'src>,
    pub open_paren: Token<'src>,
    pub close_paren: Token<'src>,
    pub args: Vec<SourceId>,
    pub commas: Vec<Token<'src>>,
}

impl<'src> HasToken<'src> for InvokeSuperSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.name == token
            || self.open_paren == token
            || self.close_paren == token
            || self.commas.contains(&token)
    }
}

impl<'src> From<InvokeSuperSource<'src>> for ExprSource<'src> {
    fn from(value: InvokeSuperSource<'src>) -> Self {
        ExprSource::InvokeSuper(value.into())
    }
}
