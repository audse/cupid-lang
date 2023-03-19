use super::{ExprSource, HasToken, SourceId};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct FunSource<'src> {
    pub fun_kw: Token<'src>,
    pub name: Option<Token<'src>>,
    pub params_src: Vec<SourceId>,
    pub commas: Vec<Token<'src>>,
    pub body_src: SourceId,
    pub open_paren: Token<'src>,
    pub close_paren: Token<'src>,
}

impl<'src> HasToken<'src> for FunSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.fun_kw == token
            || self.name == Some(token)
            || self.open_paren == token
            || self.close_paren == token
            || self.commas.contains(&token)
    }
}

impl<'src> From<FunSource<'src>> for ExprSource<'src> {
    fn from(value: FunSource<'src>) -> Self {
        ExprSource::Fun(value)
    }
}
