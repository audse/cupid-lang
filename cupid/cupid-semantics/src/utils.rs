use crate::Error;
use cupid_ast::expr;
use cupid_env::{
    database::{symbol_table::query::Query, table::QueryTable},
    environment::Env,
};

pub(super) fn is_undefined(ident: &expr::ident::Ident, env: &mut Env) -> Result<(), Error> {
    if env.database.read::<usize>(&Query::select(ident)).is_some() {
        Ok(())
    } else {
        Err(Error)
    }
}
