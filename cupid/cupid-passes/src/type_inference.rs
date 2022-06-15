
use cupid_util::InvertOption;
use crate::{name_resolution as prev_pass, PassResult, util, env::environment::Env};

#[cupid_semantics::auto_implement(Vec, Option)]
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
    Ident: cupid_util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub IdentBuilder => pub Ident {}
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
        Field<Ident> => prev_pass::Ident;
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

impl InferTypes<Ident> for prev_pass::Ident {
    fn infer_types(self, env: &mut Env) -> PassResult<Ident> {
        todo!()
    }
}

impl InferTypes<TypeDef> for prev_pass::TypeDef {
    fn infer_types(self, env: &mut Env) -> PassResult<TypeDef> {
        todo!()
    }
}