use cupid_lex::token::Token;
use cupid_ast::expr::ident::Ident;
use crate::{
    Address,
    database::source_table::row::SourceRow,
};

#[derive(Debug, Default, Clone)]
pub struct WriteQuery {
    pub source: Option<Vec<Token<'static>>>,
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