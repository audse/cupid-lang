use std::{collections::HashMap, hash::Hash};
use super::symbol_table::{SymbolTable, SymbolValue};
use cupid_util::ERR_NOT_FOUND;
use crate::{Ident, AsNode};

pub type Source = usize;
pub type Address = usize;
pub type ScopeId = usize;

#[derive(Debug, Default, Copy, Clone)]
pub enum Context {
    #[default]
    Block,
    Function,
    Loop,
    Trait,
    Type,
}

#[derive(Debug, Clone)]
pub struct Env {
    current_id: ScopeId,
    pub symbols: SymbolTable,
    pub current_closure: ScopeId,
    prev_closures: Vec<ScopeId>,
    pub closures: Vec<Closure<Ident>>,
}

impl Default for Env {
    fn default() -> Self {
        Self { 
            current_id: 0,
            symbols: SymbolTable::default(),
            current_closure: 0, 
            prev_closures: vec![0],
            closures: vec![Closure::default()] 
        }
    }
}

impl Env {
    pub fn add_toplevel_closure(&mut self, context: Context) -> ScopeId {
        self.current_id += 1;
        self.closures.push(Closure { parent: 0, id: self.current_id, context, ..Default::default() });
        self.current_id
    }
    pub fn add_closure(&mut self, context: Context) -> ScopeId {
        self.current_id += 1;
        let parent = self.current_closure;
        self.closures.push(Closure { parent, id: self.current_id, context, ..Default::default() });
        self.current_id
    }
    pub fn add_scope(&mut self, context: Context) -> ScopeId {
        self.current_id += 1;
        let current_closure = &mut self.closures[self.current_closure];
        self.closures.last_mut().unwrap().add(self.current_id, context);
        self.current_id
    }
    pub fn pop_scope(&mut self) {
        let current_closure = &mut self.closures[self.current_closure];
        self.closures.last_mut().unwrap().pop();
    }
    pub fn use_scope(&mut self, closure: ScopeId) {
        self.prev_closures.push(self.current_closure);
        self.current_closure = closure;
    }
    pub fn inside_scope<R, F: FnOnce(&mut Self) -> crate::PassResult<R>>(&mut self, closure: ScopeId, fun: F) -> crate::PassResult<R> {
        self.use_scope(closure);
        let result = fun(self)?;
        self.reset_scope();
        Ok(result)
    }
    pub fn reset_scope(&mut self) {
        let prev_closure = self.prev_closures.pop();
        self.current_closure = prev_closure.unwrap_or_default();
    }
    pub fn set_symbol(&mut self, symbol: Ident, value: SymbolValue) {
        let current_closure = &mut self.closures[self.current_closure];
        let address = self.symbols.symbols.len();
        self.symbols.set_symbol(address, value);
        current_closure.set_symbol(symbol, address);
    }
    pub fn get_symbol(&mut self, symbol: &Ident) -> crate::PassResult<Address> {
        let current_closure = &mut self.closures[self.current_closure];
        if let Some(address) = current_closure.get_symbol(symbol) {
            Ok(address)
        } else {
            Err((symbol.source(), ERR_NOT_FOUND))
        }
    }
}

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
    fn add(&mut self, id: ScopeId, context: Context) {
        self.scopes.push(BlockScope { id, context, ..Default::default() });
    }
    fn pop(&mut self) {
        self.scopes.pop();
    }
    fn set_symbol(&mut self, symbol: K, address: Address) {
        let mut current_scope = None;
        for scope in self.scopes.iter_mut() {
            if scope.get_symbol(&symbol).is_some() {
                current_scope = Some(scope);
            }
        }
        if let Some(scope) = current_scope {
            scope.set_symbol(symbol, address);
        } else {
            self.scopes.last_mut().unwrap().set_symbol(symbol, address);
        }
    }
    fn get_symbol(&mut self, symbol: &K) -> Option<Address> {
        for scope in self.scopes.iter_mut() {
            if let Some(address) = scope.get_symbol(symbol) {
                return Some(address)
            }
        }
        None
    }
}

#[derive(Debug, Default, Clone)]
pub struct BlockScope<K: Default + Hash + Eq> {
    pub id: ScopeId,
    pub context: Context,
    pub symbols: HashMap<K, Address>
}

impl<K: Default + Hash + Eq> BlockScope<K> {
    fn set_symbol(&mut self, symbol: K, address: Address) {
        self.symbols.insert(symbol, address);
    }
    fn get_symbol(&mut self, symbol: &K) -> Option<Address> {
        self.symbols.get(symbol).copied()
    }
}