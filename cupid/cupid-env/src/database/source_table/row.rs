use std::rc::Rc;

use cupid_ast::expr::ident::Ident;
use cupid_debug::source::ExprSource;

use crate::{
    Address,
    database::{
        source_table::query::{
            Query, 
            WriteQuery
        },
        selector::{
            FilterFn,
            Selector,
        },
        table::TableRow,
    },
};

#[derive(Debug, Default, Clone)]
pub struct SourceRow {
    pub address: Address,
    pub source: Rc<ExprSource>,
    pub typ: Ident,
}

impl<'row: 'q, 'q> TableRow<'row, 'q, Query> for SourceRow {
    fn matches_query(&'row self, query: &'q Query) -> bool {
        self.select_by(Some(&query.read)).is_some()
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
        let WriteQuery { source, typ, ..} = query.write;
        source.map(|s| self.source = s);
        typ.map(|t| self.typ = t);
    }
}