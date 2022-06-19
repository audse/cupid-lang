pub mod database;

pub mod environment;
pub(crate) use environment::*;

pub mod scope;

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