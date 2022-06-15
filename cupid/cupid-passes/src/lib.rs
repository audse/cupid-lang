#![allow(unused_variables)]
#![feature(derive_default_enum)]

pub mod env;

pub mod util;
pub(crate) use util::attributes::*;
pub(crate) use util::static_nodes::*;

pub type PassResult<T> = Result<T, (Source, ErrCode)>;
pub type ErrCode = usize;

/// Each AST pass takes a node from the previous pass and transforms it
/// # Stages
/// 1. `pre_analysis`
/// 2. `package_resolution`
/// 3. `type_name_resolution`
/// 4. `scope_analysis`
/// 5. `name_resolution`
/// 6. `type_inference`
/// 7. `type_checking`
/// 8. `flow_checking`
/// 9. `linting`

pub mod flow_checking;
pub mod linting;
pub mod name_resolution;
pub mod package_resolution;
pub mod pre_analysis;
pub mod scope_analysis;
pub mod type_checking;
pub mod type_inference;
pub mod type_name_resolution;

use PassExpr::*;

#[derive(Debug, Default, Clone)]
pub enum PassExpr {
    PreAnalysis(pre_analysis::Expr),
    PackageResolved(package_resolution::Expr),
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
    ($for:expr => $do:ident $args:tt) => {
        match $for {
            PreAnalysis(x) => x.$do $args,
            PackageResolved(x) => x.$do $args,
            TypeNameResolved(x) => x.$do $args,
            ScopeAnalyzed(x) => x.$do $args,
            NameResolved(x) => x.$do $args,
            TypeInferred(x) => x.$do $args,
            TypeChecked(x) => x.$do $args,
            FlowChecked(x) => x.$do $args,
            Linted(x) => x.$do $args,
            _ => unreachable!()
        }
    }
}

impl AsNode for PassExpr {
    fn scope(&self) -> Scope {
        for_each_expr!(self => scope())
    }
    fn source(&self) -> Scope {
        for_each_expr!(self => source())
    }
    fn typ(&self) -> Scope {
        for_each_expr!(self => typ())
    }
	fn set_source(&mut self, source: Source) {
        for_each_expr!(self => set_source(source));
    }
	fn set_scope(&mut self, scope: Scope) {
        for_each_expr!(self => set_scope(scope));
    }
	fn set_typ(&mut self, typ: Address) {
        for_each_expr!(self => set_typ(typ));
    }
}