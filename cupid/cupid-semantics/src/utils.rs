use crate::{Address, Error, ToError};
use cupid_ast::expr::{Expr, ident::Ident};
use cupid_debug::code::ErrorCode;
use cupid_env::{
    database::{
        source_table::query::Query as SourceQuery, symbol_table::query::Query as SymbolQuery,
        table::QueryTable,
    },
    environment::Env,
};

pub(super) fn read(ident: &Ident, env: &mut Env) -> Option<Address> {
    env.read::<Address>(&SymbolQuery::select(ident)).map(|a| *a)
}

pub(super) fn is_undefined(ident: &Ident, env: &mut Env) -> Result<(), Error> {
    let val = read(ident, env);
    if val.is_none() {
        Ok(())
    } else {
        Err(ident.err(ErrorCode::AlreadyDefined, env).with_hint(format!("`{ident:?}` is defined as `{val:?}`")))
    }
}

pub(super) fn get_type_ident<'env>(
    source: Address,
    env: &'env mut Env,
) -> Result<&'env Ident, &str> {
    if let Some(ident) = env.database.read::<Ident>(&SourceQuery::select(source)) {
        Ok(ident)
    } else {
        Err("could not get type")
    }
}

pub(super) fn insert_symbol<T: Into<Expr>>(ident: &Ident, env: &mut Env, current_expr: T, mut transform_expr: impl FnMut(&mut Env, T) -> Result<T, Error>) -> Result<(), Error> {
    env.inside_closure(ident.attr.scope, |env| {
        is_undefined(ident, env)?;

        let value = transform_expr(env, current_expr)?.into();
        let query = SymbolQuery::insert()
            .write(ident)
            .write(value);

        env.insert(query);

        Ok(())
    })
}

pub(super) fn rewrite_symbol(ident: &Ident, env: &mut Env, mut expr: impl FnMut(&mut Env, Expr) -> Result<Expr, Error>) -> Result<(), Error> {
    env.inside_closure(ident.attr.scope, |env| {
        let mut value = env
            .take::<Expr>(&SymbolQuery::select(ident))
            .unwrap();
        value = expr(env, value)?;
        env.write(SymbolQuery::select(ident).write(value));
        Ok(())
    })
}

pub(super) fn update_type(source: Address, env: &mut Env, typ: Ident) -> Result<(), Error> {
    env.database.source_table.write(SourceQuery::select(source).write(typ));
    Ok(())
}

pub(super) fn rewrite_symbol_unscoped(ident: &Ident, env: &mut Env, mut expr: impl FnMut(&mut Env, Expr) -> Result<Expr, Error>) -> Result<(), Error> {
    let mut value = env
        .database
        .symbol_table
        .take::<Expr>(&SymbolQuery::select(ident))
        .unwrap();
    value = expr(env, value)?;
    env.database.symbol_table.write(SymbolQuery::select(ident).write(value));
    Ok(())
}