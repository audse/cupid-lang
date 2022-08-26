use crate::{map_expr, map_stmt, Error, InsideClosure, ToError};
use cupid_ast::{expr, stmt, types};
use cupid_env::{environment::Env, expr_closure::Value};
use cupid_util::{InvertOption, WrapRc, WrapRefCell};
use std::{cell::RefCell, rc::Rc};

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

impl<T: ResolveTypeNames + std::default::Default> ResolveTypeNames for Rc<RefCell<T>> {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        self.swap(&self.take().resolve_type_names(env)?.ref_cell());
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

impl ResolveTypeNames for expr::ident::Ident {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        self.in_closure(env, |env, mut closure| {
            for generic in self.generics.iter() {
                closure
                    .insert(
                        generic.clone(),
                        Value::build()
                            .type_def(types::typ::Type::variable().ref_cell().rc())
                            .build(),
                    )
                    .map_err(|code| generic.err(code, env))?;
            }
            Ok(())
        })
        .unwrap()?;
        Ok(self)
    }
}
impl ResolveTypeNames for expr::namespace::Namespace {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            namespace: self.namespace.resolve_type_names(env)?,
            value: self.value.resolve_type_names(env)?,
            ..self
        })
    }
}

impl ResolveTypeNames for expr::value::Value {}

impl ResolveTypeNames for stmt::assign::Assign {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            value: self.value.resolve_type_names(env)?,
            ..self
        })
    }
}

impl ResolveTypeNames for stmt::decl::Decl {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            value: self.value.resolve_type_names(env)?,
            ..self
        })
    }
}

impl ResolveTypeNames for types::traits::Trait {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            ident: self.ident.resolve_type_names(env)?,
            methods: self.methods.resolve_type_names(env)?,
            ..self
        })
    }
}

impl ResolveTypeNames for types::typ::Type {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            ident: self.ident.resolve_type_names(env)?,
            fields: self.fields.resolve_type_names(env)?,
            methods: self.methods.resolve_type_names(env)?,
            ..self
        })
    }
}

impl ResolveTypeNames for stmt::trait_def::TraitDef {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        let ident = self.ident.resolve_type_names(env)?;
        let value = self.value.resolve_type_names(env)?;
        if let Some(scope) = env.get_closure(self.attr.source) {
            scope
                .borrow_mut()
                .parent
                .as_mut()
                .unwrap()
                .borrow_mut()
                .insert(
                    ident.clone(),
                    Value::build().trait_def(value.clone()).build(),
                )
                .map_err(|code| ident.err(code, env))?;
        }
        Ok(Self {
            ident,
            value,
            ..self
        })
    }
}

impl ResolveTypeNames for stmt::type_def::TypeDef {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        let ident = self.ident.resolve_type_names(env)?;
        let value = self.value.resolve_type_names(env)?;
        if let Some(scope) = env.get_closure_mut(self.attr.source) {
            let value = Value::build().type_def(value.clone()).build();
            scope
                .borrow_mut()
                .parent
                .as_mut()
                .unwrap()
                .borrow_mut()
                .insert(ident.clone(), value)
                .map_err(|code| ident.err(code, env))?;
        }
        Ok(Self {
            ident,
            value,
            ..self
        })
    }
}
