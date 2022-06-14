use cupid_scope::Env;
use cupid_util::InvertOption;
use crate::PassResult;

use crate::pre_analysis as prev_pass;

#[cupid_semantics::auto_implement(Vec, Option)]
pub trait ResolvePackages<T> where Self: Sized {
    fn resolve_packages(self, env: &mut Env) -> PassResult<T>;
}

crate::ast_pass_nodes! {
    Decl: crate::skip_pass! { Decl = prev_pass + ResolvePackages<Decl> resolve_packages }
    Function: crate::skip_pass! { Function = prev_pass + ResolvePackages<Function> resolve_packages }
    Ident: crate::skip_pass! { Ident = prev_pass + ResolvePackages<Ident> resolve_packages }
}

crate::impl_expr_ast_pass! {
    impl ResolvePackages<Expr> for prev_pass::Expr { resolve_packages }
}

crate::impl_block_ast_pass! {
    impl ResolvePackages<crate::Block<Expr>> for crate::Block<prev_pass::Expr> { resolve_packages }
}