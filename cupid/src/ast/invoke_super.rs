use super::{Expr, ExprHeader, HasSymbol, Header};
use crate::{arena::EntryId, pointer::Pointer, scope::symbol::Symbol, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct InvokeSuper<'src> {
        pub name: &'src str,
        pub args: Vec<EntryId>,
        pub symbol: Option<Pointer<Symbol<'src>>>
    }
}

impl<'src> From<InvokeSuper<'src>> for Expr<'src> {
    fn from(value: InvokeSuper<'src>) -> Self {
        Expr::InvokeSuper(value.into())
    }
}

impl<'src> HasSymbol<'src> for InvokeSuper<'src> {
    fn symbol_name(&self) -> &'src str {
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
