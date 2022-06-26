use crate::{
    map_expr, map_stmt,
    Error,
};
use cupid_ast::{expr, stmt, types};
use cupid_env::environment::Env;
use cupid_util::InvertOption;


#[allow(unused_variables)]
#[auto_implement::auto_implement(Vec, Option, Box)]
pub trait ResolvePackages where Self: Sized {
    fn resolve_packages(self, env: &mut Env) -> Result<Self, Error> {
        Ok(self)
    }
}

impl ResolvePackages for expr::Expr {
    fn resolve_packages(self, env: &mut Env) -> Result<Self, Error> {
        map_expr!(self => |expr| expr.resolve_packages(env)?)
    }
}

impl ResolvePackages for stmt::Stmt {
    fn resolve_packages(self, env: &mut Env) -> Result<Self, Error> {
        map_stmt!(self => |stmt| stmt.resolve_packages(env)?)
    }
}

impl ResolvePackages for expr::block::Block {}
impl ResolvePackages for expr::function::Function {}

impl ResolvePackages for expr::ident::Ident {}

impl ResolvePackages for expr::value::Value {}

impl ResolvePackages for types::traits::Trait {}
impl ResolvePackages for types::typ::Type {}

impl ResolvePackages for stmt::decl::Decl {}
impl ResolvePackages for stmt::trait_def::TraitDef {}
impl ResolvePackages for stmt::type_def::TypeDef {}