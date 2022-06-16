use super::*;

pub type TestResult = crate::PassResult<()>;

pub trait IsErrCode {
    fn is(self, code: cupid_util::ErrCode) -> bool;
}

impl<T> IsErrCode for crate::PassResult<T> {
    fn is(self, code: cupid_util::ErrCode) -> bool {
        self.is_err_and(|e| e.1 == code)
    }
}

pub fn env() -> crate::Env {
    crate::Env::default()
}

pub fn ident(name: &'static str) -> crate::Ident {
	crate::Ident { name: name.into(), ..Default::default() }
}

pub fn decl(name: &'static str) -> pre_analysis::Decl {
    pre_analysis::Decl { ident: ident(name), ..Default::default() }
}

pub fn typed_decl(name: &'static str, type_annotation: &'static str) -> pre_analysis::Decl {
    pre_analysis::Decl { ident: ident(name), type_annotation: Some(ident(type_annotation)), ..Default::default() }
}

pub fn pass<A, B, C, D, E, F>(node: A, env: &mut Env) -> crate::PassResult<F> 
where 
    A: ResolvePackages<B>, 
    B: AnalyzeTypeScope<C>, 
    C: ResolveTypeNames<D>, 
    D: AnalyzeScope<E>, 
    E: ResolveNames<F> 
{
    node.resolve_packages(env)?
        .analyze_type_scope(env)?
        .resolve_type_names(env)?
        .analyze_scope(env)?
        .resolve_names(env)
}

pub fn pass_all<A, B, C, D, E, F>(nodes: impl Into<Vec<A>>, env: &mut Env) -> crate::PassResult<Vec<F>> 
where 
    A: ResolvePackages<B>, 
    B: AnalyzeTypeScope<C>, 
    C: ResolveTypeNames<D>, 
    D: AnalyzeScope<E>, 
    E: ResolveNames<F>
{
    let nodes: Vec<B> = nodes.into().resolve_packages(env)?;
    let nodes: Vec<C> = nodes.analyze_type_scope(env)?;
    let nodes: Vec<D> = nodes.resolve_type_names(env)?;
    let nodes: Vec<E> = nodes.analyze_scope(env)?;
    let nodes: Vec<F> = nodes.resolve_names(env)?;
    Ok(nodes)
}
