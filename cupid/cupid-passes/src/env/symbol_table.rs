use std::collections::BTreeMap;
use crate::{Address, PassExpr, AsNode, ScopeId, Source};

#[derive(Debug, Default, Clone)]
pub enum Mut {
    Mutable,
    #[default]
    Immutable,
}

#[derive(Debug, Default, Clone)]
pub struct SymbolValue {
    pub value: Box<PassExpr>,
    pub mutable: Mut,
}

impl AsNode for SymbolValue {
    fn scope(&self) -> ScopeId {
        self.value.scope()
    }
    fn source(&self) -> Source {
        self.value.source()
    }
	fn set_source(&mut self, source: Source) { self.value.set_source(source); }
	fn set_scope(&mut self, scope: ScopeId) { self.value.set_scope(scope); }
}

#[derive(Debug, Default, Clone)]
pub enum SymbolTyp {
    Typ(crate::Typ),
    Address(Address),
    #[default]
    None,
}

/// The SymbolTable struct is a lighter version of the Env struct.
/// Each symbol that is set in Env will also have an associated address
/// in this table.
#[derive(Debug, Default, Clone)]
pub struct SymbolTable {
    pub symbols: BTreeMap<Address, SymbolValue>,
    pub symbol_types: BTreeMap<Source, SymbolTyp>
}

impl SymbolTable {
    pub fn set_symbol(&mut self, address: Address, value: SymbolValue) {
        self.symbols.insert(address, value);
    }
    pub fn get_symbol(&self, address: Address) -> Option<&SymbolValue> {
        self.symbols.get(&address)
    }
    pub fn get_symbol_mut(&mut self, address: Address) -> Option<&mut SymbolValue> {
        self.symbols.get_mut(&address)
    }
    pub fn set_typ<T: Into<SymbolTyp>>(&mut self, source_node: Source, value: T) {
        self.symbol_types.insert(source_node, value.into());
    }
    pub fn get_typ(&self, source_node: Source) -> Option<&SymbolTyp> {
        self.symbol_types.get(&source_node)
    }
}

impl From<Address> for SymbolTyp {
    fn from(a: Address) -> Self {
        Self::Address(a)
    }
}

impl From<crate::Typ> for SymbolTyp {
    fn from(t: crate::Typ) -> Self {
        Self::Typ(t)
    }
}