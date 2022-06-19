use crate::{Address, Ident, Mut, env::SymbolType};
use super::row::*;

#[derive(Debug, Default, Clone)]
pub struct Query<'q, V: Default> where RowExpr: WriteRowExpr<V> {
    pub address: Option<Address>,
    pub expr: Option<V>,
    pub ident: Option<Ident>,
    pub ident_ref: Option<&'q Ident>,
    pub mutable: Option<Mut>,
    pub source: (), // TODO
    pub typ: Option<SymbolType>,
}

impl<'q, V: Default> Query<'q, V> where RowExpr: WriteRowExpr<V> {
    pub fn build() -> Self {
        Self::default()
    }
    pub fn address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }
    pub fn expr<E: Into<V>>(mut self, expr: E) -> Self {
        self.expr = Some(expr.into());
        self
    }
    pub fn ident(mut self, ident: Ident) -> Self {
        self.ident = Some(ident);
        self
    }
    pub fn ident_ref(mut self, ident: &'q Ident) -> Self {
        self.ident_ref = Some(ident);
        self
    }
    pub fn mutable(mut self, mutable: Mut) -> Self {
        self.mutable = Some(mutable);
        self
    }
    pub fn typ<T: Into<SymbolType>>(mut self, typ: T) -> Self {
        self.typ = Some(typ.into());
        self
    }
    pub fn to_read_only(&self) -> Query<()> {
        Query::<()> { 
            address: self.address, 
            ident_ref: self.ident_ref.or(self.ident.as_ref()),
            ..Default::default() 
        }
    }
}

impl From<Ident> for Query<'_, ()> {
    fn from(ident: Ident) -> Self {
        Self::build().ident(ident)
    }
}

impl<'q> From<&'q Ident> for Query<'q, ()> {
    fn from(ident: &'q Ident) -> Self {
        Self::build().ident_ref(ident)
    }
}

impl From<Address> for Query<'_, ()> {
    fn from(address: Address) -> Self {
        Self::build().address(address)
    }
}

impl<V: Default> crate::AsNode for Query<'_, V> where RowExpr: WriteRowExpr<V> {
    fn address(&self) -> Address { 
        self.address
            .or(self.ident.as_ref().map(|i| i.address()))
            .or(self.ident_ref.map(|i| i.address()))
            .unwrap_or_default()
    }
    fn scope(&self) -> crate::ScopeId {
        self.ident.as_ref().map(|i| i.scope())
            .or(self.ident_ref.map(|i| i.address()))
            .unwrap_or_default()
    }
    fn set_scope(&mut self, scope: crate::ScopeId) {
        unreachable!()
    }
}