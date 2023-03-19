use super::{ExprSource, HasToken};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct GetSource<'src> {
    pub name: Token<'src>,
}

impl<'src> HasToken<'src> for GetSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.name == token
    }
}

impl<'src> From<GetSource<'src>> for ExprSource<'src> {
    fn from(value: GetSource<'src>) -> Self {
        ExprSource::Get(value.into())
    }
}
