use crate::{expr::ident::Ident, attr::Attr, stmt::decl::Decl};

#[derive(Debug, Default, Copy, Clone, derive_more::Display)]
pub enum BaseType {
    Struct,
    Sum,
    Array,
    #[default]
    Variable,
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
            base: BaseType::Struct,
            ..Self::default()
        }
    }
    pub fn typ() -> Self {
        Self {
            ident: "type".into(),
            base: BaseType::Struct,
            ..Self::default()
        }
    }
    pub fn traits() -> Self {
        Self {
            ident: "trait".into(),
            base: BaseType::Struct,
            ..Self::default()
        }
    }
}
