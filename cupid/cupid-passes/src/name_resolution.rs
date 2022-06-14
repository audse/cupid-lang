use cupid_util::{node_builder, InvertOption};
use cupid_scope::Env;
use crate::{PassResult, scope_analysis};

#[cupid_semantics::auto_implement(Vec, Option)]
pub trait ResolveNames<T> where Self: Sized {
    fn resolve_names(self, env: &mut Env) -> PassResult<T>;
}

crate::ast_pass_nodes! {
    Decl: node_builder! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl {
            pub ident_address: crate::Address,
            pub type_annotation_address: Option<crate::Address>,
            pub value: Box<Expr>,
        }
    }
    Function: node_builder! {
        #[derive(Debug, Default, Clone)]
        pub FunctionBuilder => pub Function {
            pub params: Vec<Decl>,
            pub return_type_annotation: Option<crate::Address>,
            pub body: Vec<Expr>,
        }
    }
    Ident: node_builder! {
        #[derive(Debug, Default, Clone)]
        pub IdentBuilder => pub Ident {
            pub namespace: crate::Address,
            pub name: crate::Address,
            pub generics: Vec<crate::Address>
        }
    }
}

crate::impl_expr_ast_pass! {
    impl ResolveNames<Expr> for scope_analysis::Expr { 
        resolve_names
    }
}

crate::impl_block_ast_pass! {
    impl ResolveNames<crate::Block<Expr>> for crate::Block<scope_analysis::Expr> { 
        resolve_names 
    }
}

impl ResolveNames<Decl> for scope_analysis::Decl {
    fn resolve_names(self, env: &mut Env) -> PassResult<Decl> {
        todo!()
    }
}

impl ResolveNames<Function> for scope_analysis::Function {
    fn resolve_names(self, env: &mut Env) -> PassResult<Function> {
        todo!()
    }
}

impl ResolveNames<Ident> for scope_analysis::Ident {
    fn resolve_names(self, env: &mut Env) -> PassResult<Ident> {
        todo!()
    }
}
