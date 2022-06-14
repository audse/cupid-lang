use cupid_util::{InvertOption, Bx, Str, node_builder};
use cupid_scope::Env;
use crate::{pre_analysis, PassResult, ast_pass_nodes};

#[cupid_semantics::auto_implement(Vec, Option)]
pub trait AnalyzeScope<T> where Self: Sized {
    fn analyze_scope(self, env: &mut Env) -> PassResult<T>;
}

ast_pass_nodes! {
    Decl: node_builder! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl {
            pub ident: Ident,
            pub type_annotation: Option<Ident>,
            pub value: Box<Expr>,
        }
    }
    Function: node_builder! {
        #[derive(Debug, Default, Clone)]
        pub FunctionBuilder => pub Function {
            pub params: Vec<Decl>,
            pub return_type_annotation: Option<Ident>,
            pub body: Vec<Expr>,
        }
    }
    Ident: node_builder! {
        #[derive(Debug, Default, Clone)]
        pub IdentBuilder => pub Ident {
            pub name: Str,
            pub generics: Vec<Ident>
        }
    }
}

crate::impl_expr_ast_pass! {
    impl AnalyzeScope<Expr> for pre_analysis::Expr { analyze_scope }
}

crate::impl_block_ast_pass! {
    impl AnalyzeScope<crate::Block<Expr>> for crate::Block<pre_analysis::Expr> {
        fn analyze_scope(self, env: &mut Env) -> PassResult<crate::Block<Expr>> {
            let Self { expressions, attr, ..} = self;
            Ok(crate::Block::build()
                .expressions(expressions.analyze_scope(env)?)
                .attr(attr)
                .scope(env.current_closure)
                .build())
        }
    }
}

impl AnalyzeScope<Decl> for pre_analysis::Decl {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Decl> {
        let Self { ident, value, type_annotation, attr, ..} = self;
        Ok(Decl::build()
            .ident(ident.analyze_scope(env)?)
            .value(value.analyze_scope(env)?.bx())
            .type_annotation(type_annotation.analyze_scope(env)?)
            .attr(attr)
            .scope(env.current_closure)
            .build())
    }
}

impl AnalyzeScope<Function> for pre_analysis::Function {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Function> {
        let Self { params, return_type_annotation, body, attr, ..} = self;
        Ok(Function::build()
            .params(params.analyze_scope(env)?)
            .return_type_annotation(return_type_annotation.analyze_scope(env)?)
            .body(body.analyze_scope(env)?)
            .attr(attr)
            .scope(env.add_scope(cupid_scope::Context::Function))
            .build())
    }
}

impl AnalyzeScope<Ident> for pre_analysis::Ident {
    fn analyze_scope(self, env: &mut Env) -> PassResult<Ident> {
        let Self { name, generics, attr, ..} = self;
        Ok(Ident::build()
            .name(name)
            .generics(generics.analyze_scope(env)?)
            .attr(attr)
            .scope(env.current_closure)
            .build())
    }
}