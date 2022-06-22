
use cupid_util::InvertOption;
use crate::{PassResult, type_checking as prev_pass, util, Env};

#[auto_implement::auto_implement(Vec, Option, Str, Box)]
pub trait CheckFlow<Output> where Self: Sized {
    fn check_flow(self, env: &mut Env) -> PassResult<Output>;
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
    TypeDef: util::completed_node! { prev_pass::TypeDef => CheckFlow<check_flow> }
}

crate::util::impl_default_passes! {
    impl CheckFlow + check_flow for {
        Block<Expr> => Block<prev_pass::Expr>;
        Expr => prev_pass::Expr;
        crate::Field<Address>;
        crate::Ident;
        crate::Value;
    }
}

impl CheckFlow<Decl> for prev_pass::Decl {
    fn check_flow(self, env: &mut Env) -> PassResult<Decl> {
        todo!()
    }
}

impl CheckFlow<Function> for prev_pass::Function {
    fn check_flow(self, env: &mut Env) -> PassResult<Function> {
        todo!()
    }
}