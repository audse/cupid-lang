use cupid_util::{ERR_NOT_FOUND, ERR_NOT_IN_SCOPE};

use super::{
    database::Query,
    database::{row::*, Database},
    scope::scope::Scope,
};
use crate::{AsNode, PassResult};

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
    pub fn inside_closure<R>(
        &mut self,
        closure: ScopeId,
        fun: impl FnOnce(&mut Env) -> PassResult<R>,
    ) -> PassResult<R> {
        self.scope.state.use_closure(closure);
        let result = fun(self)?;
        self.scope.state.reset_closure();
        Ok(result)
    }

    pub fn insert<V: Default>(&mut self, query: Query<V>) -> Address
    where
        RowExpr: WriteRowExpr<V>,
    {
        let address = self.database.insert(query);
        self.scope.set_symbol(address);
        address
    }

    pub fn read<Col: Selector>(&self, query: &Query<()>) -> PassResult<&Col> {
        let row = self
            .database
            .read::<Row>(query)
            .ok_or(query.err(ERR_NOT_FOUND))?;
        if self.scope.is_in_scope(row.address) {
            Ok(Col::select(row))
        } else {
            Err(query.err(ERR_NOT_IN_SCOPE))
        }
    }

    pub fn write<V: Default>(&mut self, query: Query<V>) -> PassResult<()>
    where
        RowExpr: WriteRowExpr<V>,
    {
        let address = read_address(&mut self.database, &query.to_read_only())?;
        if self.scope.is_in_scope(address) {
            self.database.write(query);
            Ok(())
        } else {
            Err(query.err(ERR_NOT_IN_SCOPE))
        }
    }

    pub fn write_pass<V: Default, F: FnOnce(&mut Env, Prev) -> PassResult<V>, Prev>(
        &mut self,
        query: Query<V>,
        closure: F,
    ) -> PassResult<()>
    where
        RowExpr: WriteRowExpr<V> + WriteRowExpr<Prev>,
        RowExpr: TakeRowExpr<Prev>,
    {
        let read_query = query.to_read_only();
        let address = read_address(&mut self.database, &read_query)?;
        if self.scope.is_in_scope(address) {
            let mut expr = self.database.take::<RowExpr>(&read_query).unwrap();
            let col: Prev = expr.take().unwrap();
            expr.write(closure(self, col)?);
            self.database.write::<RowExpr>(Query::<RowExpr>::build().address(address).expr(expr));
            Ok(())
        } else {
            Err(query.err(ERR_NOT_IN_SCOPE))
        }
    }
}

fn read_address(database: &mut Database, query: &Query<()>) -> PassResult<Address> {
    database
        .read::<Address>(query)
        .map(|address| *address)
        .ok_or(query.err(ERR_NOT_FOUND))
}
