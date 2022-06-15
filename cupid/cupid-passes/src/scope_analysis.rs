use cupid_util::{InvertOption, Bx};

use crate::{type_name_resolution as prev_pass, PassResult, util, env::environment::Env};

#[cupid_semantics::auto_implement(Vec, Option, Str)]
pub trait AnalyzeScope<Output> where Self: Sized {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Output>;
}

util::define_pass_nodes! {
    Decl: util::reuse_node! { 
        prev_pass::Decl => Pass<analyze_scope> 
    }
    Function: util::reuse_node! { 
        prev_pass::Function => Pass<analyze_scope> 
    }
    Ident: util::reuse_node! { 
        prev_pass::Ident => Pass<analyze_scope> 
    }
    TypeDef: util::reuse_node! { 
        prev_pass::TypeDef => Pass<analyze_scope> 
    }
}

crate::util::impl_default_passes! {
    impl AnalyzeScope + analyze_scope for {
        Expr => prev_pass::Expr;
        Field<Ident> => prev_pass::Ident;
        Value => crate::Value;
    }
}

impl AnalyzeScope<crate::Block<Expr>> for crate::Block<prev_pass::Expr> {
    fn analyze_scope(self, env: &mut Env) -> PassResult<crate::Block<Expr>> {
        Ok(self
            .pass(Vec::<prev_pass::Expr>::analyze_scope, env)?
            .build_scope(env.add_scope(cupid_scope::Context::Block)))
    }
}

impl AnalyzeScope<Decl> for prev_pass::Decl {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Decl> {
        Ok(self.pass(env)?.build_scope(env.current_scope))
    }
}

impl AnalyzeScope<Function> for prev_pass::Function {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Function> {
        Ok(self.pass(env)?.build_scope(env.add_scope(cupid_scope::Context::Function)))
    }
}

impl AnalyzeScope<Ident> for prev_pass::Ident {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Ident> {
        Ok(self.pass(env)?.build_scope(env.current_scope))
    }
}

impl AnalyzeScope<TypeDef> for prev_pass::TypeDef {
    fn analyze_scope(self, env: &mut Env) -> PassResult<TypeDef> {
        Ok(self.pass(env)?.build_scope(env.add_isolated_closure(None, cupid_scope::Context::Type)))
    }
}
