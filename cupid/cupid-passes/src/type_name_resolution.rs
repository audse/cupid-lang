
use cupid_util::InvertOption;

use crate::{type_scope_analysis as prev_pass, PassResult, Env, Address, Field, Ident, util};

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
    TypeDef: crate::util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub TypeDefBuilder => pub TypeDef {
            pub ident: Address,
            pub fields: Vec<Field<Address>>,
        }
    }
}

util::impl_default_passes! {
    impl ResolveTypeNames + resolve_type_names for {
        Block<Expr> => Block<prev_pass::Expr>;
        Expr => prev_pass::Expr;
        crate::Ident;
        crate::Value;
    }
}

impl ResolveTypeNames<TypeDef> for prev_pass::TypeDef {
    fn resolve_type_names(self, env: &mut Env) -> PassResult<TypeDef> {
        let ident = self.ident.resolve_type_names(env)?;

        todo!("insert type")
        // let ident_address = env.insert(Query::from(&ident));

        // Ok(TypeDef::build()
        //     .ident(self.ident.resolve_type_names(env)?)
        //     .fields(self.fields
        //         .resolve_type_names(env)?
        //         .into_iter()
        //         .map(|f: Field<Ident>| Field(f.0.address.unwrap(), f.1.map(|f| f.address.unwrap())))
        //         .collect())
        //     .build())
    }
}

impl ResolveTypeNames<Field<Ident>> for Field<Ident> {
    fn resolve_type_names(self, env: &mut Env) -> PassResult<Field<Ident>> {
        // let pass = self.pass(env)?;
        // Ok(pass)
        todo!()
    }
}