use super::row::*;
use crate::{Address, Query};

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

    pub fn read<Col: Selector>(&self, query: &Query<()>) -> Option<&Col> { 
        let row = self.rows.iter().find(|row| row.matches_query(query))?;
        Some(Col::select(row))
    }

    pub fn write<V: Default>(&mut self, query: Query<V>) -> Option<()> 
        where RowExpr: WriteRowExpr<V> 
    {
        let row = self.rows.iter_mut().find(|row| row.matches_query(&query))?;
        row.unify(query);
        Some(())
    }

    pub fn take<Col: Selector + Default>(&mut self, query: &Query<()>) -> Option<Col> {
        self.rows.iter_mut()
            .find(|row| row.matches_query(&query))
            .map(|row| {
                let col = Col::select_mut(row);
                std::mem::take(col)
            })
    }
}
