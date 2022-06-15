use cupid_util::{node_builder, InvertOption};

use crate::{PassResult, scope_analysis as prev_pass, Address, Env};

#[cupid_semantics::auto_implement(Vec, Option, Str)]
pub trait ResolveNames<Output> where Self: Sized {
    fn resolve_names(self, env: &mut Env) -> PassResult<Output>;
}

crate::util::define_pass_nodes! {
    Decl: node_builder! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl {
            pub ident_address: Address,
            pub type_annotation_address: Option<Address>,
            pub value: Box<Expr>,
        }
    }
    Function: node_builder! {
        #[derive(Debug, Default, Clone)]
        pub FunctionBuilder => pub Function {
            pub params: Vec<Decl>,
            pub return_type_annotation: Option<Address>,
            pub body: Vec<Expr>,
        }
    }
    TypeDef: node_builder! {
        #[derive(Debug, Default, Clone)]
        pub TypeDefBuilder => pub TypeDef {
            pub ident: Address,
            pub fields: Vec<crate::Field<Address>>,
        }
    }
}

crate::util::impl_default_passes! {
    impl ResolveNames + resolve_names for {
        Block<Expr> => prev_pass::Expr;
        Expr => prev_pass::Expr;
        Field<Ident> => crate::Ident;
        Ident => crate::Ident;
        Value => crate::Value;
    }
}

impl ResolveNames<Decl> for prev_pass::Decl {
    fn resolve_names(self, env: &mut Env) -> PassResult<Decl> {
        todo!()
    }
}

impl ResolveNames<Function> for prev_pass::Function {
    fn resolve_names(self, env: &mut Env) -> PassResult<Function> {
        todo!()
    }
}

impl ResolveNames<TypeDef> for prev_pass::TypeDef {
    fn resolve_names(self, env: &mut Env) -> PassResult<TypeDef> {
        todo!()
    }
}
