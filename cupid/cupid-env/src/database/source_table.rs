use derive_more::{Deref, DerefMut};
use std::collections::BTreeMap;
use crate::{
    Address,
    database::{
        db::Database,
        selector::Selector,
        source_table::{
            query::Query,
            row::SourceRow,
        },
        table::{
            QueryTable, 
            TableRow
        },
    },
};

pub mod query;
pub mod row;

#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct SourceTable(BTreeMap<Address, SourceRow>);

impl QueryTable<Query, SourceRow> for SourceTable {
    fn insert<R: Into<SourceRow>>(& mut self, query: R) -> Address {
        let mut row: SourceRow = query.into();
        row.address = self.len();
        self.0.insert(row.address, row);
        self.len() - 1
    }
    fn read<'table: 'q, 'q, Col: Selector<SourceRow>>(&'table self, query: &'q Query) -> Option<&'table Col> {
        let (_, row) = self.iter().find(|(_, row)| row.matches_query(&query))?;
        Some(Col::select(row))
    }
    fn write(&mut self, query: Query) -> Option<()> {
        self.iter_mut().find(|(_, row)| row.matches_query(&query))?.1.unify(query);
        Some(())
    }
    fn take<'table: 'q, 'q, Col: Selector<SourceRow> + Default>(&'table mut self, query: &'q Query) -> Option<Col> {
        let (_, row) = self.iter_mut().find(|(_, row)| row.matches_query(&query))?;
        let col = Col::select_mut(row);
        Some(std::mem::take(col))
    }
}

impl QueryTable<Query, SourceRow> for Database {
    fn insert<R: Into<SourceRow>>(&mut self, query: R) -> Address {
        self.source_table.insert(query)
    }
    fn read<'table: 'q, 'q, Col: Selector<SourceRow>>(&'table self, query: &'q Query) -> Option<&'table Col> {
        self.source_table.read(query)
    }
    fn write(&mut self, query: Query) -> Option<()> {
        self.source_table.write(query)
    }
    fn take<'table: 'q, 'q, Col: Selector<SourceRow> + Default>(&'table mut self, query: &'q Query) -> Option<Col> {
        self.source_table.take(query)
    }
}