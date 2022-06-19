use cupid_types::infer::Infer;
use cupid_util::{InvertOption, ERR_NOT_FOUND};
use crate::{name_resolution as prev_pass, PassResult, util, Env, Value, Address, AsNode, Ident, env::SymbolType, Query};

/// Stores the inferred type of a node in the environment database
#[cupid_semantics::auto_implement(Vec, Option, Box)]
pub trait InferTypes<Output> where Self: Sized {
    fn infer_types(self, env: &mut Env) -> PassResult<Output>;
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
        pub FunctionBuilder => pub Function {}
    }
    TypeDef: util::completed_node! { prev_pass::TypeDef => InferTypes<infer_types> }
}

crate::util::impl_default_passes! {
    impl InferTypes + infer_types for {
        Block<Expr> => Block<prev_pass::Expr>;
        Expr => prev_pass::Expr;
        crate::Field<Address>;
    }
}

impl InferTypes<Decl> for prev_pass::Decl {
    fn infer_types(self, env: &mut Env) -> PassResult<Decl> {

        // set type of decl node
        env.write::<Expr>(Query::build()
            .address(self.address())
            .typ(self.infer()))?;

        // create new decl
        let Self { ident_address, type_annotation_address, attr } = self;
        let decl = Decl { ident_address, type_annotation_address, attr };


        // do pass on stored value
        env.write_pass::<Expr, _, prev_pass::Expr>(
            Query::<Expr>::build().address(ident_address), 
            |env, prev_expr| {
                prev_expr.infer_types(env)
            }
        )?;

        Ok(decl)
    }
}

impl InferTypes<Function> for prev_pass::Function {
    fn infer_types(self, env: &mut Env) -> PassResult<Function> {
        todo!()
    }
}

impl InferTypes<Ident> for Ident {
    fn infer_types(self, env: &mut Env) -> PassResult<Ident> {

        // get the value corresponding to the current identifier
        let value = env
            .read::<Option<Expr>>(&Query::from(&self))? // make sure `read` worked
            .as_ref() 
            .ok_or(self.err(ERR_NOT_FOUND))?; // unwrap `Expr`
        
        // find the value's row and get its type
        let value_type = env
            .read::<SymbolType>(&Query::from(value.address()))?;
        
        // write the type to the current identifier's row
        let query = Query::from(self.attr.address).typ(value_type.clone());
        env.write(query)?;

        Ok(Self { generics: self.generics.infer_types(env)?, attr: self.attr, ..self})
    }
}

impl InferTypes<Value> for Value {
    fn infer_types(self, env: &mut Env) -> PassResult<Value> {
        let query = Query::from(self.address()).typ(self.infer());
        env.write(query)?;
        Ok(self)
    }
}