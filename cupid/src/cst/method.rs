use super::{fun::FunSource, HasToken};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct MethodSource<'src> {
    pub name: Token<'src>,
    pub fun: FunSource<'src>,
}

impl<'src> HasToken<'src> for MethodSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.name == token || self.fun.has_token(token)
    }
}
