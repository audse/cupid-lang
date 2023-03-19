use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct SetPropertySource<'src> {
    pub receiver: SourceId,
    pub dot: Token<'src>,
    pub property: Token<'src>,
    pub equal: Token<'src>,
    pub value: SourceId,
}

impl<'src> HasToken<'src> for SetPropertySource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.property == token || self.dot == token || self.equal == token
    }
}

impl<'src> From<SetPropertySource<'src>> for ExprSource<'src> {
    fn from(value: SetPropertySource<'src>) -> Self {
        ExprSource::SetProperty(value.into())
    }
}
