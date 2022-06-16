
use cupid_util::InvertOption;
use crate::{flow_checking as prev_pass, PassResult, util, Env};

#[cupid_semantics::auto_implement(Vec, Option, Str, Box)]
pub trait Lint<Output> where Self: Sized {
    fn lint(self, env: &mut Env) -> PassResult<Output>;
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
    impl Lint + lint for {
        Block<Expr> => prev_pass::Expr;
        Expr => prev_pass::Expr;
        Field<Ident> => crate::Ident;
        Ident => crate::Ident;
        Value => crate::Value;
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

impl Lint<TypeDef> for prev_pass::TypeDef {
    fn lint(self, env: &mut Env) -> PassResult<TypeDef> {
        todo!()
    }
}