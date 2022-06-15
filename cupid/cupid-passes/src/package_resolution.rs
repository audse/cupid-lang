
use cupid_util::{InvertOption, Bx};
use crate::{
    PassResult,
    util::reuse_node,
    pre_analysis as prev_pass,
    Env
};

#[cupid_semantics::auto_implement(Vec, Option, Str)]
pub trait ResolvePackages<Output> where Self: Sized {
    fn resolve_packages(self, env: &mut Env) -> PassResult<Output>;
}

// TODO allow non-top-level package resolution
crate::util::define_pass_nodes! {
    Decl: reuse_node! { 
        prev_pass::Decl => ResolvePackages<Decl, resolve_packages> 
    }
    Function: reuse_node! { 
        prev_pass::Function => ResolvePackages<Function, resolve_packages> 
    }
    TypeDef: reuse_node! { 
        prev_pass::TypeDef => ResolvePackages<TypeDef, resolve_packages> 
    }
}


crate::util::impl_default_passes! {
    impl ResolvePackages + resolve_packages for {
        Block<Expr> => prev_pass::Expr;
        Expr => prev_pass::Expr;
        Field<Ident> => crate::Ident;
        Ident => crate::Ident;
        Value => crate::Value;
    }
}