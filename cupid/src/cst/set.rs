use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct SetSource<'src> {
    pub name: Token<'src>,
    pub value_src: SourceId,
    pub equal: Token<'src>,
}

impl<'src> HasToken<'src> for SetSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.name == token || self.equal == token
    }
}

impl<'src> From<SetSource<'src>> for ExprSource<'src> {
    fn from(value: SetSource<'src>) -> Self {
        ExprSource::Set(value.into())
    }
}
