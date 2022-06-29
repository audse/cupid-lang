use std::rc::Rc;

use derive_more::{From, TryInto};
use cupid_ast::expr::ident::Ident;
use cupid_debug::source::ExprSource;
use crate::{
    Address,
    database::source_table::row::SourceRow,
};

#[derive(Debug, Default, Clone)]
pub struct WriteQuery {
    pub source: Option<Rc<ExprSource>>,
    pub typ: Option<Ident>,
}

#[derive(Debug, Default, Clone)]
pub struct Query {
    pub read: Address,
    pub write: WriteQuery,
}

impl Query {
    pub fn insert() -> Self {
        Self::default()
    }
    pub fn select(address: Address) -> Self {
        Self {
            read: address,
            write: WriteQuery::default()
        }
    }
    pub fn write(mut self, selector: impl Into<QuerySelector>) -> Self {
        match selector.into() {
            QuerySelector::Type(typ) => self.write.typ = Some(typ),
            QuerySelector::Source(source) => self.write.source = Some(source),
        }
        self
    }
}

impl From<Query> for SourceRow {
    fn from(q: Query) -> Self {
        let Query { write, ..} = q;
        SourceRow {
            source: write.source.unwrap_or_default(),
            typ: write.typ.unwrap_or_default(),
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, From, TryInto)]
pub enum QuerySelector {
    Type(Ident),
    Source(Rc<ExprSource>),
}