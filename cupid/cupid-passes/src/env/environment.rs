use super::symbol_table::SymbolTable;

pub struct Env {
    pub symbols: SymbolTable,
    pub current_scope: crate::Scope,
    pub closures: Vec<Closure>
}

pub struct Closure {
    pub id: crate::Scope
}