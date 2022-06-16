
use cupid_util::InvertOption;

use crate::{type_scope_analysis as prev_pass, PassResult, Env, Address, Field, util};

#[cupid_semantics::auto_implement(Vec, Option, Str, Box)]
pub trait ResolveTypeNames<Output> where Self: Sized {
    fn resolve_type_names(self, env: &mut Env) -> PassResult<Output>;
}

util::define_pass_nodes! {
    Decl: util::reuse_node! { 
        prev_pass::Decl => ResolveTypeNames<Decl, resolve_type_names> 
    }
    Function: util::reuse_node! { 
        prev_pass::Function => ResolveTypeNames<Function, resolve_type_names> 
    }
    TypeDef: cupid_util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub TypeDefBuilder => pub TypeDef {
            pub ident: Address,
            pub fields: Vec<Field<Address>>,
        }
    }
}

util::impl_default_passes! {
    impl ResolveTypeNames + resolve_type_names for {
        Block<Expr> => prev_pass::Expr;
        Expr => prev_pass::Expr;
        Field<Ident> => crate::Ident;
        Ident => crate::Ident;
        IsTyped<Ident> => crate::IsTyped<crate::Ident>;
        Value => crate::Value;
    }
}

impl ResolveTypeNames<TypeDef> for prev_pass::TypeDef {
    fn resolve_type_names(self, env: &mut Env) -> PassResult<TypeDef> {
        // let pass = self.pass(env)?;
        // Ok(pass)
        todo!()
    }
}