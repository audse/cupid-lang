use crate::{Address, environment::Context, ScopeId};

#[derive(Debug, Clone)]
pub struct Closure {
    pub parent: ScopeId,
    pub id: ScopeId,
    pub context: Context,
    pub scopes: Vec<BlockScope>,
}

impl Default for Closure {
    fn default() -> Self {
        Self { 
            parent: 0, 
            id: 0, 
            context: Context::Block, 
            scopes: vec![BlockScope::default()] 
        }
    }
}

impl Closure {

    pub(super) fn new(parent: ScopeId, id: ScopeId, context: Context) -> Self {
        Self { parent, id, context, ..Default::default() }
    }

    pub(super) fn add(&mut self, id: ScopeId, context: Context) {
        self.scopes.push(BlockScope { id, context, ..Default::default() });
    }

    pub(super) fn pop(&mut self) {
        self.scopes.pop();
    }

    pub(super) fn set_symbol(&mut self, address: Address) {
        let scope = if let Some(scope) = self.in_scope(address) {
            scope
        } else {
            self.scopes.last_mut().unwrap()
        };
        scope.set_symbol(address)
    }

    fn in_scope(&mut self, address: Address) -> Option<&mut BlockScope> {
        self.scopes.iter_mut().find(|scope| scope.in_scope(address))
    }

    pub(super) fn is_in_scope(&self, address: Address) -> bool {
        self.scopes.iter().any(|scope| scope.in_scope(address))
    }
}

#[derive(Debug, Default, Clone)]
pub struct BlockScope {
    pub id: ScopeId,
    pub context: Context,
    pub symbols: Vec<Address>
}

impl BlockScope {

    fn set_symbol(&mut self, address: Address) {
        self.symbols.push(address);
    }

    fn in_scope(&self, address: Address) -> bool {
        self.symbols.iter().any(|a| *a == address)
    }
    
}