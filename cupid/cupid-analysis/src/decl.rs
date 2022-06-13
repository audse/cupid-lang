use crate::semantic_state::*;
use crate::*;

#[cupid_semantics::semantic_states]
#[derive(Debug, Default, Clone)]
pub struct Decl {
    pre_analysis: pre_analysis::Decl,
    names_resolved: names_resolved::Decl,
    types_inferred: types_inferred::Decl,
    types_checked: types_checked::Decl,
}

pub mod pre_analysis {
    use super::*;
    build_struct! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl { 
            pub name: SemanticNode<Id>,
            pub type_annotation: Option<SemanticNode<Id>>,
            pub value: Box<SemanticNode<Expr>>,
            pub mutable: bool
        }
    }
}

pub mod names_resolved {
    use super::*;
    build_struct! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl {
            pub name_address: Address,
            pub type_address: Option<Address>,
            pub value: Box<SemanticNode<Expr>>,
        }
    }
}

pub mod types_inferred {
    use super::*;
    build_struct! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl {
            pub name_address: Address,
            pub type_address: Option<Address>,
            pub inferred_type_address: Address,
            pub value: Box<SemanticNode<Expr>>
        }
    }
}

pub mod types_checked {
    use super::*;
    build_struct! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl {
            pub name_address: Address,
            pub value: Box<SemanticNode<Expr>>,
        }
    }
}

impl Analysis for SemanticNode<Decl> {
    fn analyze_scopes(mut self, env: &mut Env) -> Result<Self, ErrCode> {
        let mut node = self.data.get_pre_analysis()?;
        node.name = node.name.analyze_scopes(env)?;
        node.type_annotation = node.type_annotation.analyze_scopes(env)?;
        node.value = node.value.analyze_scopes(env)?.bx();
        self.closure = env.current_closure;
        self.data = Decl(NodeState::ScopeAnalyzed(node));
        Ok(self)
    }
    fn resolve_names(mut self, env: &mut Env) -> Result<Self, ErrCode> {
        let node = self.data.get_scopes_analyzed()?;
        let name = &node.name.data.get_scopes_analyzed()?;
        let type_annotation: Option<Ident> = node.type_annotation.map(|t| t.data.get_scopes_analyzed()).invert()?;
        let new_node = names_resolved::Decl {
            name_address: env.set_address(name).expect("no address!"),
            type_address: type_annotation.map(|t| env.get_address(&t)).invert().expect("no type address!"),
            value: node.value.resolve_names(env)?.bx(),
        };
        self.data = Decl(NodeState::NamesResolved(new_node));
        Ok(self)
    }
}