use crate::{
    error::CupidError,
    token::{Token, TokenType},
};

pub trait Iter<'src> {
    fn next(&mut self) -> Token<'src>;
    fn advance(&mut self) -> Token<'src>;
    fn expect(
        &mut self,
        kind: TokenType,
        msg: impl std::fmt::Display,
    ) -> Result<Token<'src>, CupidError>;
    fn matches_any(&mut self, kinds: &[TokenType]) -> Option<Token<'src>>;
    fn matches(&mut self, kind: TokenType) -> Option<Token<'src>>;
    fn check_any(&mut self, kinds: &[TokenType]) -> bool;
    fn check(&mut self, kind: TokenType) -> bool;
}
