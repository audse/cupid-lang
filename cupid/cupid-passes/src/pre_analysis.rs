use cupid_util::node_builder;

crate::ast_pass_nodes! {
    Decl: node_builder! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl {
            pub ident: Ident,
            pub type_annotation: Option<Ident>,
            pub value: Box<Expr>,
            pub mutable: bool,
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
            pub namespace: Option<Box<Ident>>,
            pub name: std::borrow::Cow<'static, str>,
            pub generics: Vec<Ident>
        }
    }
}
