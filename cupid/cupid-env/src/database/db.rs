use crate:: database::{
    source_table::SourceTable,
    symbol_table::SymbolTable,
};

#[derive(Debug, Default, Clone)]
pub struct Database {
    pub source_table: SourceTable,
    pub symbol_table: SymbolTable,
}
