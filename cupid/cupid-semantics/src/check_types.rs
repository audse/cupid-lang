use crate::{error, map_expr, map_stmt,
    utils::{get_type_ident, rewrite_symbol_unscoped},
    Error,
};
use cupid_ast::{expr, stmt, types};
use cupid_env::environment::Env;
use cupid_util::InvertOption;

#[allow(unused_variables)]
#[auto_implement::auto_implement(Vec, Box, Option)]
pub trait CheckTypes
where
    Self: Sized,
{
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        Ok(self)
    }
}

impl CheckTypes for expr::Expr {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        map_expr!(self => |expr| expr.check_types(env)?)
    }
}

impl CheckTypes for stmt::Stmt {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        map_stmt!(self => |stmt| stmt.check_types(env)?)
    }
}

impl CheckTypes for expr::block::Block {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            expressions: self.expressions.check_types(env)?,
            ..self
        })
    }
}

impl CheckTypes for expr::function::Function {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        let return_type = get_type_ident(self.body.attr.source, env)?;
        let return_type_annotation = self
            .return_type_annotation
            .clone()
            .unwrap_or_else(|| "none".into());
        if return_type != &return_type_annotation {
            Err(error(format!(
                "type mismatch: expected `{}`, found `{}`",
                return_type_annotation.name, return_type.name
            )))
        } else {
            Ok(Self {
                body: self.body.check_types(env)?,
                ..self
            })
        }
    }
}

impl CheckTypes for expr::ident::Ident {}

impl CheckTypes for expr::value::Value {}

impl CheckTypes for types::traits::Trait {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            methods: self.methods.check_types(env)?,
            ..self
        })
    }
}

impl CheckTypes for types::typ::Type {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            methods: self.methods.check_types(env)?,
            ..self
        })
    }
}

impl CheckTypes for stmt::decl::Decl {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        rewrite_symbol_unscoped(&self.ident, env, |env, expr| {
            let val_type = expr
                .attr()
                .map(|a| get_type_ident(a.source, env).cloned())
                .unwrap_or(Ok("none".into()))?;
            if let Some(type_annotation) = &self.type_annotation && type_annotation != &val_type {
                return Err(error(format!("type mismatch: expected `{}`, found `{}`", type_annotation.name, val_type.name)))
            }
            expr.check_types(env)
        })?;
        Ok(Self {
            ident: self.ident.check_types(env)?,
            type_annotation: self.type_annotation.check_types(env)?,
            ..self
        })
    }
}

impl CheckTypes for stmt::trait_def::TraitDef {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        rewrite_symbol_unscoped(&self.ident, env, |env, expr| {
            expr.check_types(env)
        })?;
        Ok(self)
    }
}

impl CheckTypes for stmt::type_def::TypeDef {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        rewrite_symbol_unscoped(&self.ident, env, |env, expr| {
            expr.check_types(env)
        })?;
        Ok(self)
    }
}