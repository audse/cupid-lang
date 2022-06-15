
use cupid_util::InvertOption;
use crate::{PassResult, type_checking as prev_pass, util, env::environment::Env};

#[cupid_semantics::auto_implement(Vec, Option, Str)]
pub trait CheckFlow<Output> where Self: Sized {
    fn check_flow(self, env: &mut Env) -> PassResult<Output>;
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
    impl CheckFlow + check_flow for {
        Block<Expr> => prev_pass::Expr;
        Expr => prev_pass::Expr;
        Field<Ident> => prev_pass::Ident;
        Value => crate::Value;
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

impl CheckFlow<Ident> for prev_pass::Ident {
    fn check_flow(self, env: &mut Env) -> PassResult<Ident> {
        todo!()
    }
}

impl CheckFlow<TypeDef> for prev_pass::TypeDef {
    fn check_flow(self, env: &mut Env) -> PassResult<TypeDef> {
        todo!()
    }
}