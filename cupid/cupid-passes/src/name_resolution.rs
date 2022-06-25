use cupid_util::InvertOption;
use crate::{PassResult, scope_analysis as prev_pass, Env, Address, Ident, util, AsNode, Query};

#[auto_implement::auto_implement(Vec, Option, Str, Box)]
pub trait ResolveNames<Output> where Self: Sized {
    fn resolve_names(self, env: &mut Env) -> PassResult<Output>;
}

util::define_pass_nodes! {
    Decl: util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl {
            pub ident_address: Address,
            pub type_annotation_address: Option<Address>,
        }
    }
    Function: util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub FunctionBuilder => pub Function {
            pub params: Vec<Decl>,
            pub return_type_annotation_address: Option<Address>,
            pub body: Vec<Expr>,
        }
    }
    TypeDef: util::completed_node! { prev_pass::TypeDef => ResolveNames<resolve_names> }
}

util::impl_default_passes! {
    impl ResolveNames + resolve_names for {
        Block<Expr> => Block<prev_pass::Expr>;
        Expr => prev_pass::Expr;
        crate::Field<Address>;
        crate::Value;
    }
}

impl ResolveNames<Decl> for prev_pass::Decl {
    #[trace::trace]
    fn resolve_names(self, env: &mut Env) -> PassResult<Decl> {
        env.inside_closure(self.attr.scope, |env| {
            let Self { ident, type_annotation, mutable, value, attr } = self;

            // Make sure the current symbol does not already exist
            if env.read::<Address>(&Query::select(&ident)).is_err() {

                // If a type annotation is provided, make sure it exists
                let type_annotation_address = type_annotation
                    .map(|t| env.read::<Address>(&Query::select(&t)))
                    .invert()?
                    .map(|a| *a);

                let query = Query::<Expr>::select(ident)
                    .write_expr(*value.resolve_names(env)?)
                    .write(mutable);
                
                Ok(Decl::build()
                    .ident_address(env.insert(query))
                    .type_annotation_address(type_annotation_address)
                    .attr(attr)
                    .build())
            } else {
                Err((attr.address, cupid_util::ERR_ALREADY_DEFINED))
            }
        })
    }
}

impl ResolveNames<Function> for prev_pass::Function {
    #[trace::trace]
    fn resolve_names(self, env: &mut Env) -> PassResult<Function> {
        env.inside_closure(self.attr.scope, |env| {
            let Self { params, body, return_type_annotation, attr } = self;

            let return_type_annotation = return_type_annotation
                .resolve_names(env)?
                .map(|ident| env.read::<Address>(&Query::select(&ident)))
                .invert()?
                .map(|address| *address);

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
    #[trace::trace]
    fn resolve_names(self, env: &mut Env) -> PassResult<Ident> {
        env.inside_closure(self.attr.scope, |env| {
            // if there is a namespace
            if let Some(namespace) = &self.namespace {
                // make sure the namespaced symbol exists

                let query = Query::select(&**namespace);
                let namespace_address = env.read::<Address>(&query)?;

                // use the namespace's scope
                let name_scope = env.database.read::<Ident>(&Query::<()>::select(*namespace_address).read).unwrap().scope();
                env.inside_closure(name_scope, |env| {
                    resolve_ident_names(self, env)
                })
            } else {
                resolve_ident_names(self, env)
            }
        })
    }
}

fn resolve_ident_names(ident: Ident, env: &mut Env) -> PassResult<Ident> {
    use crate::{Value::VType, Type};

    // make sure symbol exists in scope
    let address = *env.read::<Address>(&Query::select(&ident))?;
    eprintln!("{ident:?} exists");
    
    let Ident { generics, attr, ..} = ident;

    // create symbols for provided generics
    for generic in generics.iter() {
        // if generic exists, resolve it
        if let Ok(generic_address) = env.read::<Address>(&Query::select(&*generic)) {
            let generic_address = *generic_address;
            env.write_pass::<Expr, _, prev_pass::Expr>(
                Query::<Expr>::select(generic_address),
                |env, prev_expr| prev_expr.resolve_names(env)
            )?;

        // otherwise, create it
        } else {
            let expr: Expr = VType(Type::generic(generic.clone())).into();
            let query = Query::<Expr>::select(generic.clone())
                .write(Type::typ())
                .write_expr(expr);
            env.insert(query);
        }
    }
    Ok(Ident { generics, attr, ..ident })
}
