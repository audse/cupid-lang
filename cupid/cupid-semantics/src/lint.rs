use crate::{map_expr, map_stmt, Error, InsideClosure, ToError};
use cupid_ast::{expr, stmt, types};
use cupid_debug::code::ErrorCode;
use cupid_env::environment::Env;
use cupid_util::InvertOption;

#[allow(unused_variables)]
#[auto_implement::auto_implement(Vec, Option, Box)]
pub trait Lint
where
    Self: Sized,
{
    fn lint(self, env: &mut Env) -> Result<Self, Error> {
        Ok(self)
    }
}

impl Lint for expr::Expr {
    fn lint(self, env: &mut Env) -> Result<Self, Error> {
        map_expr!(self => |expr| expr.lint(env)?)
    }
}

impl Lint for stmt::Stmt {
    fn lint(self, env: &mut Env) -> Result<Self, Error> {
        map_stmt!(self => |stmt| stmt.lint(env)?)
    }
}

impl Lint for expr::block::Block {
    fn lint(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            expressions: self.expressions.lint(env)?,
            ..self
        })
    }
}

impl Lint for expr::function::Function {
    fn lint(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            params: self.params.lint(env)?,
            body: self.body.lint(env)?,
            ..self
        })
    }
}

impl Lint for expr::function_call::FunctionCall {
    fn lint(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            args: self.args.lint(env)?,
            function: self.function.lint(env)?,
            ..self
        })
    }
}

impl Lint for expr::ident::Ident {
    fn lint(self, env: &mut Env) -> Result<Self, Error> {
        let num_refs = self
            .in_closure(env, |_env, closure| {
                closure.lookup(&self).unwrap().get_refs()
            })
            .unwrap_or_default();
        if num_refs <= 1 {
            Err(self.err(ErrorCode::UnusedVariable, env))
        } else {
            Ok(self)
        }
    }
}

impl Lint for expr::namespace::Namespace {}

impl Lint for expr::value::Value {}

impl Lint for types::traits::Trait {}

impl Lint for types::typ::Type {}

impl Lint for stmt::allocate::Allocate {
    fn lint(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            ident: self.ident.lint(env)?,
            ..self
        })
    }
}

impl Lint for stmt::assign::Assign {
    fn lint(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self(self.0.lint(env)?))
    }
}

impl Lint for stmt::decl::Decl {
    fn lint(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            allocate: self.allocate.lint(env)?,
            ..self
        })
    }
}

impl Lint for stmt::implement::Impl {
    fn lint(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self {
            trait_ident: self.trait_ident.lint(env)?,
            type_ident: self.type_ident.lint(env)?,
            methods: self.methods.lint(env)?,
            ..self
        })
    }
}

impl Lint for stmt::trait_def::TraitDef {
    fn lint(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self(self.0.lint(env)?))
    }
}

impl Lint for stmt::type_def::TypeDef {
    fn lint(self, env: &mut Env) -> Result<Self, Error> {
        Ok(Self(self.0.lint(env)?))
    }
}
