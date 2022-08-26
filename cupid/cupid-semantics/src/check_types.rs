use crate::{map_expr, map_stmt, Error, InsideClosure, ToError};
use cupid_ast::{expr, stmt, types, types::typ::Type};
use cupid_debug::code::ErrorCode;
use cupid_env::environment::Env;
use cupid_util::{InvertOption, WrapRefCell};
use std::{cell::RefCell, rc::Rc};

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

impl<T: CheckTypes + std::default::Default> CheckTypes for Rc<RefCell<T>> {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        self.swap(&self.take().check_types(env)?.ref_cell());
        Ok(self)
    }
}

pub trait GetType {
    fn get_type(&self, env: &mut Env) -> Option<Rc<Type>>;
    fn get_type_value(&self, _env: &mut Env) -> Option<Rc<RefCell<Type>>> {
        None
    }
}

impl GetType for expr::ident::Ident {
    fn get_type(&self, env: &mut Env) -> Option<Rc<Type>> {
        self.in_closure(env, |_env, closure| {
            if let Some(val) = closure.lookup(self) {
                val.get_type()
            } else {
                None
            }
        })
        .flatten()
    }
    fn get_type_value(&self, env: &mut Env) -> Option<Rc<RefCell<Type>>> {
        self.in_closure(env, |_env, closure| {
            if let Some(val) = closure.lookup(self) {
                val.get_type_value()
            } else {
                None
            }
        })
        .flatten()
    }
}

impl GetType for Option<expr::ident::Ident> {
    fn get_type(&self, env: &mut Env) -> Option<Rc<Type>> {
        self.as_ref().map(|i| i.get_type(env)).flatten()
    }
    fn get_type_value(&self, env: &mut Env) -> Option<Rc<RefCell<Type>>> {
        self.as_ref().map(|i| i.get_type_value(env)).flatten()
    }
}

impl GetType for expr::Expr {
    fn get_type(&self, env: &mut Env) -> Option<Rc<Type>> {
        match self {
            expr::Expr::Empty => None,
            _ => self
                .in_closure(env, |_env, closure| closure.get_type())
                .flatten(),
        }
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
        if let Some(return_type_annotation) = &self.return_type_annotation {
            let return_type = self
                .body
                .in_closure(env, |_env, closure| closure.get_type())
                .flatten()
                .unwrap();
            if &return_type.ident != return_type_annotation {
                let hint = format!(
                    "expected `{}`, found `{}`",
                    return_type_annotation.name, return_type.ident.name
                );
                return Err(self.err(ErrorCode::TypeMismatch, env).with_hint(hint));
            }
        }
        Ok(Self {
            body: self.body.check_types(env)?,
            ..self
        })
    }
}

impl CheckTypes for expr::function_call::FunctionCall {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        let function_type = self.function.get_type(env).unwrap();

        match self.args.len().cmp(&(function_type.fields.len() - 1)) {
            std::cmp::Ordering::Greater => return Err(self.err(ErrorCode::TooManyArgs, env)),
            std::cmp::Ordering::Less => return Err(self.err(ErrorCode::NotEnoughArgs, env)),
            _ => (),
        }

        let arg_param_list = self.args.iter().zip(function_type.fields.iter());
        for (arg, param) in arg_param_list {
            let arg_type = arg.get_type(env).unwrap();
            if let Some(param_type) = &param.type_annotation {
                let param_type = param_type.get_type_value(env).unwrap();
                if arg_type.ident != param_type.borrow().ident {
                    return Err(arg.err(ErrorCode::TypeMismatch, env).with_hint(format!(
                        "expected type `{}`, found type `{}`",
                        param_type.borrow().ident.name,
                        arg_type.ident.name,
                    )));
                }
            }
        }
        Ok(self)
    }
}

impl CheckTypes for expr::ident::Ident {}

impl CheckTypes for expr::namespace::Namespace {
    fn check_types(self, _env: &mut Env) -> Result<Self, Error> {
        todo!()
    }
}

impl CheckTypes for expr::value::Value {}

impl CheckTypes for types::traits::Trait {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        let methods = self.methods.check_types(env)?;
        for method in methods.iter() {
            let typ = method.ident.get_type(env).unwrap();
            if !typ.is_function() {
                return Err(method.err(ErrorCode::ExpectedFunction, env));
            }
        }
        Ok(Self { methods, ..self })
    }
}

impl CheckTypes for types::typ::Type {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        let methods = self.methods.check_types(env)?;
        for method in methods.iter() {
            let typ = method.ident.get_type(env).unwrap();
            if !typ.is_function() {
                return Err(method.err(ErrorCode::ExpectedFunction, env));
            }
        }
        Ok(Self { methods, ..self })
    }
}

impl CheckTypes for stmt::assign::Assign {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        let ident_type = self.ident.get_type(env).unwrap();
        let val_type = self.value.borrow().get_type(env).unwrap();
        if val_type.ident != ident_type.ident {
            return Err(self.err(ErrorCode::TypeMismatch, env).with_hint(format!(
                "expected `{}`, found `{}`",
                ident_type.ident.name, val_type.ident.name
            )));
        }
        Ok(Self {
            value: self.value.check_types(env)?,
            ..self
        })
    }
}

impl CheckTypes for stmt::decl::Decl {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        if let Some(type_annotation) = &self.type_annotation {
            let val_type = self.value.borrow().get_type(env).unwrap();
            if &val_type.ident != type_annotation {
                return Err(self.err(ErrorCode::TypeMismatch, env).with_hint(format!(
                    "expected `{}`, found `{}`",
                    type_annotation.name, val_type.ident.name
                )));
            }
        }
        Ok(Self {
            value: self.value.check_types(env)?,
            ..self
        })
    }
}

impl CheckTypes for stmt::trait_def::TraitDef {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            value: self.value.check_types(env)?,
            ..self
        })
    }
}

impl CheckTypes for stmt::type_def::TypeDef {
    fn check_types(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            value: self.value.check_types(env)?,
            ..self
        })
    }
}
