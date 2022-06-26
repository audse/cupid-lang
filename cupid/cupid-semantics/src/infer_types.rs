use crate::{
    map_expr, map_stmt,
    Error, utils::{update_type, rewrite_symbol_unscoped},
};
use cupid_ast::{expr, stmt, types};
use cupid_env::{environment::Env, database::{table::QueryTable, symbol_table::query::Query}};
use cupid_types::infer::Infer;
use cupid_util::InvertOption;

#[allow(unused_variables)]
#[auto_implement::auto_implement(Vec, Option, Box)]
pub trait InferTypes where Self: Sized {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> { Ok(self) }
}

impl InferTypes for expr::Expr {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        map_expr!(self => |expr| expr.infer_types(env)?)
    }
}

impl InferTypes for stmt::Stmt {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        map_stmt!(self => |stmt| stmt.infer_types(env)?)
    }
}

impl InferTypes for expr::block::Block {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        update_type(self.attr.source, env, self.infer().ident)?;
        Ok(Self {
            expressions: self.expressions.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for expr::function::Function {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        update_type(self.attr.source, env, self.infer().ident)?;
        Ok(Self {
            params: self.params.infer_types(env)?,
            body: self.body.infer_types(env)?,
            return_type_annotation: self.return_type_annotation.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for expr::ident::Ident {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        let value = env.read::<expr::Expr>(&Query::select(self.address.unwrap()));
        if let Some(value) = value {
            let typ = value.infer().ident;
            update_type(value.attr().unwrap_or_else(|| self.attr).source, env, typ.clone())?;
            update_type(self.attr.source, env, typ)?;
        } else {
            update_type(self.attr.source, env, self.infer().ident)?;
        }
        Ok(Self {
            namespace: self.namespace.infer_types(env)?,
            generics: self.generics.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for expr::value::Value {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        update_type(self.attr.source, env, self.infer().ident)?;
        Ok(self)
    }
}

impl InferTypes for types::traits::Trait {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        update_type(self.attr.source, env, self.infer().ident)?;
        Ok(Self {
            ident: self.ident.infer_types(env)?,
            methods: self.methods.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for types::typ::Type {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        update_type(self.attr.source, env, self.infer().ident)?;
        Ok(Self {
            ident: self.ident.infer_types(env)?,
            fields: self.fields.infer_types(env)?,
            methods: self.methods.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for stmt::decl::Decl {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        update_type(self.attr.source, env, self.infer().ident)?;
        rewrite_symbol_unscoped(&self.ident, env, |env, expr| {
            expr.infer_types(env)
        })?;
        Ok(Self {
            ident: self.ident.infer_types(env)?,
            type_annotation: self.type_annotation.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for stmt::trait_def::TraitDef {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        update_type(self.attr.source, env, self.infer().ident)?;
        rewrite_symbol_unscoped(&self.ident, env, |env, expr| {
            expr.infer_types(env)
        })?;
        Ok(Self {
            ident: self.ident.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for stmt::type_def::TypeDef {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        update_type(self.attr.source, env, self.infer().ident)?;
        Ok(Self {
            ident: self.ident.infer_types(env)?,
            ..self
        })
    }
}