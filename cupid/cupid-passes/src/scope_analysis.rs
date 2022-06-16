use cupid_util::InvertOption;

use crate::{env::environment::Context, type_name_resolution as prev_pass, util, Env, Ident, IsTyped, PassResult, Untyped};

#[cupid_semantics::auto_implement(Vec, Option, Str, Box)]
pub trait AnalyzeScope<Output>
where
    Self: Sized,
{
    fn analyze_scope(self, env: &mut Env) -> PassResult<Output>;
}

util::define_pass_nodes! {
    Decl: util::reuse_node! {
        prev_pass::Decl => Pass<analyze_scope>
    }
    Function: util::reuse_node! {
        prev_pass::Function => Pass<analyze_scope>
    }
    TypeDef: util::completed_node! { prev_pass::TypeDef => AnalyzeScope<analyze_scope> }
}

crate::util::impl_default_passes! {
    impl AnalyzeScope + analyze_scope for {
        Expr => prev_pass::Expr;
        Field<Ident> => Ident;
        Value => crate::Value;
    }
}

impl AnalyzeScope<crate::Block<Expr>> for crate::Block<prev_pass::Expr> {
    fn analyze_scope(self, env: &mut Env) -> PassResult<crate::Block<Expr>> {
        let scope = env.add_scope(Context::Block);
        env.inside_scope(scope, |env| {
            Ok(self
                .pass(Vec::<prev_pass::Expr>::analyze_scope, env)?
                .build_scope(scope))
        })
    }
}

impl AnalyzeScope<Decl> for prev_pass::Decl {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Decl> {
        Ok(self.pass(env)?.build_scope(env.current_closure))
    }
}

impl AnalyzeScope<Function> for prev_pass::Function {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Function> {
        let scope = env.add_closure(Context::Function);
        env.inside_scope(scope, |env| Ok(self.pass(env)?.build_scope(scope)))
    }
}

impl AnalyzeScope<Ident> for Ident {
    fn analyze_scope(mut self, env: &mut Env) -> PassResult<Ident> {
        self.namespace = self.namespace.analyze_scope(env)?;
        self.attr.scope = self.namespace
            .as_ref()
            .map(|n| n.attr.scope)
            .unwrap_or(env.current_closure);
        env.inside_scope(self.attr.scope, |env| {
            self.generics = self.generics.analyze_scope(env)?;
            Ok(self)
        })
    }
}

impl AnalyzeScope<IsTyped<Ident>> for IsTyped<Ident> {
    fn analyze_scope(self, env: &mut Env) -> PassResult<IsTyped<Ident>> {
        Ok(Untyped(self.into_inner().analyze_scope(env)?))
    }
}