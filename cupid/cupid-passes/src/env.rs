pub(super) mod closure;

pub mod database;

pub mod environment;
pub(crate) use environment::*;

pub mod query;

pub(super) mod state;

#[derive(Debug, Clone)]
pub enum SymbolType {
    Type(crate::Type),
    Address(crate::Address),
}

impl Default for SymbolType {
    fn default() -> Self {
        Self::Type(crate::Type::default())
    }
}