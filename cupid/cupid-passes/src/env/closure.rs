use std::{hash::Hash, collections::HashMap};
use crate::{Address, env::Context, ScopeId, Mut};

#[derive(Debug, Clone)]
pub struct Closure<K: Default + Hash + Eq> {
    pub parent: ScopeId,
    pub id: ScopeId,
    pub context: Context,
    pub scopes: Vec<BlockScope<K>>,
}

impl<K: Default + Hash + Eq> Default for Closure<K> {
    fn default() -> Self {
        Self { 
            parent: 0, 
            id: 0, 
            context: Context::Block, 
            scopes: vec![BlockScope::default()] 
        }
    }
}

impl<K: Default + Hash + Eq> Closure<K> {
    pub(super) fn new(parent: ScopeId, id: ScopeId, context: Context) -> Self {
        Self { parent, id, context, ..Default::default() }
    }
    pub(super) fn add(&mut self, id: ScopeId, context: Context) {
        self.scopes.push(BlockScope { id, context, ..Default::default() });
    }
    pub(super) fn pop(&mut self) {
        self.scopes.pop();
    }
    pub(super) fn set_symbol(&mut self, symbol: K, address: Address, mutable: Mut) {
        let mut current_scope = None;
        for scope in self.scopes.iter_mut() {
            if scope.get_symbol(&symbol).is_some() {
                current_scope = Some(scope);
            }
        }
        if let Some(scope) = current_scope {
            scope.set_symbol(symbol, address, mutable);
        } else {
            self.scopes.last_mut().unwrap().set_symbol(symbol, address, mutable);
        }
    }
    pub(super) fn get_symbol(&mut self, symbol: &K) -> Option<(Address, Mut)> {
        for scope in self.scopes.iter_mut() {
            if let Some(address) = scope.get_symbol(symbol) {
                return Some(address)
            }
        }
        None
    }
    pub(super) fn get_ident(&mut self, address: Address) -> Option<&K> {
        for scope in self.scopes.iter_mut() {
            if let Some(ident) = scope.get_ident(address) {
                return Some(ident)
            }
        }
        None
    }
}

#[derive(Debug, Default, Clone)]
pub struct BlockScope<K: Default + Hash + Eq> {
    pub id: ScopeId,
    pub context: Context,
    pub symbols: HashMap<K, (Address, Mut)>
}

impl<K: Default + Hash + Eq> BlockScope<K> {
    fn set_symbol(&mut self, symbol: K, address: Address, mutable: Mut) {
        self.symbols.insert(symbol, (address, mutable));
    }
    fn get_symbol(&mut self, symbol: &K) -> Option<(Address, Mut)> {
        self.symbols.get(symbol).copied()
    }
    fn get_ident(&mut self, address: Address) -> Option<&K> {
        self.symbols
            .iter()
            .find_map(|(ident, symbol_address)| if symbol_address.0 == address { 
                Some(ident) 
            } else { 
                None 
            })
    }
}