use cupid_ast::{expr::{Expr, ident::Ident}, stmt::decl::Mut};
use crate::{
    Address,
    database::{
        symbol_table::row::SymbolRow,
        selector::FilterFn,
    }
};

use super::row::Ref;

#[derive(Debug, Default, Clone)]
pub struct Query<'q> {
    pub read: ReadQuery<'q>,
    pub write: WriteQuery<'q>,
} // TODO add flags e.g. `SelectAll`

#[derive(Debug, Default, Clone)]
pub struct ReadQuery<'q> {
    pub address: Option<Address>,
    pub filter: FilterFn<'q, SymbolRow>,
    pub ident: Option<Ident>,
    pub ident_ref: Option<&'q Ident>,
}

#[derive(Debug, Default, Clone)]
pub struct WriteQuery<'q> {
    pub expr: Option<Expr>,
    pub ident: Option<Ident>,
    pub ident_ref: Option<&'q Ident>,
    pub mutable: Option<Mut>,
    pub refs: Ref,
}

impl<'q> Query<'q> {
    pub fn insert() -> Self {
        Self::default()
    }
    pub fn select(selector: impl Into<QuerySelector<'q>>) -> Self {
        let mut query = Query::<'q>::default();
        match selector.into() {
            QuerySelector::Address(a) => query.read.address = Some(a),
            QuerySelector::Filter(i) => query.read.filter = FilterFn(Some(i)),
            QuerySelector::Ident(i) => query.read.ident = Some(i),
            QuerySelector::IdentRef(i) => query.read.ident_ref = Some(i),
            _ => ()
        }
        query
    }
    pub fn write(mut self, selector: impl Into<QuerySelector<'q>>) -> Self {
        match selector.into() {
            QuerySelector::Expr(e) => self.write.expr = Some(e),
            QuerySelector::Ident(i) => self.write.ident = Some(i),
            QuerySelector::IdentRef(i) => self.write.ident = Some(i.clone()),
            QuerySelector::Mutable(m) => self.write.mutable = Some(m),
            QuerySelector::Ref(r) => self.write.refs = r,
            _ => ()
        }
        self
    }
}

#[derive(Clone, derive_more::From)]
pub enum QuerySelector<'q> {
    Address(Address),
    Expr(Expr),
    Filter(&'q dyn Fn(&SymbolRow) -> bool),
    Ident(Ident),
    IdentRef(&'q Ident),
    Mutable(Mut),
    Ref(Ref)
}

impl From<Query<'_>> for SymbolRow {
    fn from(q: Query) -> Self {
        let Query { read, write, ..} = q;
        SymbolRow {
            address: read.address.unwrap_or_default(),
            expr: write.expr.unwrap_or_default(),
            ident: write.ident.or_else(|| read.ident).or_else(|| read.ident_ref.cloned()).unwrap_or_default(),
            mutable: write.mutable.unwrap_or_default(),
            refs: Ref(0)
        }
    }
}