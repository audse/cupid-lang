use super::row::*;
use crate::{Address, ReadQuery, WriteQuery, Query};

#[derive(Debug, Default, Clone)]
pub struct Database {
    rows: Vec<Row>,
}

impl Database {

    pub fn insert<V: Default>(&mut self, query: Query<V>) -> Address 
        where RowExpr: WriteRowExpr<V> 
    {
        let mut row: Row = query.into();
        row.address = self.rows.len();
        self.rows.push(row);
        self.rows.len() - 1
    }

    pub fn read<'db: 'q, 'q, Col: Selector>(&'db self, query: &'q ReadQuery<'q>) -> Option<&'db Col> { 
        let row = self.rows.iter().find(|row| row.matches_query(&query))?;
        Some(Col::select(row))
    }

    pub fn index<'db>(&'db mut self, query: &'db ReadQuery<'db>) -> Option<usize> {
        self.rows.iter_mut().position(|row| row.matches_query(query))
    }

    pub fn write<'db, V: Default>(&'db mut self, read_query: ReadQuery<'db>, write_query: WriteQuery<V>) -> Option<()> 
        where RowExpr: WriteRowExpr<V> 
    {
        self.rows.iter_mut().find(|row| row.matches_query(&read_query))?.unify(write_query);
        Some(())
    }

    pub fn take<'db, Col: Selector + Default>(&'db mut self, query: &'db ReadQuery<'db>) -> Option<Col> {
        let row = self.rows.iter_mut().find(|row| row.matches_query(&query))?;
        let col = Col::select_mut(row);
        Some(std::mem::take(col))
    }
}
