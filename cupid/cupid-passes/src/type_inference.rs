use cupid_scope::Env;
use cupid_util::InvertOption;
use crate::PassResult;
use cupid_util::node_builder;

use crate::name_resolution as prev_pass;

#[cupid_semantics::auto_implement(Vec, Option)]
pub trait InferTypes<T> where Self: Sized {
    fn infer_types(self, env: &mut Env) -> PassResult<T>;
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
    impl InferTypes<Expr> for prev_pass::Expr { infer_types }
}

crate::impl_block_ast_pass! {
    impl InferTypes<crate::Block<Expr>> for crate::Block<prev_pass::Expr> { infer_types }
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