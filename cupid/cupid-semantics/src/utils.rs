use crate::{Address, Error, error};
use cupid_ast::expr::{Expr, ident::Ident};
use cupid_env::{
    database::{
        source_table::query::Query as SourceQuery, symbol_table::query::Query as SymbolQuery,
        table::QueryTable,
    },
    environment::Env,
};

pub(super) fn read(ident: &Ident, env: &mut Env) -> Option<Address> {
    env.database
        .read::<Address>(&SymbolQuery::select(ident))
        .map(|a| *a)
}

pub(super) fn is_defined(ident: &Ident, env: &mut Env) -> Result<(), Error> {
    if read(ident, env).is_some() {
        Ok(())
    } else {
        Err(error(format!("not defined: `{ident:?}`")))
    }
}

pub(super) fn is_undefined(ident: &Ident, env: &mut Env) -> Result<(), Error> {
    let val = read(ident, env);
    if val.is_none() {
        Ok(())
    } else {
        Err(error(format!("already defined: `{ident:?}` is defined as `{val:?}`")))
    }
}

// pub(super) fn get_type_ident<'env>(
//     source: Address,
//     env: &'env mut Env,
// ) -> Result<&'env Ident, Error> {
//     if let Some(ident) = env.database.read::<Ident>(&SourceQuery::select(source)) {
//         Ok(ident)
//     } else {
//         Err(Error)
//     }
// }

pub(super) fn insert_symbol<T: Into<Expr>>(ident: &Ident, env: &mut Env, current_expr: T, mut transform_expr: impl FnMut(&mut Env, T) -> Result<T, Error>) -> Result<(), Error> {
    env.inside_closure(ident.attr.scope, |env| {
        is_undefined(ident, env)?;

        let query = SymbolQuery::insert()
            .write(ident)
            .write(transform_expr(env, current_expr)?.into());
        env.database.symbol_table.insert(query);

        Ok(())
    })
}

pub(super) fn rewrite_symbol(ident: &Ident, env: &mut Env, mut expr: impl FnMut(&mut Env, Expr) -> Result<Expr, Error>) -> Result<(), Error> {
    env.inside_closure(ident.attr.scope, |env| {
        let mut value = env
            .database
            .symbol_table
            .take::<Expr>(&SymbolQuery::select(ident))
            .unwrap();
        value = expr(env, value)?;
        env.database
            .symbol_table
            .write(SymbolQuery::select(ident).write(value));
        Ok(())
    })
}