use cupid_util::{ERR_NOT_FOUND, ERR_NOT_IN_SCOPE};

use super::state::EnvState;
use crate::PassResult;
use super::{closure::*, database::{Database, Row, RowExpr, Selector, WriteRowExpr}, query::Query};

pub type Source = usize;
pub type Address = usize;
pub type ScopeId = usize;

#[derive(Debug, Default, Copy, Clone)]
pub enum Mut {
    Mutable,
    #[default]
    Immutable,
}

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
    pub state: EnvState,
    pub database: Database,
    pub closures: Vec<Closure>,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            state: EnvState::default(),
            database: Database::default(),
            closures: vec![Closure::default()] 
        }
    }
}

impl Env {
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
    pub fn inside_closure<R>(&mut self, closure: ScopeId, fun: impl FnOnce(&mut Env) -> PassResult<R>) -> PassResult<R> {
        self.state.use_closure(closure);
        let result = fun(self)?;
        self.state.reset_closure();
        Ok(result)
    }

    pub fn insert<V: Default>(&mut self, query: Query<V>) -> Address where RowExpr: WriteRowExpr<V> {
        let address = self.database.insert(query);
        self.current_closure().set_symbol(address);
        address
    }

    pub fn read<Col: Selector>(&self, query: &Query<()>) -> Option<&Col> { 
        let row = self.database.read::<Row>(query)?;
        if self.is_in_scope(row.address) {
            Some(Col::select(row))
        } else {
            None
        }
    }

    pub fn write<V: Default>(&mut self, query: Query<V>) -> Option<()> where RowExpr: WriteRowExpr<V> { 
        let address = *self.database.read::<Address>(&query.to_read_only())?;
        if self.is_in_scope(address) {
            self.database.write(query)
        } else {
            None
        }
    }

    pub fn write_pass<V: Default, F: FnOnce(&mut Env, RowExpr) -> crate::PassResult<RowExpr>>(&mut self, query: Query<V>, closure: F) -> crate::PassResult<()> where RowExpr: WriteRowExpr<V> {
        let address = *self.database.read::<Address>(&query.to_read_only()).ok_or((0, ERR_NOT_FOUND))?;
        if self.is_in_scope(address) {
            let expr = self.database.take::<RowExpr>(&query.to_read_only()).unwrap();
            let new_expr = closure(self, expr)?;
            self.database.write::<RowExpr>(Query::<RowExpr>::build().address(address).expr(new_expr));            
            Ok(())
        } else {
            Err((0, ERR_NOT_IN_SCOPE))
        }
    }

    fn is_in_scope(&self, address: Address) -> bool {
        self.closures[self.state.current_closure].is_in_scope(address)
    }
}

/// Increases the current `ScopeId`, performs the given closure, and 
/// returns the incremented `ScopeId`
fn increment_id(env: &mut Env, closure: impl FnOnce(&mut Env)) -> ScopeId {
    env.state.current_id += 1;
    closure(env);
    env.state.current_id
}