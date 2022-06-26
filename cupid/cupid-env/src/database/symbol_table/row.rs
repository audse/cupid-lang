use derive_more::{Add, AddAssign};
use cupid_ast::{expr::{Expr, ident::Ident}, stmt::decl::Mut};
use crate::{
    Address,
    database::{
        selector::{
            FilterFn,
            Selector
        },
        symbol_table::query::{
            Query,
            WriteQuery,
        },
        table::TableRow,
    }
};


#[derive(Debug, Default, Clone)]
pub struct SymbolRow {
    pub address: Address,
    pub ident: Ident,
    pub mutable: Mut,
    pub expr: Expr,
    pub refs: Ref,
}

#[derive(Debug, Default, Clone, Copy, Add, AddAssign)]
pub struct Ref(pub usize);

impl<'row: 'q, 'q> TableRow<'row, 'q, Query<'q>> for SymbolRow {
    fn matches_query(&'row self, query: &'q Query<'q>) -> bool {
        self.select_by(query.read.address.as_ref())
            .or_else(|| self.filter_by(&query.read.filter))
            .or_else(|| self.select_by(query.read.ident.as_ref()))
            .or_else(|| self.select_by(query.read.ident_ref))
            .is_some()
    }
    fn filter_by(&'row self, selector: &FilterFn<Self>) -> Option<&'row Self> { 
        if let Some(selector) = selector.0 {
            if selector(&self) { return Some(self) }
        }
        None
    }
    fn select_by<Col: Selector<Self> + PartialEq>(&self, selector: Option<&Col>) -> Option<&Self> { 
        if let Some(selector) = selector {
            if Col::select(self) == selector { return Some(self) }
        }
        None
    }
    fn unify(&mut self, query: Query) {
        let WriteQuery { expr, ident, ident_ref, mutable, refs, ..} = query.write;

        expr.map(|e| self.expr = e);
        ident.map(|i| self.ident = i);
        ident_ref.map(|i| self.ident = i.to_owned());
        mutable.map(|m| self.mutable = m);
        self.refs += refs;
    }
}
