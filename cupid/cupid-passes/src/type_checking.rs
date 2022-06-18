
use cupid_util::InvertOption;
use crate::{type_inference as prev_pass, util, PassResult, Env};

#[cupid_semantics::auto_implement(Vec, Option, Box)]
pub trait CheckTypes<Output> where Self: Sized {
    fn check_types(self, env: &mut Env) -> PassResult<Output>;
}

util::define_pass_nodes! {
    Decl: crate::util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl {}
    }
    Function: crate::util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub FunctionBuilder => pub Function {}
    }
    TypeDef: util::completed_node! { prev_pass::TypeDef => CheckTypes<check_types> }
}

crate::util::impl_default_passes! {
    impl CheckTypes + check_types for {
        Block<Expr> => Block<prev_pass::Expr>;
        Expr => prev_pass::Expr;
        crate::Field<Address>;
        crate::Ident;
        crate::Value;
    }
}

impl CheckTypes<Decl> for prev_pass::Decl {
    fn check_types(self, env: &mut Env) -> PassResult<Decl> {
        todo!()
    }
}

impl CheckTypes<Function> for prev_pass::Function {
    fn check_types(self, env: &mut Env) -> PassResult<Function> {
        todo!()
    }
}
