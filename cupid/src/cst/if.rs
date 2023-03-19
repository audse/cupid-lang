use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct IfSource<'src> {
    pub if_kw: Token<'src>,
    pub condition_src: SourceId,
    pub body_src: SourceId,
    pub else_kw: Option<Token<'src>>,
    pub else_body_src: Option<SourceId>,
}

impl<'src> HasToken<'src> for IfSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.if_kw == token || self.else_kw == Some(token)
    }
}

impl<'src> From<IfSource<'src>> for ExprSource<'src> {
    fn from(value: IfSource<'src>) -> Self {
        ExprSource::If(value.into())
    }
}
