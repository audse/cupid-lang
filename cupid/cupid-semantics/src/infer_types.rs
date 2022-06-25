use crate::{
    map_expr, map_stmt,
    Error,
};
use cupid_ast::{expr, stmt, types};
use cupid_env::environment::Env;
use cupid_util::InvertOption;

#[allow(unused_variables)]
#[auto_implement::auto_implement(Vec, Option, Box)]
pub trait InferTypes where Self: Sized {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error> { Ok(self) }
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

impl InferTypes for expr::block::Block {}
impl InferTypes for expr::function::Function {}
impl InferTypes for expr::ident::Ident {}
impl InferTypes for expr::value::Value {}

impl InferTypes for types::traits::Trait {}
impl InferTypes for types::typ::Type {}

impl InferTypes for stmt::decl::Decl {}
impl InferTypes for stmt::trait_def::TraitDef {}
impl InferTypes for stmt::type_def::TypeDef {}