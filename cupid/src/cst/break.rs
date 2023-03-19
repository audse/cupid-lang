use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct BreakSource<'src> {
    pub break_kw: Token<'src>,
    pub value_src: Option<SourceId>,
}

impl<'src> HasToken<'src> for BreakSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.break_kw == token
    }
}

impl<'src> From<BreakSource<'src>> for ExprSource<'src> {
    fn from(value: BreakSource<'src>) -> Self {
        ExprSource::Break(value.into())
    }
}
