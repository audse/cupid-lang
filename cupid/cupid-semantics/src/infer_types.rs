use crate::{map_expr, map_stmt, Error, InsideClosure, ToError};
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
                if let Some(typ) = value.get_type() {
                    closure.decorate_closure(typ);
                    Ok(())
                } else {
                    Err(self.err(ErrorCode::CannotInfer, env))
                }
            } else {
                Err(self.err(ErrorCode::NotFound, env))
            }
        })
        .unwrap()?;
        Ok(Self {
            generics: self.generics.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for expr::namespace::Namespace {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        self.in_closure(env, |_env, mut closure| {
            closure.decorate_closure(self.value.infer().rc());
        });
        self.value.in_closure(env, |env, mut closure| {
            if let expr::Expr::Ident(ident) = &*self.namespace
                && let Some(namespace_address) = closure.lookup(ident).map(|v| v.get_source()).flatten() 
                && let Some(namespace_closure) = env.get_closure(namespace_address) 
            {
                closure.namespaces.push(namespace_closure);
            }
        });
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

impl InferTypes for stmt::assign::Assign {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        self.in_closure(env, |_env, mut closure| {
            closure.decorate_closure(self.infer().rc());
        });
        Ok(Self {
            ident: self.ident.infer_types(env)?,
            value: self.value.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for stmt::decl::Decl {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        self.in_closure(env, |_env, mut closure| {
            closure.decorate_closure(self.infer().rc());
        });
        let value = self.value.infer_types(env)?;
        let type_annotation = self.type_annotation.infer_types(env)?;
        if !value.borrow().is_empty() {
            value
                .in_closure(env, |env, mut closure| {
                    if let Some(typ) = closure.get_type() {
                        closure.decorate(&self.ident, typ.clone());
                        Ok(())
                    } else {
                        Err(self.ident.err(ErrorCode::CannotInfer, env))
                    }
                })
                .unwrap()?;
        }
        if let Some(type_annotation) = &type_annotation {
            if let Some(closure) = env.get_closure(self.attr.source) {
                let type_symbol = closure.borrow().lookup(type_annotation).unwrap();
                let typ = if let Some(type_value) = type_symbol.get_type_value() {
                    type_value.borrow().clone().rc()
                } else {
                    return Err(type_annotation.err(ErrorCode::ExpectedType, env));
                };
                closure.borrow_mut().decorate(&self.ident, typ);
            }
        } else if value.borrow().is_empty() {
            return Err(self.ident.err(ErrorCode::CannotInfer, env));
        }
        Ok(Self {
            ident: self.ident.infer_types(env)?,
            type_annotation,
            value,
            ..self
        })
    }
}

impl InferTypes for stmt::trait_def::TraitDef {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        self.in_closure(env, |_env, mut closure| {
            let typ = self.value.borrow().infer().rc();
            closure.decorate(&self.ident, typ.clone());
            closure.decorate_closure(typ);
        });
        Ok(Self {
            ident: self.ident.infer_types(env)?,
            value: self.value.infer_types(env)?,
            ..self
        })
    }
}

impl InferTypes for stmt::type_def::TypeDef {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> {
        self.in_closure(env, |_env, mut closure| {
            let typ = self.value.borrow().infer().rc();
            closure.decorate(&self.ident, typ.clone());
            closure.decorate_closure(typ);
        });
        Ok(Self {
            ident: self.ident.infer_types(env)?,
            value: self.value.infer_types(env)?,
            ..self
        })
    }
}
