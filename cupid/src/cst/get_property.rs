use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct GetPropertySource<'src> {
    pub receiver: SourceId,
    pub property: Token<'src>,
    pub dot: Token<'src>,
}

impl<'src> HasToken<'src> for GetPropertySource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.property == token || self.dot == token
    }
}

impl<'src> From<GetPropertySource<'src>> for ExprSource<'src> {
    fn from(value: GetPropertySource<'src>) -> Self {
        ExprSource::GetProperty(value.into())
    }
}
