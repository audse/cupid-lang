
use cupid_util::InvertOption;
use crate::{flow_checking as prev_pass, PassResult, util, Env};

#[cupid_semantics::auto_implement(Vec, Option, Str, Box)]
pub trait Lint<Output> where Self: Sized {
    fn lint(self, env: &mut Env) -> PassResult<Output>;
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
    TypeDef: util::completed_node! { prev_pass::TypeDef => Lint<lint> }
}

crate::util::impl_default_passes! {
    impl Lint + lint for {
        Block<Expr> => Block<prev_pass::Expr>;
        Expr => prev_pass::Expr;
        crate::Field<Address>;
        crate::Ident;
        crate::IsTyped<Ident>;
        crate::Value;
    }
}

/// Lint ideas
/// 1. Unused declaration
impl Lint<Decl> for prev_pass::Decl {
    fn lint(self, env: &mut Env) -> PassResult<Decl> {
        todo!()
    }
}

impl Lint<Function> for prev_pass::Function {
    fn lint(self, env: &mut Env) -> PassResult<Function> {
        todo!()
    }
}