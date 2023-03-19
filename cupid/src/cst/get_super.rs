use super::{ExprSource, HasToken};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct GetSuperSource<'src> {
    pub name: Token<'src>,
}

impl<'src> HasToken<'src> for GetSuperSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.name == token
    }
}

impl<'src> From<GetSuperSource<'src>> for ExprSource<'src> {
    fn from(value: GetSuperSource<'src>) -> Self {
        ExprSource::GetSuper(value.into())
    }
}
