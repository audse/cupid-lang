use super::{Expr, ExprHeader, HasSymbol, Header, SourceId};
use crate::{arena::EntryId, pointer::Pointer, scope::symbol::Symbol, token::Token, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Set<'src> {
        pub name: Token<'src>,
        pub value: EntryId,
        pub symbol: Option<Pointer<Symbol<'src>>>,
    }
}

pub struct SetSource<'src> {
    pub name: Token<'src>,
    pub value: SourceId,
    pub equal: Token<'src>,
}

impl<'src> HasSymbol<'src> for Set<'src> {
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

impl<'src> From<Set<'src>> for Expr<'src> {
    fn from(value: Set<'src>) -> Self {
        Expr::Set(value.into())
    }
}
