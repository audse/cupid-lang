
use cupid_util::InvertOption;

use crate::{package_resolution as prev_pass, PassResult, Env};

#[cupid_semantics::auto_implement(Vec, Option, Str, Box)]
pub trait AnalyzeTypeScope<Output> where Self: Sized {
    fn analyze_type_scope(self, env: &mut Env) -> PassResult<Output>;
}

crate::util::define_pass_nodes! {
    Decl: crate::util::reuse_node! { 
        prev_pass::Decl => AnalyzeTypeScope<Decl, analyze_type_scope> 
    }
    Function: crate::util::reuse_node! { 
        prev_pass::Function => AnalyzeTypeScope<Function, analyze_type_scope> 
    }
    TypeDef: crate::util::reuse_node! { 
        prev_pass::TypeDef => Pass<analyze_type_scope> 
    }
}

crate::util::impl_default_passes! {
    impl AnalyzeTypeScope + analyze_type_scope for {
        Block<Expr> => prev_pass::Expr;
        Expr => prev_pass::Expr;
        Field<Ident> => crate::Ident;
        Ident => crate::Ident;
        Value => crate::Value;
    }
}

impl AnalyzeTypeScope<TypeDef> for prev_pass::TypeDef {
    fn analyze_type_scope(self, env: &mut Env) -> PassResult<TypeDef> {
        let pass = self.pass(env)?;
        Ok(pass)
    }
}