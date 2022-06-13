#![allow(unused)]
use cupid_util::{InvertOption, Bx};
use cupid_scope::Env;
use crate::{pre_analysis, PassResult};

pub trait AnalyzeScope<T> where Self: Sized {
    fn analyze_scope(self, env: &mut Env) -> PassResult<T>;
}

#[derive(Debug, Default, Clone)]
pub enum Expr {
    Decl(Decl),
    Ident(pre_analysis::Ident),

    #[default]
    Empty,
}

/// Only structs that undergo changes in a phase will be redefined.
/// Otherwise, the most recent pass' structure will be used.
/// In this pass, `Decl.value` is redefined, but `Ident`s have
/// not changed since pre-analysis.
cupid_util::node_builder! {
    #[derive(Debug, Default, Clone)]
    pub DeclBuilder => pub Decl {
        pub ident: pre_analysis::Ident,
        pub type_annotation: Option<pre_analysis::Ident>,
        pub value: Box<Expr>,
    }
}

impl AnalyzeScope<Expr> for pre_analysis::Expr {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Expr> {
        todo!()
    }
}

impl AnalyzeScope<pre_analysis::Ident> for pre_analysis::Ident {
    fn analyze_scope(self, env: &mut Env) -> PassResult<pre_analysis::Ident> {
        Ok(pre_analysis::Ident::build()
            .name(self.name)
            .generics(self.generics
                .into_iter()
                .map(|g| g.analyze_scope(env))
                .collect::<PassResult<Vec<pre_analysis::Ident>>>()?)
            .meta(self.source, env.current_closure, self.typ)
            .build())
    }
}

impl AnalyzeScope<Decl> for pre_analysis::Decl {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Decl> {
        Ok(Decl::build()
            .ident(self.ident.analyze_scope(env)?)
            .value(self.value.analyze_scope(env)?.bx())
            .type_annotation(self.type_annotation.map(|t| t.analyze_scope(env)).invert()?)
            .meta(self.source, env.current_closure, self.typ)
            .build())
    }
}