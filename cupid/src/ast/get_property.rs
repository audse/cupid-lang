use super::{Expr, ExprHeader, HasSymbol, Header, SourceId};
use crate::{arena::EntryId, pointer::Pointer, scope::symbol::Symbol, token::Token, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct GetProperty<'src> {
        pub receiver: EntryId,
        pub property: Token<'src>,
        pub symbol: Option<Pointer<Symbol<'src>>>,
    }
}

pub struct GetPropertySource<'src> {
    pub receiver: SourceId,
    pub property: Token<'src>,
    pub dot: Token<'src>,
}

impl<'src> HasSymbol<'src> for GetProperty<'src> {
    fn symbol_token(&self) -> Token<'src> {
        self.property
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

impl<'src> From<GetProperty<'src>> for Expr<'src> {
    fn from(value: GetProperty<'src>) -> Self {
        Expr::GetProperty(value.into())
    }
}
