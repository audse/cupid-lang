#![allow(unused_variables)]
#![feature(derive_default_enum, is_some_with, trait_alias)]

pub mod env;
pub(crate) use env::{Address, Source, ScopeId, Env, SymbolValue, Mut};

pub mod tests;

pub mod util;
pub(crate) use util::attributes::*;
pub(crate) use util::static_nodes::*;

pub type PassResult<T> = Result<T, (Source, ErrCode)>;
pub type ErrCode = usize;

/// Each AST pass takes a node from the previous pass and transforms it
/// # Stages
///  1. `pre_analysis`
///  2. `package_resolution`
///  3. `type_scope_analysis`
///  4. `type_name_resolution`
///  5. `scope_analysis`
///  6. `name_resolution`
///  7. `type_inference`
///  8. `type_checking`
///  9. `flow_checking`
/// 10. `linting`

pub mod flow_checking;
pub mod linting;
pub mod name_resolution;
pub mod package_resolution;
pub mod pre_analysis;
pub mod scope_analysis;
pub mod type_checking;
pub mod type_inference;
pub mod type_name_resolution;
pub mod type_scope_analysis;

use PassExpr::*;

#[derive(Debug, Default, Clone)]
pub enum PassExpr {
    PreAnalysis(pre_analysis::Expr),
    PackageResolved(package_resolution::Expr),
    TypeScopeAnalyzed(type_scope_analysis::Expr),
    TypeNameResolved(type_name_resolution::Expr),
    ScopeAnalyzed(scope_analysis::Expr),
    NameResolved(name_resolution::Expr),
    TypeInferred(type_inference::Expr),
    TypeChecked(type_checking::Expr),
    FlowChecked(flow_checking::Expr),
    Linted(linting::Expr),
    
    #[default]
    Empty
}

macro_rules! for_each_expr {
    ($for:expr => $do:expr) => {
        match $for {
            PreAnalysis(x) => $do(x),
            PackageResolved(x) => $do(x),
            TypeNameResolved(x) => $do(x),
            ScopeAnalyzed(x) => $do(x),
            NameResolved(x) => $do(x),
            TypeInferred(x) => $do(x),
            TypeChecked(x) => $do(x),
            FlowChecked(x) => $do(x),
            Linted(x) => $do(x),
            _ => unreachable!()
        }
    }
}

pub trait AsExpr<T> {
    fn as_expr(self) -> PassResult<T>;
}

impl PassExpr {
    pub fn unwrap_value(self) -> PassResult<Value> {
        for_each_expr!(self => |x| AsExpr::as_expr(x))
    }
}

impl AsNode for PassExpr {
    fn scope(&self) -> ScopeId {
        for_each_expr!(self => |x: &dyn AsNode| x.scope())
    }
    fn source(&self) -> ScopeId {
        for_each_expr!(self => |x: &dyn AsNode| x.source())
    }
	fn set_source(&mut self, source: Source) {
        for_each_expr!(self => |x: &mut dyn AsNode| x.set_source(source));
    }
	fn set_scope(&mut self, scope: ScopeId) {
        for_each_expr!(self => |x: &mut dyn AsNode| x.set_scope(scope));
    }
}

/// Creates `From<T> for PassExpr` impl block for every provided `Expr`
macro_rules! impl_pass_expr_from_expr {
    ($($pass:ident => $into:ident;)*) => { $(
        impl From<$pass::Expr> for PassExpr {
            fn from(expr: $pass::Expr) -> Self {
                Self::$into(expr)
            }
        }
    )* };
}

impl_pass_expr_from_expr! {
    pre_analysis => PreAnalysis;
    package_resolution => PackageResolved;
    type_scope_analysis => TypeScopeAnalyzed;
    type_name_resolution => TypeNameResolved;
    scope_analysis => ScopeAnalyzed;
    name_resolution => NameResolved;
    type_inference => TypeInferred;
    type_checking => TypeChecked;
    flow_checking => FlowChecked;
    linting => Linted;
}