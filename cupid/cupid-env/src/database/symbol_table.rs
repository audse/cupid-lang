use derive_more::{Deref, DerefMut};
use std::collections::BTreeMap;
use crate::{
    Address,
    database::{
        db::Database,
        selector::Selector,
        symbol_table::{
            query::Query,
            row::SymbolRow,
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
pub struct SymbolTable(BTreeMap<Address, SymbolRow>);

impl QueryTable<Query<'_>, SymbolRow> for SymbolTable {

    fn insert<R: Into<SymbolRow>>(&mut self, query: R) -> Address {
        let mut row: SymbolRow = query.into();
        row.address = self.len();
        self.0.insert(row.address, row);
        self.len() - 1
    }

    fn read<'table: 'q, 'q, Col: Selector<SymbolRow>>(&'table self, query: &'q Query<'q>) -> Option<&'table Col> { 
        let (_, row) = self.iter().find(|(_, row)| row.matches_query(&query))?;
        Some(Col::select(row))
    }

    fn write(&mut self, query: Query) -> Option<()> {
        self.iter_mut().find(|(_, row)| row.matches_query(&query))?.1.unify(query);
        Some(())
    }

    fn take<'table: 'q, 'q, Col: Selector<SymbolRow> + Default>(&'table mut self, query: &'q Query<'q>) -> Option<Col> {
        let (_, row) = self.iter_mut().find(|(_, row)| row.matches_query(&query))?;
        let col = Col::select_mut(row);
        Some(std::mem::take(col))
    }
}

impl QueryTable<Query<'_>, SymbolRow> for Database {
    fn insert<R: Into<SymbolRow>>(&mut self, query: R) -> Address {
        self.symbol_table.insert(query)
    }
    fn read<'table: 'q, 'q, Col: Selector<SymbolRow>>(&'table self, query: &'q Query<'q>) -> Option<&'table Col> {
        self.symbol_table.read(query)
    }
    fn write(&mut self, query: Query) -> Option<()> {
        self.symbol_table.write(query)
    }
    fn take<'table: 'q, 'q, Col: Selector<SymbolRow> + Default>(&'table mut self, query: &'q Query<'q>) -> Option<Col> {
        self.symbol_table.take(query)
    }
}