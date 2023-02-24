use crate::{map_expr, map_stmt, Error, ToError, ToErrorCode};
use cupid_ast::{attr::GetAttr, expr, stmt, types};
use cupid_debug::code::ErrorCode;
use cupid_env::{environment::Env, expr_closure::Value};
use cupid_util::{InvertOption, WrapRc, WrapRefCell};
use std::{cell::RefCell, rc::Rc};

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

impl<T: ResolveNames + std::default::Default> ResolveNames for Rc<RefCell<T>> {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        self.swap(&self.take().resolve_names(env)?.ref_cell());
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
        Ok(Self {
            expressions: self.expressions.resolve_names(env)?,
            ..self
        })
    }
}

impl ResolveNames for expr::function::Function {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            params: self.params.resolve_names(env)?,
            return_type_annotation: self.return_type_annotation.resolve_names(env)?,
            body: self.body.resolve_names(env)?,
            ..self
        })
    }
}

impl ResolveNames for expr::function_call::FunctionCall {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            function: self.function.resolve_names(env)?,
            args: self.args.resolve_names(env)?,
            ..self
        })
    }
}

impl ResolveNames for expr::ident::Ident {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        if let Some(_) = env
            .get_closure(self.attr.source)
            .unwrap()
            .borrow_mut()
            .reference(&self)
        {
            Ok(self)
        } else {
            Err(self.err_not_found(env))
        }
    }
}

impl ResolveNames for expr::namespace::Namespace {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            namespace: self.namespace.resolve_names(env)?,
            // Don't resolve value yet- it may be related to the type
            // value: self.value.resolve_names(env)?,
            ..self
        })
    }
}

impl ResolveNames for expr::value::Value {}

impl ResolveNames for types::traits::Trait {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            ident: self.ident.resolve_names(env)?,
            methods: self.methods.resolve_names(env)?,
            ..self
        })
    }
}

impl ResolveNames for types::typ::Type {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            ident: self.ident.resolve_names(env)?,
            fields: self.fields.resolve_names(env)?,
            methods: self.methods.resolve_names(env)?,
            ..self
        })
    }
}

impl ResolveNames for stmt::allocate::Allocation {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        match self {
            Self::Expr(e) => Ok(Self::Expr(e.resolve_names(env)?)),
            Self::Trait(t) => Ok(Self::Trait(t.resolve_names(env)?)),
            Self::Type(t) => Ok(Self::Type(t.resolve_names(env)?)),
        }
    }
}

impl ResolveNames for stmt::allocate::Allocate {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        use stmt::allocate::AllocationStage;
        match self.stage {
            AllocationStage::NameResolution => {
                let value = self.value.resolve_names(env)?;
                if let Some(scope) = env.get_closure_mut(self.attr.source) {
                    let value = Value::build().value(value.clone()).build();
                    scope
                        .borrow_mut()
                        .parent
                        .as_mut()
                        .unwrap()
                        .borrow_mut()
                        .insert(self.ident.clone(), value)
                        .map_err(|code| self.ident.err(code, env))?;
                }
                Ok(Self {
                    ident: self.ident.resolve_names(env)?,
                    value,
                    ..self
                })
            }
            AllocationStage::TypeNameResolution | AllocationStage::Runtime => Ok(Self {
                ident: self.ident.resolve_names(env)?,
                value: self.value.resolve_names(env)?,
                ..self
            }),
        }
    }
}

impl ResolveNames for stmt::assign::Assign {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self(self.0.resolve_names(env)?))
    }
}

impl ResolveNames for stmt::decl::Decl {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            allocate: self.allocate.resolve_names(env)?,
            type_annotation: self.type_annotation.resolve_names(env)?,
            ..self
        })
    }
}

impl ResolveNames for stmt::implement::Impl {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            trait_ident: self.trait_ident.resolve_names(env)?,
            type_ident: self.type_ident.resolve_names(env)?,
            methods: self.methods.resolve_names(env)?,
            ..self
        })
    }
}

impl ResolveNames for stmt::trait_def::TraitDef {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self(self.0.resolve_names(env)?))
    }
}

impl ResolveNames for stmt::type_def::TypeDef {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self(self.0.resolve_names(env)?))
    }
}
