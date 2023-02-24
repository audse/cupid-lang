pub mod analyze_scope;
pub mod check_flow;
pub mod check_types;
pub mod infer_types;
pub mod lint;
pub mod resolve_names;
pub mod resolve_packages;
pub mod resolve_type_names;

use std::{cell::RefMut, rc::Rc};

use cupid_ast::attr::GetAttr;
use cupid_debug::{code::ErrorCode, error::Error, source::ExprSource};
use cupid_env::{
    database::{source_table::query::Query, table::QueryTable},
    environment::Env,
    expr_closure::ExprClosure,
};

mod tests;

pub type Address = usize;

pub trait ToError: GetAttr {
    fn err(&self, code: ErrorCode, env: &mut cupid_env::environment::Env) -> Error {
        let context = env
            .database
            .read::<Rc<ExprSource>>(&Query::select(self.attr().source))
            .cloned()
            .unwrap_or_default();
        let source = env.source.clone();
        Error::new(context, source, code)
    }
}

impl<T: GetAttr> ToError for T {}

pub trait ToErrorCode: ToError {
    fn err_not_found(&self, env: &mut cupid_env::environment::Env) -> Error {
        self.err(ErrorCode::NotFound, env)
    }
}

impl ToErrorCode for cupid_ast::expr::ident::Ident {
    fn err_not_found(&self, env: &mut cupid_env::environment::Env) -> Error {
        self.err(ErrorCode::NotFound, env).with_message(format!("The identifier `{}` could not be found in the current scope", self.name))
    }
}

pub trait InsideClosure
where
    Self: GetAttr + Sized,
{
    fn in_closure<Returns, Fun: FnOnce(&mut Env, RefMut<ExprClosure>) -> Returns>(
        &self,
        env: &mut Env,
        fun: Fun,
    ) -> Option<Returns> {
        if let Some(closure) = env.get_closure(self.attr().source) {
            Some(fun(env, closure.borrow_mut()))
        } else {
            None
        }
    }
}

macro_rules! map_expr {
    ($to:ident => |$exp:ident| $inside:expr) => {{
        use expr::Expr::*;
        match $to {
            Block($exp) => Ok(Block($inside)),
            Function($exp) => Ok(Function($inside)),
            FunctionCall($exp) => Ok(FunctionCall($inside)),
            Ident($exp) => Ok(Ident($inside)),
            Namespace($exp) => Ok(Namespace($inside)),
            Value($exp) => Ok(Value($inside)),
            Trait($exp) => Ok(Trait($inside)),
            Type($exp) => Ok(Type($inside)),
            Stmt($exp) => Ok(Stmt($inside)),
            Empty => Ok(Empty),
        }
    }};
}

macro_rules! map_stmt {
    ($to:ident => |$stm:ident| $inside:expr) => {{
        use stmt::Stmt::*;
        match $to {
            Allocate($stm) => Ok(Allocate($inside)),
            Assign($stm) => Ok(Assign($inside)),
            Decl($stm) => Ok(Decl($inside)),
            Impl($stm) => Ok(Impl($inside)),
            TraitDef($stm) => Ok(TraitDef($inside)),
            TypeDef($stm) => Ok(TypeDef($inside)),
        }
    }};
}

macro_rules! for_expr {
    ($to:ident => |$exp:ident| $inside:expr) => {{
        use expr::Expr::*;
        match $to {
            Block($exp) => $inside,
            Function($exp) => $inside,
            FunctionCall($exp) => $inside,
            Ident($exp) => $inside,
            Namespace($exp) => $inside,
            Value($exp) => $inside,
            Trait($exp) => $inside,
            Type($exp) => $inside,
            Stmt($exp) => $inside,
            Empty => panic!("cannot perform operation on empty expression"),
        }
    }};
}

macro_rules! for_stmt {
    ($to:ident => |$stm:ident| $inside:expr) => {{
        use stmt::Stmt::*;
        match $to {
            Allocate($stm) => $inside,
            Assign($stm) => $inside,
            Decl($stm) => $inside,
            Impl($stm) => $inside,
            TraitDef($stm) => $inside,
            TypeDef($stm) => $inside,
        }
    }};
}

pub(crate) use {for_expr, for_stmt, map_expr, map_stmt};
