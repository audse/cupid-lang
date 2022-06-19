use crate::{Address, env::Context, ScopeId};
use super::{closure::Closure, state::ScopeState};

#[derive(Debug, Clone)]
pub struct Scope {
    pub closures: Vec<Closure>,
    pub state: ScopeState,
}

impl Default for Scope {
    fn default() -> Self {
        Self { closures: vec![Closure::default()], state: ScopeState::default() }
    }
}

impl Scope {

    fn current_closure(&mut self) -> &mut Closure {
        &mut self.closures[self.state.current_closure]
    }

    pub fn add_toplevel_closure(&mut self, context: Context) -> ScopeId {
        increment_id(self, |env| {
            env.closures.push(Closure::new(0, env.state.id(), context))
        })
    }

    pub fn add_closure(&mut self, context: Context) -> ScopeId {
        increment_id(self, |env| {
            env.closures.push(Closure::new(env.state.closure(), env.state.id(), context))
        })
    }

    pub fn add_scope(&mut self, context: Context) -> ScopeId {
        increment_id(self, |env| {
            let id = env.state.id();
            env.current_closure().add(id, context);
        })
    }

    pub fn pop_scope(&mut self) {
        self.current_closure().pop();
    }

    pub fn is_in_scope(&self, address: Address) -> bool {
        self.closures[self.state.current_closure].is_in_scope(address)
    }

    pub fn set_symbol(&mut self, address: Address) {
        self.current_closure().set_symbol(address)
    }
}

/// Increases the current `ScopeId`, performs the given closure, and 
/// returns the incremented `ScopeId`
fn increment_id(scope: &mut Scope, closure: impl FnOnce(&mut Scope)) -> ScopeId {
    scope.state.current_id += 1;
    closure(scope);
    scope.state.current_id
}