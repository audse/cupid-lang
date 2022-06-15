
use cupid_util::InvertOption;
use crate::{type_inference as prev_pass, util, PassResult, env::environment::Env};

#[cupid_semantics::auto_implement(Vec, Option)]
pub trait CheckTypes<Output> where Self: Sized {
    fn check_types(self, env: &mut Env) -> PassResult<Output>;
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
    impl CheckTypes + check_types for {
        Block<Expr> => prev_pass::Expr;
        Expr => prev_pass::Expr;
        Field<Ident> => prev_pass::Ident;
        Value => crate::Value;
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

impl CheckTypes<Ident> for prev_pass::Ident {
    fn check_types(self, env: &mut Env) -> PassResult<Ident> {
        todo!()
    }
}

impl CheckTypes<TypeDef> for prev_pass::TypeDef {
    fn check_types(self, env: &mut Env) -> PassResult<TypeDef> {
        todo!()
    }
}