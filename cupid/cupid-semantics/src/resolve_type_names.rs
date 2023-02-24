use crate::{map_expr, map_stmt, Error, InsideClosure, ToError, ToErrorCode};
use cupid_ast::{attr::GetAttr, expr, stmt, types};
use cupid_debug::code::ErrorCode;
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

impl ResolveTypeNames for stmt::allocate::Allocation {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        match self {
            Self::Expr(e) => Ok(Self::Expr(e.resolve_type_names(env)?)),
            Self::Trait(t) => Ok(Self::Trait(t.resolve_type_names(env)?)),
            Self::Type(t) => Ok(Self::Type(t.resolve_type_names(env)?)),
        }
    }
}

impl ResolveTypeNames for stmt::allocate::Allocate {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        use stmt::allocate::AllocationStage;
        let value = Self {
            ident: self.ident.resolve_type_names(env)?,
            value: self.value.resolve_type_names(env)?,
            ..self
        };
        match self.stage {
            AllocationStage::TypeNameResolution => {
                if let Some(closure) = env.get_closure(value.attr.source) {
                    closure
                        .borrow_mut()
                        .parent
                        .as_mut()
                        .unwrap()
                        .borrow_mut()
                        .insert(
                            value.ident.clone(),
                            Value::build().value(value.value.clone()).build(),
                        )
                        .map_err(|code| value.ident.err(code, env))?;
                }
                if let Some(closure) = env.get_closure(value.value.attr().source) {
                    let mut closure = closure.borrow_mut();
                    for generic in value.ident.generics.iter() {
                        closure
                            .insert(
                                generic.clone(),
                                Value::build()
                                    .type_def(types::typ::Type::variable().ref_cell().rc())
                                    .build(),
                            )
                            .map_err(|code| generic.err(code, env))?;
                    }
                }
                Ok(value)
            }
            AllocationStage::NameResolution | AllocationStage::Runtime => Ok(value),
        }
    }
}

impl ResolveTypeNames for stmt::assign::Assign {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self(self.0.resolve_type_names(env)?))
    }
}

impl ResolveTypeNames for stmt::decl::Decl {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            allocate: self.allocate.resolve_type_names(env)?,
            ..self
        })
    }
}

impl ResolveTypeNames for stmt::implement::Impl {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        self.in_closure(env, |env, closure| {
            let trait_value = closure.lookup(&self.trait_ident)?.get_trait_value()?;
            let type_value = closure.lookup(&self.type_ident)?.get_type_value()?;

            let trait_closure = env.get_closure(trait_value.borrow().attr().source)?;
            let type_closure = env.get_closure(type_value.borrow().attr().source)?;

            type_closure.borrow_mut().namespaces.push(trait_closure);
            Some(())
        })
        .flatten()
        .ok_or_else(|| self.trait_ident.err_not_found(env))?;
        Ok(Self {
            trait_ident: self.trait_ident.resolve_type_names(env)?,
            type_ident: self.type_ident.resolve_type_names(env)?,
            methods: self.methods.resolve_type_names(env)?,
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
        Ok(Self(self.0.resolve_type_names(env)?))
    }
}

impl ResolveTypeNames for stmt::type_def::TypeDef {
    fn resolve_type_names(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self(self.0.resolve_type_names(env)?))
    }
}
