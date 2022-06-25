use crate::{expr::ident::Ident, attr::Attr, stmt::decl::Decl};

#[derive(Debug, Default, Copy, Clone, derive_more::Display)]
pub enum BaseType {
    #[default]
    Primitive,
    Struct,
    Sum,
    Array,
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone)]
    pub TypeBuilder => pub Type {
        pub ident: Ident,
        pub fields: Vec<Decl>,
        pub methods: Vec<Decl>,
        pub base: BaseType,
        pub attr: Attr,
    }
}

impl Type {
    pub fn none() -> Self {
        Self {
            ident: "none".into(),
            ..Self::default()
        }
    }
    pub fn typ() -> Self {
        Self {
            ident: "type".into(),
            ..Self::default()
        }
    }
}
