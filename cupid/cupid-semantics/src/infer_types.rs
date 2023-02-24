use crate::{map_expr, map_stmt, Error, InsideClosure, ToError, ToErrorCode};
use cupid_ast::{attr::GetAttr, expr, stmt, types};
use cupid_debug::code::ErrorCode;
use cupid_env::environment::Env;
use cupid_types::infer::Infer;
use cupid_util::{InvertOption, WrapRc, WrapRefCell};
use std::{cell::RefCell, rc::Rc};

#[allow(unused_variables)]
#[auto_implement::auto_implement(Vec, Option, Box)]
pub trait InferTypes
where
    Self: Sized,
{
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        Ok(self)
    }
}

impl<T: GetAttr + Sized> InsideClosure for T {}

impl<T: InferTypes + std::default::Default> InferTypes for Rc<RefCell<T>> {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        self.swap(&self.take().infer_types(env)?.ref_cell());
        Ok(self)
    }
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
        self.in_closure(env, |_env, mut closure| {
            closure.decorate_closure(self.infer().rc());
        });
        Ok(Self {
            expressions: self.expressions.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for expr::function::Function {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        let value = Self {
            params: self.params.infer_types(env)?,
            body: self.body.infer_types(env)?,
            return_type_annotation: self.return_type_annotation.infer_types(env)?,
            ..self
        };
        value.in_closure(env, |_env, mut closure| {
            closure.decorate_closure(value.infer().rc());
        });
        Ok(value)
    }
}

impl InferTypes for expr::function_call::FunctionCall {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        let value = Self {
            function: self.function.infer_types(env)?,
            args: self.args.infer_types(env)?,
            ..self
        };
        let typ = value
            .function
            .in_closure(env, |env, closure| {
                if let Some(typ) = closure.get_type() {
                    Ok(typ)
                } else {
                    Err(value.err(ErrorCode::CannotInfer, env))
                }
            })
            .unwrap()?;
        value.in_closure(env, |_env, mut closure| closure.decorate_closure(typ));
        Ok(value)
    }
}

impl InferTypes for expr::ident::Ident {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        self.in_closure(env, |env, mut closure| {
            if let Some(value) = closure.lookup(&self) {
                if let Some(typ_value) = value.get_type_value() {
                    closure.decorate_closure(types::typ::Type::typ().rc());
                    Ok(())
                } else if let Some(typ) = value.get_type() {
                    closure.decorate_closure(typ);
                    Ok(())
                } else {
                    Err(self.err(ErrorCode::CannotInfer, env))
                }
            } else {
                Err(self.err_not_found(env))
            }
        })
        .unwrap()?;
        Ok(self)
    }
}

impl InferTypes for expr::namespace::Namespace {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        self.in_closure(env, |env, mut closure| {
            closure.decorate_closure(self.value.infer().rc());
            let namespace_type = self.namespace.infer().rc();
            if let Some(type_address) = closure.lookup(&namespace_type.ident).map(|v| v.get_source()).flatten()
                && let Some(type_closure) = env.get_closure(type_address)  
            {
                closure.namespaces.push(type_closure);
            }
        });
        self.value.in_closure(env, |env, mut closure| {
            if let expr::Expr::Ident(ident) = &*self.namespace {
                let namespace = closure.lookup(ident)
                    .ok_or_else(|| ident.err_not_found(env))?;
                let namespace_address = namespace.get_source()
                    .ok_or_else(|| self.err(ErrorCode::ExpectedExpression, env))?;
                let namespace_closure = env.get_closure(namespace_address)
                    .ok_or_else(|| self.err(ErrorCode::CannotInfer, env))?;
                closure.namespaces.push(namespace_closure);
                Ok(())
            } else {
                Ok(())
            }
        }).unwrap()?;
        Ok(Self {
            namespace: self.namespace.infer_types(env)?,
            value: self.value.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for expr::value::Value {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        self.in_closure(env, |_env, mut closure| {
            closure.decorate_closure(self.infer().rc());
        });
        Ok(self)
    }
}

impl InferTypes for types::traits::Trait {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        self.in_closure(env, |_env, mut closure| {
            closure.decorate_closure(self.infer().rc());
        });
        Ok(Self {
            ident: self.ident.infer_types(env)?,
            methods: self.methods.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for types::typ::Type {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        self.in_closure(env, |_env, mut closure| {
            closure.decorate_closure(self.infer().rc());
        });
        Ok(Self {
            ident: self.ident.infer_types(env)?,
            fields: self.fields.infer_types(env)?,
            methods: self.methods.infer_types(env)?,
            ..self
        })
    }
}


impl InferTypes for stmt::allocate::Allocation {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        match self {
            Self::Expr(e) => Ok(Self::Expr(e.infer_types(env)?)),
            Self::Trait(t) => Ok(Self::Trait(t.infer_types(env)?)),
            Self::Type(t) => Ok(Self::Type(t.infer_types(env)?)),
        }
    }
}

impl InferTypes for stmt::allocate::Allocate {
    fn infer_types(self,env: &mut Env) -> Result<Self,Error> {
        self.in_closure(env, |_env, mut closure| {
            let typ = self.value.infer().rc();
            closure.decorate(&self.ident, typ.clone());
            closure.decorate_closure(typ);
        });
        if !self.value.is_empty() {
            self.value.in_closure(env, |env, mut closure| {
                for generic in self.ident.generics.iter() {
                    if let Some(value) = closure.lookup(&generic) {
                        if let Some(typ) = value.get_type_value() {
                            let typ = typ.borrow().clone().rc();
                            closure.decorate(generic, typ.clone());
                            generic.in_closure(env, |env, mut generic_closure| {
                                generic_closure.decorate_closure(typ);
                            });
                        } else {
                            return Err(generic.err(ErrorCode::CannotInfer, env));
                        }
                    } else {
                        return Err(generic.err_not_found(env));
                    }
                }
                Ok(())
            }).unwrap()?;
        }
        Ok(Self {
            ident: self.ident.infer_types(env)?,
            value: self.value.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for stmt::assign::Assign {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self(self.0.infer_types(env)?))
    }
}

impl InferTypes for stmt::decl::Decl {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            allocate: self.allocate.infer_types(env)?,
            type_annotation: self.type_annotation.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for stmt::implement::Impl {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        self.in_closure(env, |env, mut closure| {
            closure.decorate_closure(self.infer().rc());
        });
        Ok(Self {
            trait_ident: self.trait_ident.infer_types(env)?,
            type_ident: self.type_ident.infer_types(env)?,
            methods: self.methods.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for stmt::trait_def::TraitDef {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self(self.0.infer_types(env)?))
    }
}

impl InferTypes for stmt::type_def::TypeDef {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self(self.0.infer_types(env)?))
    }
}
