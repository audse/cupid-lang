use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct CallSource<'src> {
    pub open_paren: Token<'src>,
    pub close_paren: Token<'src>,
    pub callee_src: SourceId,
    pub args_src: Vec<SourceId>,
    pub commas: Vec<Token<'src>>,
}

impl<'src> HasToken<'src> for CallSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.open_paren == token || self.close_paren == token || self.commas.contains(&token)
    }
}

impl<'src> From<CallSource<'src>> for ExprSource<'src> {
    fn from(value: CallSource<'src>) -> Self {
        ExprSource::Call(value)
    }
}
