use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct LoopSource<'src> {
    pub loop_kw: Token<'src>,
    pub body_src: SourceId,
}

impl<'src> HasToken<'src> for LoopSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.loop_kw == token
    }
}

impl<'src> From<LoopSource<'src>> for ExprSource<'src> {
    fn from(value: LoopSource<'src>) -> Self {
        ExprSource::Loop(value.into())
    }
}
