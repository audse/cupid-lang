use std::collections::BTreeMap;
use crate::*;

pub type Address = usize;

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
            Err(ERR_NOT_FOUND)
        }
    }
	pub fn update_closure(&mut self, address: Address, closure: usize) -> Result<(), ErrCode> {
        self.modify_symbol(address, |val| val.attributes_mut().closure = closure)
	}
}