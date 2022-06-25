use crate::{
    database::db::Database,
    scope::scope::Scope,
    ScopeId,
};

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
    pub database: Database,
    pub scope: Scope,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            database: Database::default(),
            scope: Scope::default(),
        }
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
}
