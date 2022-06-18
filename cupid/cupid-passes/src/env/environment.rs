use super::{symbol_table::{SymbolTable, SymbolType}, state::EnvState};
use cupid_util::ERR_NOT_FOUND;
use crate::{Ident, AsNode, PassExpr, PassResult, Type, Value};
use super::closure::*;

pub type Source = usize;
pub type Address = usize;
pub type ScopeId = usize;

type ModifyFn = fn(&mut Env, PassExpr) -> PassResult<PassExpr>;

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
    pub symbols: SymbolTable,
    pub closures: Vec<Closure<Ident>>,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            state: EnvState::default(),
            symbols: SymbolTable::default(),
            closures: vec![Closure::default()] 
        }
    }
}

impl Env {
    fn current_closure(&mut self) -> &mut Closure<Ident> {
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
    pub fn set_symbol(&mut self, symbol: Ident, mut value: PassExpr, mutable: Mut) -> Address {
        let address = self.symbols.symbols.len();

        // FOR TESTING ONLY, type indices are messed up when nodes don't have sourced data
        if value.source() == 0 { value.set_source(address) }

        self.symbols.set_symbol(address, value);
        self.current_closure().set_symbol(symbol, address, mutable);
        address
    }
    pub fn get_symbol(&mut self, symbol: &Ident) -> PassResult<(Address, Mut)> {
        if let Some(address) = self.current_closure().get_symbol(symbol) {
            Ok(address)
        } else {
            Err(symbol.err(ERR_NOT_FOUND))
        }
    }
    pub fn modify_symbol(&mut self, address: Address, modify: ModifyFn) -> PassResult<()> {
        let value = if let Some(symbol) = self.symbols.symbols.get_mut(&address) {
            std::mem::take(symbol)
        } else {
            return Err((0, ERR_NOT_FOUND))
        };
        let value = modify(self, value)?;
        self.symbols.set_symbol(address, value);
        Ok(())
    }
    pub fn get_ident(&mut self, address: Address) -> PassResult<&Ident> {
        self.closures
            .iter_mut()
            .find_map(|closure| closure.get_ident(address))
            .ok_or((0, ERR_NOT_FOUND))
    }
    pub fn stored_type(&mut self, node: impl AsNode) -> PassResult<Type> {
        match self.symbols.get_type(node.source()) {
            Some(SymbolType::Type(t)) => return Ok(t.to_owned()),
            Some(SymbolType::Address(a)) => {
                let symbol = self.symbols.get_symbol(*a);
                if let Some(value) = symbol {
                    let val: Value = value.to_owned().try_into()?;
                    return val.try_into()
                }
            },
            _ => ()
        }
        Err(node.err(ERR_NOT_FOUND))
    }
}

fn increment_id(env: &mut Env, closure: impl FnOnce(&mut Env)) -> ScopeId {
    env.state.current_id += 1;
    closure(env);
    env.state.current_id
}