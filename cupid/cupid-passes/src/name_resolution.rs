use cupid_util::{node_builder, InvertOption, Bx};

use crate::{PassResult, scope_analysis as prev_pass, Address, Env, IsTyped, Ident, Field};

#[cupid_semantics::auto_implement(Vec, Option, Str, Box)]
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
            pub return_type_annotation_address: Option<Address>,
            pub body: Vec<Expr>,
        }
    }
    TypeDef: node_builder! {
        #[derive(Debug, Default, Clone)]
        pub TypeDefBuilder => pub TypeDef {
            pub ident: Address,
            pub fields: Vec<Field<Address>>,
        }
    }
}

crate::util::impl_default_passes! {
    impl ResolveNames + resolve_names for {
        Block<Expr> => prev_pass::Expr;
        Expr => prev_pass::Expr;
        Field<Ident> => Ident;
        Value => crate::Value;
    }
}

impl ResolveNames<Decl> for prev_pass::Decl {
    fn resolve_names(self, env: &mut Env) -> PassResult<Decl> {
        env.inside_scope(self.attr.scope, |env| {
            let Self { ident, type_annotation, mutable, value, attr } = self;

            // Make sure the current symbol does not already exist
            if env.get_symbol(&ident).is_err() {

                // If a type annotation is provided, make sure it exists
                let type_annotation_address = type_annotation
                    .map(|t| env.get_symbol(&t))
                    .invert()?;

                let value = value.resolve_names(env)?;
                let symbol_value = crate::SymbolValue { 
                    value: crate::PassExpr::default().bx(),
                    mutable,
                    attr: ident.attr,
                };

                Ok(Decl::build()
                    .ident_address(env.set_symbol(ident, symbol_value))
                    .type_annotation_address(type_annotation_address)
                    .value(value)
                    .attr(attr)
                    .build())
            } else {
                Err((attr.source, cupid_util::ERR_ALREADY_DEFINED))
            }
        })
    }
}

impl ResolveNames<Function> for prev_pass::Function {
    fn resolve_names(self, env: &mut Env) -> PassResult<Function> {
        env.inside_scope(self.attr.scope, |env| {
            let Self { params, body, return_type_annotation, attr } = self;
            let return_type_annotation = return_type_annotation
                .resolve_names(env)?
                .map(|t| t.address)
                .flatten();
            Ok(Function::build()
                .params(params.resolve_names(env)?)
                .body(body.resolve_names(env)?)
                .return_type_annotation_address(return_type_annotation)
                .attr(attr)
                .build())
        })
    }
}

impl ResolveNames<Ident> for Ident {
    fn resolve_names(self, env: &mut Env) -> PassResult<Ident> {
        env.inside_scope(self.attr.scope, |env| {
            // if there is a namespace
            if let Some(namespace) = &self.namespace {
                // make sure the namespaced symbol exists
                let namespace_address = env.get_symbol(&namespace)?;

                // use the namespace's scope
                let name_scope = env.symbols.get_symbol(namespace_address).unwrap().attr.scope;
                env.inside_scope(name_scope, |env| {
                    resolve_ident_names(self, env)
                })
            } else {
                resolve_ident_names(self, env)
            }
        })
    }
}

fn resolve_ident_names(ident: Ident, env: &mut Env) -> PassResult<Ident> {
    // make sure symbol exists in scope
    let address = env.get_symbol(&ident)?; 
    
    let Ident { generics, attr, ..} = ident;
    let generics = generics.resolve_names(env)?;

    // create symbols for provided generics
    for generic in generics.iter() {
        if let crate::Untyped(ident) = generic {
            let value = crate::SymbolValue { 
                attr: ident.attr,
                ..Default::default()
            };
            env.set_symbol(ident.clone(), value);
        }
    }
    Ok(Ident { generics, address: Some(address), attr, ..ident })
}

impl ResolveNames<IsTyped<Ident>> for IsTyped<Ident> {
    fn resolve_names(self, env: &mut Env) -> PassResult<IsTyped<Ident>> {
        use crate::{Typed, Untyped};
        let generic = match self {
            Typed(ident, type_address) => Typed(ident.resolve_names(env)?, type_address),
            Untyped(ident) => match env.get_symbol(&ident) {
                Ok(type_address) => Typed(ident, type_address),
                Err(_) => Untyped(ident) // assumed to be generic type
                // TODO this should probably still fail- are generics in scope?
            }
        };
        Ok(generic)
    }
}

impl ResolveNames<TypeDef> for prev_pass::TypeDef {
    fn resolve_names(self, env: &mut Env) -> PassResult<TypeDef> {
        todo!()
    }
}
