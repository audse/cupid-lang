use super::{ExprSource, HasToken};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct ConstantSource<'src> {
    pub value: Token<'src>,
}

impl<'src> HasToken<'src> for ConstantSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        self.value == token
    }
}

impl<'src> From<ConstantSource<'src>> for ExprSource<'src> {
    fn from(value: ConstantSource<'src>) -> Self {
        ExprSource::Constant(value)
    }
}
