use crate::{
    map_expr, map_stmt,
    Error,
};
use cupid_ast::{expr, stmt, types};
use cupid_env::{environment::Env, database::{symbol_table::{query::Query, row::Ref}, table::QueryTable}};
use cupid_util::InvertOption;


#[allow(unused_variables)]
#[auto_implement::auto_implement(Vec, Option, Box)]
pub trait CheckFlow where Self: Sized {
    fn check_flow(self, env: &mut Env) -> Result<Self, Error> {
        Ok(self)
    }
}

impl CheckFlow for expr::Expr {
    fn check_flow(self, env: &mut Env) -> Result<Self, Error> {
        map_expr!(self => |expr| expr.check_flow(env)?)
    }
}

impl CheckFlow for stmt::Stmt {
    fn check_flow(self, env: &mut Env) -> Result<Self, Error> {
        map_stmt!(self => |stmt| stmt.check_flow(env)?)
    }
}

impl CheckFlow for expr::block::Block {}
impl CheckFlow for expr::function::Function {}
impl CheckFlow for expr::function_call::FunctionCall {}

impl CheckFlow for expr::ident::Ident {
    fn check_flow(self, env: &mut Env) -> Result<Self, Error> {
        // Write a reference to the database
        env.database.symbol_table.write(Query::select(&self).write(Ref(1)));
        Ok(self)
    }
}

impl CheckFlow for expr::namespace::Namespace {}

impl CheckFlow for expr::value::Value {}

impl CheckFlow for types::traits::Trait {}
impl CheckFlow for types::typ::Type {}

impl CheckFlow for stmt::decl::Decl {}
impl CheckFlow for stmt::trait_def::TraitDef {}
impl CheckFlow for stmt::type_def::TypeDef {}