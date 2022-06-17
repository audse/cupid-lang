
use cupid_util::InvertOption;

use crate::{package_resolution as prev_pass, PassResult, env::{Env, Context}, AsNode, util, Ident};

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
            .build_scope(env.current_closure))
    }
}

impl AnalyzeTypeScope<crate::IsTyped<Ident>> for crate::IsTyped<Ident> {
    fn analyze_type_scope(self, env: &mut Env) -> PassResult<crate::IsTyped<Ident>> {
        let mut ident = self.pass(
            AnalyzeTypeScope::analyze_type_scope,
            AnalyzeTypeScope::analyze_type_scope,
            env
        )?;
        ident.set_scope(env.current_closure);
        Ok(ident)
    }
}

impl AnalyzeTypeScope<TypeDef> for prev_pass::TypeDef {
    fn analyze_type_scope(self, env: &mut Env) -> PassResult<TypeDef> {
        let scope = env.add_toplevel_closure(Context::Type);
        env.inside_scope(scope, |env| {
            Ok(self.pass(env)?.build_scope(scope))
        })
    }
}