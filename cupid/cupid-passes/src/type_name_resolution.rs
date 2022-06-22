
use cupid_util::{InvertOption, Bx, ERR_ALREADY_DEFINED};

use crate::{type_scope_analysis as prev_pass, AsNode, PassResult, Env, Address, Field, util, Query, Type, env::SymbolType};

#[auto_implement::auto_implement(Vec, Option, Str, Box)]
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
            pub ident_address: Address,
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
        env.inside_closure(self.scope(), |env| {
            let query = Query::<Expr>::select(&self.ident).write(Type::typ());

            let fields = self.fields
                .into_iter()
                .map(|mut field| {

                    // find the field's type (if provided)
                    let type_address = field.1
                        .map(|typ| Ok(*env.read::<Address>(&Query::select(typ))?))
                        .invert()?;
                    
                    let field_type = type_address
                        .map(|typ| SymbolType::Address(typ))
                        .or(Some(SymbolType::Type(Type::typ())))
                        .unwrap();

                    // use the current type as a namespace
                    field.0.namespace = Some(self.ident.clone().bx());

                    // make sure ident doesn't exist in scope
                    if env.read::<Address>(&Query::select(&field.0)).is_err() {

                        // create an address for the field's identifier
                        let ident_query = Query::<()>::select(field.0).write(field_type);
                        let ident_address = env.insert(ident_query);

                        Ok(Field(ident_address, type_address))
                    } else {
                        Err(field.0.err(ERR_ALREADY_DEFINED))
                    }

                })
                .collect::<PassResult<Vec<Field<Address>>>>()?;

            Ok(TypeDef::build()
                .ident_address(env.insert(query))
                .fields(fields)
                .build())
        })
    }
}