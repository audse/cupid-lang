
use cupid_util::{InvertOption, Bx};

use crate::{package_resolution as prev_pass, PassResult, env::environment::Env};

#[cupid_semantics::auto_implement(Vec, Option, Str)]
pub trait ResolveTypeNames<Output> where Self: Sized {
    fn resolve_type_names(self, env: &mut Env) -> PassResult<Output>;
}

crate::util::define_pass_nodes! {
    Decl: crate::util::reuse_node! { 
        prev_pass::Decl => ResolveTypeNames<Decl, resolve_type_names> 
    }
    Function: crate::util::reuse_node! { 
        prev_pass::Function => ResolveTypeNames<Function, resolve_type_names> 
    }
    Ident: crate::util::reuse_node! { 
        prev_pass::Ident => ResolveTypeNames<Ident, resolve_type_names> 
    }
    TypeDef: crate::util::reuse_node! { 
        prev_pass::TypeDef => Pass<resolve_type_names> 
    }
}

crate::util::impl_default_passes! {
    impl ResolveTypeNames + resolve_type_names for {
        Block<Expr> => prev_pass::Expr;
        Expr => prev_pass::Expr;
        Field<Ident> => prev_pass::Ident;
        Value => crate::Value;
    }
}

impl ResolveTypeNames<TypeDef> for prev_pass::TypeDef {
    fn resolve_type_names(self, env: &mut Env) -> PassResult<TypeDef> {
        let pass = self.pass(env)?;
        Ok(pass)
    }
}