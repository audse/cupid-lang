use super::{Expr, ExprHeader, HasSymbol, Header};
use crate::{pointer::Pointer, scope::symbol::Symbol, token::Token, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Get<'src> {
        pub name: Token<'src>,
        pub symbol: Option<Pointer<Symbol<'src>>>
    }
}

impl<'src> HasSymbol<'src> for Get<'src> {
    fn symbol_token(&self) -> Token<'src> {
        self.name
    }
    fn symbol(&self) -> Option<&Pointer<Symbol<'src>>> {
        self.symbol.as_ref()
    }
    fn symbol_mut(&mut self) -> Option<&mut Pointer<Symbol<'src>>> {
        self.symbol.as_mut()
    }
    fn set_symbol(&mut self, symbol: Option<Pointer<Symbol<'src>>>) {
        self.symbol = symbol;
    }
}

pub struct GetSource<'src> {
    pub name: Token<'src>,
}

impl<'src> From<Get<'src>> for Expr<'src> {
    fn from(value: Get<'src>) -> Self {
        Expr::Get(value.into())
    }
}
