#![allow(unused_imports, unused)]

use super::env::{*, database::*};
use super::*;

pub(super)type TestResult = crate::PassResult<()>;

pub(super)trait IsErrCode {
    fn is(self, code: cupid_util::ErrCode) -> bool;
}

impl<T> IsErrCode for crate::PassResult<T> {
    fn is(self, code: cupid_util::ErrCode) -> bool {
        self.is_err_and(|e| e.1 == code)
    }
}

pub(super) fn env() -> crate::Env {
    crate::Env::default()
}

pub(super) fn ident(name: &'static str) -> crate::Ident {
	crate::Ident { name: name.into(), ..Default::default() }
}

pub(super) fn decl(name: &'static str) -> pre_analysis::Decl {
    pre_analysis::Decl { ident: ident(name), ..Default::default() }
}

pub(super) fn decl_val(name: &'static str, value: Value) -> pre_analysis::Decl {
    pre_analysis::Decl { 
        ident: ident(name),
        value: pre_analysis::Expr::Value(value).bx(),
        ..Default::default()
    }
}

pub(super) fn typed_decl(name: &'static str, type_annotation: &'static str) -> pre_analysis::Decl {
    pre_analysis::Decl { ident: ident(name), type_annotation: Some(ident(type_annotation)), ..Default::default() }
}

pub(super) fn int(i: i32) -> Value {
    VInteger(i, Attributes::default())
}

pub(super) fn typ(name: &'static str) -> Type {
    Type::from(ident(name))
}

pub(super) fn int_typ() -> Type {
    Type::from(ident("int"))
}

pub(super) fn add_typ(env: &mut Env, t: Type) -> PassResult<()> {
    let query = Query::<name_resolution::Expr>::build()
        .ident(t.name.clone())
        .expr(name_resolution::Expr::from(VType(t)));
        
    env.insert(query);
    Ok(())
}

pub(super) fn pass<A, B, C, D, E, F, G>(node: A, env: &mut Env) -> crate::PassResult<G> 
where 
    A: ResolvePackages<B>, 
    B: AnalyzeTypeScope<C>, 
    C: ResolveTypeNames<D>, 
    D: AnalyzeScope<E>, 
    E: ResolveNames<F>,
    F: InferTypes<G>,
{
    node.resolve_packages(env)?
        .analyze_type_scope(env)?
        .resolve_type_names(env)?
        .analyze_scope(env)?
        .resolve_names(env)?
        .infer_types(env)
}

pub(super) fn pass_all<A, B, C, D, E, F, G>(nodes: impl Into<Vec<A>>, env: &mut Env) -> crate::PassResult<Vec<G>> 
where 
    A: ResolvePackages<B>, 
    B: AnalyzeTypeScope<C>, 
    C: ResolveTypeNames<D>, 
    D: AnalyzeScope<E>, 
    E: ResolveNames<F>,
    F: InferTypes<G>,
{
    let nodes: Vec<B> = nodes.into().resolve_packages(env)?;
    let nodes: Vec<C> = nodes.analyze_type_scope(env)?;
    let nodes: Vec<D> = nodes.resolve_type_names(env)?;
    let nodes: Vec<E> = nodes.analyze_scope(env)?;
    let nodes: Vec<F> = nodes.resolve_names(env)?;
    let nodes: Vec<G> = nodes.infer_types(env)?;
    Ok(nodes)
}
