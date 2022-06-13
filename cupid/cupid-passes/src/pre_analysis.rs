
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
        pub name: std::borrow::Cow<'static, str>,
        pub generics: Vec<Ident>
    }
}

cupid_util::node_builder! {
    #[derive(Debug, Default, Clone)]
    pub DeclBuilder => pub Decl {
        pub ident: Ident,
        pub type_annotation: Option<Ident>,
        pub value: Box<Expr>,
        pub mutable: bool,
    }
}