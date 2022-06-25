use crate::{map_expr, map_stmt, utils::is_undefined, Error};
use cupid_ast::{expr, stmt};
use cupid_env::{
    database::{symbol_table::query::Query, table::QueryTable},
    environment::Env,
};
use cupid_util::InvertOption;

#[allow(unused_variables)]
#[auto_implement::auto_implement(Vec, Box, Option)]
pub trait ResolveTypeNames
where
    Self: Sized,
{
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(self)
    }
}

impl ResolveTypeNames for expr::Expr {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        map_expr!(self => |expr| expr.resolve_type_names(env)?)
    }
}

impl ResolveTypeNames for stmt::Stmt {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        map_stmt!(self => |stmt| stmt.resolve_type_names(env)?)
    }
}

impl ResolveTypeNames for expr::block::Block {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            expressions: self.expressions.resolve_type_names(env)?,
            ..self
        })
    }
}

impl ResolveTypeNames for expr::function::Function {}

impl ResolveTypeNames for expr::ident::Ident {}

impl ResolveTypeNames for expr::value::Value {}

impl ResolveTypeNames for stmt::decl::Decl {}

impl ResolveTypeNames for stmt::trait_def::TraitDef {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        is_undefined(&self.ident, env)?;

        let query = Query::select(&self.ident).write(expr::Expr::Value(self.value.into()));
        env.database.symbol_table.insert(query);

        // after this, the trait def is discarded
        Ok(Self::default())
    }
}

impl ResolveTypeNames for stmt::type_def::TypeDef {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        is_undefined(&self.ident, env)?;

        let query = Query::select(&self.ident).write(expr::Expr::Value(self.value.into()));
        env.database.symbol_table.insert(query);

        // after this, the type def is discarded
        Ok(Self::default())
    }
}
