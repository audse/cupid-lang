use crate::{
    Address,
    database::{
        db::Database,
        selector::Selector,
        table::QueryTable,
    },
    environment::Env,
};

pub mod db;
pub mod selector;
pub mod symbol_table;
pub mod source_table;
pub mod table;

impl<Q, Row: QueryTable<Q, Row>> QueryTable<Q, Row> for Env 
where 
    Database: QueryTable<Q, Row>,  
    Row: Selector<Row>, Address: Selector<Row> 
{
    fn insert<R: Into<Row>>(&mut self, query: R) -> Address {
        let address = self.database.insert::<R>(query);
        self.scope.set_symbol(address);
        address
    }
    fn read<'db: 'q, 'q, Col: Selector<Row>>(&'db self, query: &'q Q) -> Option<&'db Col> where Row: 'db {
        let row = self.database.read::<Row>(&query)?;
        if self.scope.is_in_scope(*Address::select(row)) {
            Some(Col::select(row))
        } else {
            None
        }
    }
    fn write(&mut self, query: Q) -> Option<()> {
        let address = *(self.database.read::<Address>(&query)?);
        if self.scope.is_in_scope(address) {
            self.database.write(query);
            Some(())
        } else {
            None
        }
    }
}