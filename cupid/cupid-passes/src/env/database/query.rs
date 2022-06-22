use crate::{Address, Ident, Mut, env::SymbolType, Type};
use super::row::*;

#[derive(Default, Clone)]
pub struct FilterFn<'q>(pub Option<&'q dyn Fn(&Row) -> bool>);

#[derive(Debug, Default, Clone)]
pub struct Query<'q, V: Default> where RowExpr: WriteRowExpr<V> {
    pub read: ReadQuery<'q>,
    pub write: WriteQuery<'q, V>,
} // TODO add flags e.g. `SelectAll`

#[derive(Debug, Default, Clone)]
pub struct ReadQuery<'q> {
    pub address: Option<Address>,
    pub filter: FilterFn<'q>,
    pub ident: Option<Ident>,
    pub ident_ref: Option<&'q Ident>,
}

impl<'q> std::fmt::Debug for FilterFn<'q> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "filter function") }
}

#[derive(Debug, Default, Clone)]
pub struct WriteQuery<'q, V: Default> where RowExpr: WriteRowExpr<V> {
    pub expr: Option<V>,
    pub ident: Option<Ident>,
    pub ident_ref: Option<&'q Ident>,
    pub mutable: Option<Mut>,
    pub source: (), // TODO
    pub typ: Option<SymbolType>,
}

impl<'q, V: Default> Query<'q, V> where RowExpr: WriteRowExpr<V> {
    pub fn insert() -> Self {
        Self::default()
    }
    pub fn select(selector: impl Into<QuerySelector<'q>>) -> Self where {
        let mut query = Query::<'q, V>::default();
        match selector.into() {
            QuerySelector::Address(a) => query.read.address = Some(a),
            QuerySelector::Filter(i) => query.read.filter = FilterFn(Some(i)),
            QuerySelector::Ident(i) => query.read.ident = Some(i),
            QuerySelector::IdentRef(i) => query.read.ident_ref = Some(i),
            _ => ()
        }
        query
    }
    pub fn write(mut self, selector: impl Into<QuerySelector<'q>>) -> Self where {
        match selector.into() {
            QuerySelector::Ident(i) => self.write.ident = Some(i),
            QuerySelector::IdentRef(i) => self.write.ident_ref = Some(i),
            QuerySelector::Mutable(m) => self.write.mutable = Some(m),
            QuerySelector::Type(t) => self.write.typ = Some(t),
            _ => ()
        }
        self
    }
    pub fn write_expr(mut self, selector: impl Into<V>) -> Self {
        self.write.expr = Some(selector.into());
        self
    }
}

#[derive(Clone, derive_more::From)]
pub enum QuerySelector<'q> {
    Address(Address),
    Filter(&'q dyn Fn(&Row) -> bool),
    Ident(Ident),
    IdentRef(&'q Ident),
    Mutable(Mut),
    Type(SymbolType),
}

impl crate::AsNode for ReadQuery<'_> {
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

impl<V: Default> crate::AsNode for Query<'_, V> where RowExpr: WriteRowExpr<V> {
    fn address(&self) -> Address { self.read.address() }
    fn scope(&self) -> crate::ScopeId { self.read.scope() }
    fn set_scope(&mut self, scope: crate::ScopeId) { unreachable!() }
}

impl<'q> From<Type> for QuerySelector<'q> {
    fn from(typ: Type) -> Self { Self::Type(SymbolType::from(typ)) }
}