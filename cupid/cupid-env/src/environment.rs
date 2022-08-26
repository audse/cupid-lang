use crate::{
    database::db::Database,
    expr_closure::{Arena, ExprClosure},
    scope::scope::Scope,
    Address, ScopeId,
};
use cupid_util::{WrapRc, WrapRefCell};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

#[derive(Debug, Default, Copy, Clone)]
pub enum Context {
    #[default]
    Block,
    Function,
    Loop,
    Method,
    Trait,
    Type,
}

#[derive(Debug, Clone)]
pub struct Env {
    pub source: Rc<String>,
    pub database: Database,
    pub scope: Scope,
    pub arena: Arena,
    pub closures: BTreeMap<Address, Rc<RefCell<ExprClosure>>>,
}

impl Default for Env {
    fn default() -> Self {
        let mut new = Self {
            database: Database::default(),
            scope: Scope::default(),
            source: Rc::new(String::new()),
            arena: Arena::default(),
            closures: BTreeMap::default(),
        };
        new.add_closure(0, None);
        new
    }
}

impl Env {
    pub fn inside_closure<R, E>(
        &mut self,
        closure: ScopeId,
        fun: impl FnOnce(&mut Env) -> Result<R, E>,
    ) -> Result<R, E> {
        self.scope.state.use_closure(closure);
        let result = fun(self)?;
        self.scope.state.reset_closure();
        Ok(result)
    }

    pub fn add_closure(
        &mut self,
        address: Address,
        parent: Option<Rc<RefCell<ExprClosure>>>,
    ) -> Rc<RefCell<ExprClosure>> {
        self.closures.insert(
            address,
            ExprClosure {
                parent,
                ..Default::default()
            }
            .ref_cell()
            .rc(),
        );
        self.get_closure(address).unwrap()
    }

    pub fn get_closure(&self, address: Address) -> Option<Rc<RefCell<ExprClosure>>> {
        self.closures.get(&address).cloned()
    }

    pub fn get_closure_mut(&mut self, address: Address) -> Option<Rc<RefCell<ExprClosure>>> {
        self.closures.get_mut(&address).cloned()
    }
}
