use crate::decl::*;
use crate::*;
use NodeState::*;

// https://tomlee.co/2014/04/a-more-detailed-tour-of-the-rust-compiler/
pub enum SemanticPass {
	PreAnalysis,
	PackageResolution,
	TypeNameResolution,
	ScopeAnalysis,
	NameResolution,
	TypeInference,
	TypeChecking,
	FlowChecking,
	Linting,
}

#[allow(unused_variables)]
pub trait Analysis where Self: Sized {
    fn resolve_packages(self, env: &mut Env) -> Result<Self, ErrCode> { Ok(self) }
	fn resolve_type_names(self, env: &mut Env) -> Result<Self, ErrCode> { Ok(self) }
	fn analyze_scopes(self, env: &mut Env) -> Result<Self, ErrCode> { Ok(self) }
    fn resolve_names(self, env: &mut Env) -> Result<Self, ErrCode> { Ok(self) }
    fn infer_types(self, env: &mut Env) -> Result<Self, ErrCode> { Ok(self) }
    fn check_types(self, env: &mut Env) -> Result<Self, ErrCode> { Ok(self) }
    fn check_flow(self, env: &mut Env) -> Result<Self, ErrCode> { Ok(self) }
    fn lint(self, env: &mut Env) -> Result<Self, ErrCode> { Ok(self) }
}

#[derive(Debug, Default, Clone)]
pub enum NodeState<A, B, C, D, E, F, G, H, I> {
	PreAnalysis(A),
	PackageResolved(B),
	TypeNamesResolved(C),
	ScopeAnalyzed(D),
	NamesResolved(E),
	TypesInferred(F),
	TypesChecked(G),
	FlowChecked(H),
	Linted(I),
    
    #[default]
    Empty,
}

pub type Address = usize;
pub type Source = usize;
pub type Closure = usize;

#[derive(Debug, Default, Clone)]
pub struct SemanticNode<T> {
	pub data: T,
	pub source: Source,
	pub closure: Closure,
    pub type_address: Address,
}

#[derive(Debug, Default, Clone)]
pub enum Expr {
    Decl(Decl),
    Id(Id),

    #[default]
    Empty,
}

#[cupid_semantics::semantic_states]
#[derive(Debug, Default, Clone)]
pub struct Id {
    pre_analysis: Ident,
}
impl Analysis for SemanticNode<Id> {}

impl Analysis for SemanticNode<Expr> {}

impl<T> Analysis for Option<SemanticNode<T>> where SemanticNode<T>: Analysis {
    fn analyze_scopes(self, env: &mut Env) -> Result<Self, ErrCode> {
        Ok(if let Some(s) = self { Some(s.analyze_scopes(env)?) } else { None })
    }
}

impl<T> SemanticNode<T> {
    pub fn bx(self) -> Box<Self> { Box::new(self) }
}

impl<A, B, C, D, E, F, G, H, I> NodeState<A, B, C, D, E, F, G, H, I> {
    pub fn get_pre_analysis(self) -> Result<A, ErrCode> {
        match self {
            PreAnalysis(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    pub fn get_package_resolved(self) -> Result<B, ErrCode> {
        match self {
            PackageResolved(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    pub fn get_type_names_resolved(self) -> Result<C, ErrCode> {
        match self {
            TypeNamesResolved(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    pub fn get_scopes_analyzed(self) -> Result<D, ErrCode> {
        match self {
            ScopeAnalyzed(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    pub fn get_names_resolved(self) -> Result<E, ErrCode> {
        match self {
            NamesResolved(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    pub fn get_types_inferred(self) -> Result<F, ErrCode> {
        match self {
            TypesInferred(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    pub fn get_types_checked(self) -> Result<G, ErrCode> {
        match self {
            TypesChecked(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    pub fn get_flow_checked(self) -> Result<H, ErrCode> {
        match self {
            FlowChecked(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    pub fn get_linted(self) -> Result<I, ErrCode> {
        match self {
            Linted(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
}