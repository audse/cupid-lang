use crate::{
    map_expr, map_stmt,
    utils::{insert_symbol, read, rewrite_symbol},
    Error, error,
};
use cupid_ast::{expr, stmt, types};
use cupid_env::{environment::Env, database::{table::QueryTable, symbol_table::query::Query}};
use cupid_util::{InvertOption, Bx};

#[allow(unused_variables)]
#[auto_implement::auto_implement(Vec, Box, Option)]
pub trait ResolveNames
where
    Self: Sized,
{
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(self)
    }
}

impl ResolveNames for expr::Expr {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        map_expr!(self => |expr| expr.resolve_names(env)?)
    }
}

impl ResolveNames for stmt::Stmt {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        map_stmt!(self => |stmt| stmt.resolve_names(env)?)
    }
}

impl ResolveNames for expr::block::Block {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        env.inside_closure(self.attr.scope, |env| {
            Ok(Self {
                expressions: self.expressions.resolve_names(env)?,
                ..self
            })
        })
    }
}

impl ResolveNames for expr::function::Function {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        let return_type_annotation = self.return_type_annotation.resolve_names(env)?;
        env.inside_closure(self.attr.scope, |env| {
            Ok(Self {
                params: self.params.resolve_names(env)?,
                return_type_annotation,
                body: self.body.resolve_names(env)?,
                ..self
            })
        })
    }
}

impl ResolveNames for expr::ident::Ident {
    fn resolve_names(mut self, env: &mut Env) -> Result<Self, Error> {
        let namespace = self.namespace
            .as_ref()
            .map(|name| env.read::<expr::Expr>(&Query::select(&**name)))
            .flatten()
            .map(|n| n.attr())
            .flatten();
        let scope = namespace.unwrap_or_else(|| self.attr).scope;
        env.inside_closure(scope, |env| {
            self.address = Some(read(&self, env).ok_or_else(|| error(format!("not defined: {self:#?}")))?);
            Ok(self)
        })
    }
}

impl ResolveNames for expr::value::Value {}

impl ResolveNames for types::traits::Trait {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        env.inside_closure(self.attr.scope, |env| {
            Ok(Self {
                ident: self.ident.resolve_names(env)?,
                methods: self.methods.resolve_names(env)?,
                ..self
            })
        })
    }
}

impl ResolveNames for types::typ::Type {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        env.inside_closure(self.attr.scope, |env| {
            Ok(Self {
                ident: self.ident.resolve_names(env)?,
                fields: self.fields.resolve_names(env)?,
                methods: self.methods.resolve_names(env)?,
                ..self
            })
        })
    }
}

impl ResolveNames for stmt::decl::Decl {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        // TODO resolve ident generics + namespace
        insert_symbol(&self.ident, env, *self.value, |env, expr: expr::Expr| Ok(expr.resolve_names(env)?))?;
        Ok(Self {
            ident: self.ident.resolve_names(env)?,
            type_annotation: self.type_annotation.resolve_names(env)?,
            // value is moved into DB, so use a default
            value: expr::Expr::default().bx(),
            ..self
        })
    }
}

impl ResolveNames for stmt::trait_def::TraitDef {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        rewrite_symbol(&self.ident, env, |env, trait_value| {
            trait_value.resolve_names(env)
        })?;
        Ok(Self {
            ident: self.ident.resolve_names(env)?,
            ..self
        })
    }
}

impl ResolveNames for stmt::type_def::TypeDef {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        rewrite_symbol(&self.ident, env, |env, type_value| {
            type_value.resolve_names(env)
        })?;
        Ok(Self {
            ident: self.ident.resolve_names(env)?,
            ..self
        })
    }
}
