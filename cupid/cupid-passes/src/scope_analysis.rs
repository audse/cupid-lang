#![allow(unused)]
use cupid_util::{InvertOption, Bx, Str};
use cupid_scope::Env;
use crate::{pre_analysis, PassResult, ast_pass_nodes};

pub trait AnalyzeScope<T> where Self: Sized {
    fn analyze_scope(self, env: &mut Env) -> PassResult<T>;
}

ast_pass_nodes! {
    Decl: cupid_util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl {
            pub ident: Ident,
            pub type_annotation: Option<Ident>,
            pub value: Box<Expr>,
        }
    }
    Ident: cupid_util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub IdentBuilder => pub Ident {
            pub name: Str,
            pub generics: Vec<Ident>
        }
    }
}

impl<R, T: AnalyzeScope<R>> AnalyzeScope<Vec<R>> for Vec<T> {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Vec<R>> {
        self.into_iter()
            .map(|i| i.analyze_scope(env))
            .collect::<PassResult<Vec<R>>>()
    }
}

impl<R, T: AnalyzeScope<R>> AnalyzeScope<Option<R>> for Option<T> {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Option<R>> {
        self.map(|i| i.analyze_scope(env)).invert()
    }
}

impl AnalyzeScope<Expr> for pre_analysis::Expr {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Expr> {
        match self {
            Self::Decl(decl) => Ok(Expr::Decl(decl.analyze_scope(env)?)),
            _ => todo!()
        }
    }
}

impl AnalyzeScope<Decl> for pre_analysis::Decl {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Decl> {
        let Self { ident, value, type_annotation, source, typ, ..} = self;
        Ok(Decl::build()
            .ident(ident.analyze_scope(env)?)
            .value(value.analyze_scope(env)?.bx())
            .type_annotation(type_annotation.analyze_scope(env)?)
            .meta(source, env.current_closure, typ)
            .build())
    }
}

impl AnalyzeScope<Ident> for pre_analysis::Ident {
    fn analyze_scope(mut self, env: &mut Env) -> PassResult<Ident> {
        let Self { name, generics, source, typ, ..} = self;
        Ok(Ident::build()
            .name(name)
            .generics(generics.analyze_scope(env)?)
            .meta(source, env.current_closure, typ)
            .build())
    }
}

impl AnalyzeScope<crate::Value> for crate::Value {
    fn analyze_scope(self, env: &mut Env) -> PassResult<crate::Value> { Ok(self) }
}