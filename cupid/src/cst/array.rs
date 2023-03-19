use crate::token::Token;

use super::{expr::ExprSource, HasToken, SourceId};

#[derive(Debug, Clone)]
pub struct ArraySource<'src> {
    pub open_bracket: Token<'src>,
    pub close_bracket: Token<'src>,
    pub commas: Vec<Token<'src>>,
    pub items_src: Vec<SourceId>,
}

impl<'src> HasToken<'src> for ArraySource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.open_bracket == token || self.close_bracket == token || self.commas.contains(&token)
    }
}

impl<'src> From<ArraySource<'src>> for ExprSource<'src> {
    fn from(value: ArraySource<'src>) -> Self {
        ExprSource::Array(value)
    }
}
