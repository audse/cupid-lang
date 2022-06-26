use crate::{
    Address,
    database::{
        selector::Selector,
        table::QueryTable,
    },
    environment::Env,
};

use self::symbol_table::{row::SymbolRow, query::Query};

pub mod db;
pub mod selector;
pub mod symbol_table;
pub mod source_table;
pub mod table;

impl QueryTable<Query<'_>, SymbolRow> for Env {
    fn insert<R: Into<SymbolRow>>(&mut self, query: R) -> Address {
        let address = self.database.insert::<R>(query);
        self.scope.set_symbol(address);
        address
    }
    fn read<'db: 'q, 'q, Col: Selector<SymbolRow>>(&'db self, query: &'q Query) -> Option<&'db Col> where SymbolRow: 'db {
        let row = self.database.read::<SymbolRow>(&query)?;
        if self.scope.is_in_scope(*Address::select(row)) {
            Some(Col::select(row))
        } else {
            None
        }
    }
    fn write(&mut self, query: Query) -> Option<()> {
        let address = *(self.database.read::<Address>(&query)?);
        if self.scope.is_in_scope(address) {
            self.database.write(query);
            Some(())
        } else {
            None
        }
    }
    fn take<'db: 'q, 'q, Col: Selector<SymbolRow> + Default>(&'db mut self, query: &'q Query<'_>) -> Option<Col> {
        let address = *(self.database.read::<Address>(query)?);
        if self.scope.is_in_scope(address) {
            self.database.take::<Col>(query)
        } else {
            None
        }
    }
}