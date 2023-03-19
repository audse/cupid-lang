use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct DefineSource<'src> {
    pub let_kw: Option<Token<'src>>,
    pub equal: Option<Token<'src>>,
    pub name: Token<'src>,
    pub value_src: Option<SourceId>,
}

impl<'src> HasToken<'src> for DefineSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.let_kw == Some(token) || self.name == token || self.equal == Some(token)
    }
}

impl<'src> From<DefineSource<'src>> for ExprSource<'src> {
    fn from(value: DefineSource<'src>) -> Self {
        ExprSource::Define(value)
    }
}
