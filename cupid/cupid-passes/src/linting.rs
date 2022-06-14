use cupid_scope::Env;
use cupid_util::{InvertOption, node_builder};
use crate::PassResult;

use crate::flow_checking as prev_pass;

#[cupid_semantics::auto_implement(Vec, Option)]
pub trait Lint<T> where Self: Sized {
    fn lint(self, env: &mut Env) -> PassResult<T>;
}

crate::ast_pass_nodes! {
    Decl: node_builder! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl {}
    }
    Function: node_builder! {
        #[derive(Debug, Default, Clone)]
        pub FunctionBuilder => pub Function {}
    }
    Ident: node_builder! {
        #[derive(Debug, Default, Clone)]
        pub IdentBuilder => pub Ident {}
    }
}

crate::impl_expr_ast_pass! {
    impl Lint<Expr> for prev_pass::Expr { lint }
}

crate::impl_block_ast_pass! {
    impl Lint<crate::Block<Expr>> for crate::Block<prev_pass::Expr> { lint }
}

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

impl Lint<Ident> for prev_pass::Ident {
    fn lint(self, env: &mut Env) -> PassResult<Ident> {
        todo!()
    }
}