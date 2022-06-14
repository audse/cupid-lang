use cupid_scope::Env;
use cupid_util::InvertOption;
use crate::PassResult;

use crate::package_resolution as prev_pass;

#[cupid_semantics::auto_implement(Vec, Option)]
pub trait ResolveTypeNames<T> where Self: Sized {
    fn resolve_type_names(self, env: &mut Env) -> PassResult<T>;
}

crate::ast_pass_nodes! {
    Decl:
        #[derive(Debug, Default, Clone)]
        pub struct Decl(pub crate::pre_analysis::Decl);
    Function:
        #[derive(Debug, Default, Clone)]
        pub struct Function(pub crate::pre_analysis::Function);
    Ident: 
        #[derive(Debug, Default, Clone)]
        pub struct Ident(pub crate::pre_analysis::Ident);
}

crate::impl_expr_ast_pass! {
    impl ResolveTypeNames<Expr> for prev_pass::Expr { resolve_type_names }
}

crate::impl_block_ast_pass! {
    impl ResolveTypeNames<crate::Block<Expr>> for crate::Block<prev_pass::Expr> { resolve_type_names }
}

impl ResolveTypeNames<Decl> for prev_pass::Decl {
    fn resolve_type_names(self, _: &mut Env) -> PassResult<Decl> {
        Ok(Decl(self.0))
    }
}

impl ResolveTypeNames<Function> for prev_pass::Function {
    fn resolve_type_names(self, _: &mut Env) -> PassResult<Function> {
        Ok(Function(self.0))
    }
}

impl ResolveTypeNames<Ident> for prev_pass::Ident {
    fn resolve_type_names(self, _: &mut Env) -> PassResult<Ident> {
        Ok(Ident(self.0))
    }
}