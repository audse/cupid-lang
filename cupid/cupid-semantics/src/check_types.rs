use crate::{map_expr, map_stmt,
    utils::{get_type_ident, rewrite_symbol_unscoped},
    Error, ToError,
};
use cupid_ast::{expr, stmt, types};
use cupid_debug::code::ErrorCode;
use cupid_env::{environment::Env, database::{symbol_table::query::Query, table::QueryTable}};
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
        let return_type = get_type_ident(self.body.attr.source, env);
        let return_type = match return_type {
            Ok(v) => v,
            Err(_) => return Err(self.err(ErrorCode::ExpectedType, env))
        };
        let return_type_annotation = self
            .return_type_annotation
            .clone()
            .unwrap_or_else(|| "none".into());
        if return_type != &return_type_annotation {
            let hint = format!(
                "expected `{}`, found `{}`",
                return_type_annotation.name, return_type.name
            );
            Err(self.err(ErrorCode::TypeMismatch, env).with_hint(hint))
        } else {
            Ok(Self {
                body: self.body.check_types(env)?,
                ..self
            })
        }
    }
}

impl CheckTypes for expr::function_call::FunctionCall {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        let (mut arg_types, mut param_types) = (vec![], vec![]);
        for arg in &self.args {
            let t = get_type_ident(arg.attr().unwrap().source, env).cloned();
            match t {
                Ok(typ) => arg_types.push(typ),
                Err(_) => return Err(self.err(ErrorCode::NotFound, env))
            }
        }
        let function = env
            .read::<expr::Expr>(&Query::select(self.function.address.unwrap()))
            .unwrap();
        let function = match function {
            expr::Expr::Function(function) => function,
            _ => return Err(self.err(ErrorCode::ExpectedFunction, env))
        };
        for param in &function.params {
            param_types.push(param.type_annotation.as_ref().unwrap().clone())
        }
        let matched_args = arg_types.into_iter().zip(param_types);
        for (arg_type, param_type) in matched_args {
            if arg_type != param_type {
                return Err(
                    arg_type
                        .err(ErrorCode::TypeMismatch, env)
                        .with_hint(format!("expected type `{param_type:#?}`, found type `{arg_type:#?}`"))
                );
            }
        }
        Ok(self)
    }
}

impl CheckTypes for expr::ident::Ident {}

impl CheckTypes for expr::namespace::Namespace {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        todo!()
    }
}

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
                .unwrap_or(Ok("none".into()));
            let val_type = match val_type {
                Ok(v) => v,
                Err(_) => return Err(self.err(ErrorCode::ExpectedType, env))
            };
            if let Some(type_annotation) = &self.type_annotation && type_annotation != &val_type {
                return Err(
                    self.err(ErrorCode::TypeMismatch, env)
                        .with_hint(format!("expected `{}`, found `{}`", type_annotation.name, val_type.name))
                );
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
