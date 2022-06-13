crate::ast_pass_nodes!{
    Decl: cupid_util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub DeclBuilder => pub Decl {
            pub ident_address: crate::Address,
            pub type_annotation_address: Option<crate::Address>,
            pub value: Box<crate::SemanticNode<Expr>>,
        }
    }
    Ident: cupid_util::node_builder! {
        #[derive(Debug, Default, Clone)]
        pub IdentBuilder => pub Ident {
            pub name: crate::Address,
            pub generics: Vec<crate::Address>
        }
    }
}
