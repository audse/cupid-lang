use crate::{
    map_expr, map_stmt,
    utils::insert_symbol,
    Error,
};
use cupid_ast::{expr, stmt, types};
use cupid_env::environment::Env;
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

impl ResolveTypeNames for expr::function_call::FunctionCall {}

impl ResolveTypeNames for expr::ident::Ident {}
impl ResolveTypeNames for expr::namespace::Namespace {}

impl ResolveTypeNames for expr::value::Value {}

impl ResolveTypeNames for stmt::decl::Decl {}

impl ResolveTypeNames for types::traits::Trait {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        env.inside_closure(self.attr.scope, |env| {
            Ok(Self {
                methods: self.methods.resolve_type_names(env)?,
                ..self
            })
        })
    }
}

impl ResolveTypeNames for types::typ::Type {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        env.inside_closure(self.attr.scope, |env| {
            Ok(Self {
                fields: self.fields.resolve_type_names(env)?,
                methods: self.methods.resolve_type_names(env)?,
                ..self
            })
        })
    }
}

impl ResolveTypeNames for stmt::trait_def::TraitDef {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        insert_symbol(&self.ident, env, self.value.into(), |env, expr: expr::Expr| {
            Ok(expr.resolve_type_names(env)?)
        })?;
        Ok(Self {
            // value is moved into DB, so use a default
            value: types::traits::Trait::default(),
            ..self
        })
    }
}

impl ResolveTypeNames for stmt::type_def::TypeDef {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        insert_symbol(
            &self.ident,
            env,
            self.value.into(),
            |env, expr: expr::Expr| Ok(expr.resolve_type_names(env)?),
        )?;
        Ok(Self {
            // value is moved into DB, so use a default
            value: types::typ::Type::default(),
            ..self
        })
    }
}
