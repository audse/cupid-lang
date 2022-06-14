use cupid_scope::Env;
use cupid_util::InvertOption;
use crate::PassResult;
use cupid_util::node_builder;

use crate::type_inference as prev_pass;

#[cupid_semantics::auto_implement(Vec, Option)]
pub trait CheckTypes<T> where Self: Sized {
    fn check_types(self, env: &mut Env) -> PassResult<T>;
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
    impl CheckTypes<Expr> for prev_pass::Expr { check_types }
}

crate::impl_block_ast_pass! {
    impl CheckTypes<crate::Block<Expr>> for crate::Block<prev_pass::Expr> { check_types }
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