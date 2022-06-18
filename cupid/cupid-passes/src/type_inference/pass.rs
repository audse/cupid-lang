use cupid_types::infer::Infer;
use cupid_util::InvertOption;
use crate::{name_resolution as prev_pass, PassResult, util, Env, Value, Address, AsNode, Ident};

/// Stores the inferred type of a node in the environment, accessible by the node's `source` attribute
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
        env.symbols.set_type(self.source(), self.infer());

        // create new decl
        let Self { ident_address, type_annotation_address, attr } = self;
        let decl = Decl { ident_address, type_annotation_address, attr };

        // do pass on stored value
        env.modify_symbol(ident_address, |env, value: crate::PassExpr| {
            let expr: prev_pass::Expr = value.try_into()?;
            let new_value = expr.infer_types(env)?;
            Ok(crate::PassExpr::TypeInferred(new_value))
        })?;

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
        let Self { generics, address, attr, ..} = self;
        let value = env.symbols.get_symbol(address.unwrap()).unwrap();
        let typ = env.symbols.get_type(value.source()).unwrap().clone();
        env.symbols.set_type(attr.source, typ);
        Ok(Self { generics: generics.infer_types(env)?, address, attr, ..self})
    }
}

impl InferTypes<Value> for Value {
    fn infer_types(self, env: &mut Env) -> PassResult<Value> {
        env.symbols.set_type(self.source(), self.infer());
        Ok(self)
    }
}