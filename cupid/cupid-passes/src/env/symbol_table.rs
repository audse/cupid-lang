use std::collections::BTreeMap;
use crate::{Address, PassExpr, Source};

/// The SymbolTable struct is a lighter version of the Env struct.
/// Each symbol that is set in Env will also have an associated address
/// in this table.
#[derive(Debug, Default, Clone)]
pub struct SymbolTable {
    pub symbols: BTreeMap<Address, PassExpr>,
    pub symbol_types: BTreeMap<Source, SymbolType>
}

impl SymbolTable {
    pub fn set_symbol(&mut self, address: Address, value: PassExpr) {
        self.symbols.insert(address, value);
    }
    pub fn get_symbol(&self, address: Address) -> Option<&PassExpr> {
        self.symbols.get(&address)
    }
    pub fn get_symbol_mut(&mut self, address: Address) -> Option<&mut PassExpr> {
        self.symbols.get_mut(&address)
    }
    pub fn set_type<T: Into<SymbolType>>(&mut self, source_node: Source, value: T) {
        self.symbol_types.insert(source_node, value.into());
    }
    pub fn get_type(&self, source_node: Source) -> Option<&SymbolType> {
        self.symbol_types.get(&source_node)
    }
}

#[derive(Debug, Clone)]
pub enum SymbolType {
    Type(crate::Type),
    Address(Address),
}