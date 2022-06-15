use std::collections::BTreeMap;
use crate::{Address, PassExpr, ErrCode, AsNode, Scope, Source};

#[derive(Debug, Default, Clone)]
pub enum Mut {
    Mutable,
    #[default]
    Immutable,
}

#[derive(Debug, Default, Clone)]
pub struct SymbolValue {
    value: Box<PassExpr>,
    mutable: Mut,
}

impl AsNode for SymbolValue {
    fn scope(&self) -> Scope {
        self.value.scope()
    }
    fn source(&self) -> Source {
        self.value.source()
    }
    fn typ(&self) -> Address {
        self.value.typ()
    }
	fn set_source(&mut self, source: Source) { self.value.set_source(source); }
	fn set_scope(&mut self, scope: Scope) { self.value.set_scope(scope); }
	fn set_typ(&mut self, typ: Address) { self.value.set_typ(typ); }
}

/// The SymbolTable struct is a lighter version of the Env struct.
/// Each symbol that is set in Env will also have an associated address
/// in this table.
#[derive(Debug, Default, Clone)]
pub struct SymbolTable {
    pub symbols: BTreeMap<Address, SymbolValue>,
}

impl SymbolTable {
    pub fn set_symbol(&mut self, address: Address, value: SymbolValue) {
        self.symbols.insert(address, value);
    }
    pub fn get_symbol(&self, address: Address) -> Option<&SymbolValue> {
        self.symbols.get(&address)
    }
    pub fn modify_symbol(&mut self, address: Address, mut modify: impl FnMut(&mut SymbolValue)) -> Result<(), ErrCode> {
        if let Some(value) = self.symbols.get_mut(&address) {
            modify(value);
            Ok(())
        } else {
            Err(cupid_util::ERR_NOT_FOUND)
        }
    }
}