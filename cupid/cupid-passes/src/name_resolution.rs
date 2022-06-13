
#[derive(Debug, Default, Clone)]
pub enum Expr {
    Decl(Decl),
    Ident(Ident),
    Value(crate::Value),

    #[default]
    Empty
}

cupid_util::node_builder! {
    #[derive(Debug, Default, Clone)]
    pub IdentBuilder => pub Ident {
        pub name: crate::Address,
        pub generics: Vec<crate::Address>
    }
}

cupid_util::node_builder! {
    #[derive(Debug, Default, Clone)]
    pub DeclBuilder => pub Decl {
        pub ident_address: crate::Address,
        pub type_annotation_address: Option<crate::Address>,
        pub value: Box<crate::SemanticNode<Expr>>,
    }
}
