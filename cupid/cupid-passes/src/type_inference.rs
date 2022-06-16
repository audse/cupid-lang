
use cupid_util::InvertOption;
use crate::{name_resolution as prev_pass, PassResult, util, Env};

#[cupid_semantics::auto_implement(Vec, Option, Box)]
pub trait InferTypes<Output> where Self: Sized {
    fn infer_types(self, env: &mut Env) -> PassResult<Output>;
}

util::define_pass_nodes! {
    Decl: cupid_util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl {}
    }
    Function: cupid_util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub FunctionBuilder => pub Function {}
    }
    TypeDef: cupid_util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub TypeDefBuilder => pub TypeDef {}
    }
}

crate::util::impl_default_passes! {
    impl InferTypes + infer_types for {
        Block<Expr> => prev_pass::Expr;
        Expr => prev_pass::Expr;
        Field<Ident> => crate::Ident;
        Ident => crate::Ident;
        Value => crate::Value;
    }
}

impl InferTypes<Decl> for prev_pass::Decl {
    fn infer_types(self, env: &mut Env) -> PassResult<Decl> {
        todo!()
    }
}

impl InferTypes<Function> for prev_pass::Function {
    fn infer_types(self, env: &mut Env) -> PassResult<Function> {
        todo!()
    }
}

impl InferTypes<TypeDef> for prev_pass::TypeDef {
    fn infer_types(self, env: &mut Env) -> PassResult<TypeDef> {
        todo!()
    }
}