use super::{Expr, ExprHeader, HasSymbol, Header, SourceId};
use crate::{arena::EntryId, pointer::Pointer, scope::symbol::Symbol, token::Token, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct Invoke<'src> {
        pub receiver: EntryId,
        pub callee: Token<'src>,
        pub args: Vec<EntryId>,
        pub symbol: Option<Pointer<Symbol<'src>>>
    }
}

pub struct InvokeSource<'src> {
    pub receiver: SourceId,
    pub dot: Token<'src>,
    pub callee: Token<'src>,
    pub open_paren: Token<'src>,
    pub close_paren: Token<'src>,
    pub commas: Vec<Token<'src>>,
    pub args: Vec<SourceId>,
}

impl<'src> HasSymbol<'src> for Invoke<'src> {
    fn symbol_token(&self) -> Token<'src> {
        self.callee
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

impl<'src> From<Invoke<'src>> for Expr<'src> {
    fn from(value: Invoke<'src>) -> Self {
        Expr::Invoke(value)
    }
}
