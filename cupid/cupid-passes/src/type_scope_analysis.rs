
use cupid_util::InvertOption;

use crate::{package_resolution as prev_pass, PassResult, Env, env::Context, util, Ident};

#[cupid_semantics::auto_implement(Vec, Option, Str, Box)]
pub trait AnalyzeTypeScope<Output> where Self: Sized {
    fn analyze_type_scope(self, env: &mut Env) -> PassResult<Output>;
}

util::define_pass_nodes! {
    Decl: util::reuse_node! { 
        prev_pass::Decl => AnalyzeTypeScope<Decl, analyze_type_scope> 
    }
    Function: util::reuse_node! { 
        prev_pass::Function => AnalyzeTypeScope<Function, analyze_type_scope> 
    }
    TypeDef: util::reuse_node! { 
        prev_pass::TypeDef => Pass<analyze_type_scope> 
    }
}

util::impl_default_passes! {
    impl AnalyzeTypeScope + analyze_type_scope for {
        Block<Expr> => Block<prev_pass::Expr>;
        Expr => prev_pass::Expr;
        crate::Field<Ident>;
        crate::Value;
    }
}

impl AnalyzeTypeScope<Ident> for Ident {
    fn analyze_type_scope(self, env: &mut Env) -> PassResult<Ident> {
        Ok(self
            .pass(AnalyzeTypeScope::analyze_type_scope, Self::analyze_type_scope, env)?
            .build_scope(env.scope.state.closure()))
    }
}

impl AnalyzeTypeScope<TypeDef> for prev_pass::TypeDef {
    fn analyze_type_scope(self, env: &mut Env) -> PassResult<TypeDef> {
        let scope = env.scope.add_toplevel_closure(Context::Type);
        env.inside_closure(scope, |env| {
            Ok(self.pass(env)?.build_scope(scope))
        })
    }
}