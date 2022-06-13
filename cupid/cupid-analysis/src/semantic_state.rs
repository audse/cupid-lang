use crate::*;

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
}

type Address = usize;
type Source = usize;
type Closure = usize;

pub struct SemanticNode<T> {
	pub data: T,
	pub source: Source,
	pub closure: Closure,
}

pub enum Expr {
    Decl(Decl),
    Id(Id),
}

impl Analysis for SemanticNode<Expr> {}

pub struct PreAnalysisDecl {
	pub name: SemanticNode<Id>,
	pub type_annotation: Option<SemanticNode<Id>>,
	pub value: Box<SemanticNode<Expr>>,
	pub mutable: bool
}

pub struct NamesResolvedDecl {
    pub name_address: Address,
    pub type_address: Option<Address>,
    pub value: Box<SemanticNode<Expr>>,
}

#[cupid_semantics::semantic_states]
pub struct Decl {
    pre_analysis: PreAnalysisDecl,
    names_resolved: NamesResolvedDecl,
}

#[cupid_semantics::semantic_states]
pub struct Id {
    pre_analysis: Ident,
}

impl Analysis for SemanticNode<Id> {}

impl Analysis for SemanticNode<Decl> {
    fn analyze_scopes(mut self, env: &mut Env) -> Result<Self, ErrCode> {
        let mut node: PreAnalysisDecl = self.data.0.get_pre_analysis()?;
        node.name = node.name.analyze_scopes(env)?;
        node.type_annotation = node.type_annotation.map(|t| t.analyze_scopes(env)).invert()?;
        node.value = Box::new(node.value.analyze_scopes(env)?);
        self.closure = env.current_closure;
        self.data = Decl(NodeState::ScopeAnalyzed(node));
        Ok(self)
    }
    fn resolve_names(mut self, env: &mut Env) -> Result<Self, ErrCode> {
        let node: PreAnalysisDecl = self.data.0.get_scope_analyzed()?;
        let name: &Ident = &node.name.data.0.get_scope_analyzed()?;
        let type_annotation: Option<Ident> = node.type_annotation.map(|t| t.data.0.get_scope_analyzed()).invert()?;
        let new_node = NamesResolvedDecl {
            name_address: env.set_address(name).expect("no address!"),
            type_address: type_annotation.map(|t| env.get_address(&t)).invert().expect("no type address!"),
            value: Box::new(node.value.resolve_names(env)?),
        };
        self.data = Decl(NodeState::NamesResolved(new_node));
        Ok(self)
    }
}

use NodeState::*;

pub trait GetNode<A, B, C, D, E, F, G, H, I> {
    fn node(self) -> NodeState<A, B, C, D, E, F, G, H, I>;
}

impl<A, B, C, D, E, F, G, H, I> NodeState<A, B, C, D, E, F, G, H, I> {
    fn get_pre_analysis(self) -> Result<A, ErrCode> {
        match self {
            PreAnalysis(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    fn get_package_resolved(self) -> Result<B, ErrCode> {
        match self {
            PackageResolved(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    fn get_type_names_resolved(self) -> Result<C, ErrCode> {
        match self {
            TypeNamesResolved(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    fn get_scope_analyzed(self) -> Result<D, ErrCode> {
        match self {
            ScopeAnalyzed(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    fn get_names_resolved(self) -> Result<E, ErrCode> {
        match self {
            NamesResolved(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    fn get_type_inferred(self) -> Result<F, ErrCode> {
        match self {
            TypesInferred(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    fn get_type_checked(self) -> Result<G, ErrCode> {
        match self {
            TypesChecked(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    fn get_flow_checked(self) -> Result<H, ErrCode> {
        match self {
            FlowChecked(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
    fn get_linted(self) -> Result<I, ErrCode> {
        match self {
            Linted(data) => Ok(data),
            _ => Err(ERR_UNREACHABLE)
        }
    }
}