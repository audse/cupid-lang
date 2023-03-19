use super::{Expr, ExprHeader, HasSymbol, Header};
use crate::{arena::EntryId, pointer::Pointer, scope::symbol::Symbol, with_header};

with_header! {
    #[derive(Debug, Clone)]
    pub struct SetProperty<'src> {
        pub receiver: EntryId,
        pub property: &'src str,
        pub value: EntryId,
        pub symbol: Option<Pointer<Symbol<'src>>>,
    }
}

impl<'src> HasSymbol<'src> for SetProperty<'src> {
    fn symbol_name(&self) -> &'src str {
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

impl<'src> From<SetProperty<'src>> for Expr<'src> {
    fn from(value: SetProperty<'src>) -> Self {
        Expr::SetProperty(value.into())
    }
}
