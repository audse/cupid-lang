use crate::{
    attr::{Attr, GetAttr},
    expr::ident::Ident,
    stmt::decl::Decl,
};

#[derive(
    Debug, Default, Copy, Clone, derive_more::Display, serde::Serialize, serde::Deserialize,
)]
pub enum BaseType {
    Struct,
    Sum,
    Array,
    #[default]
    Variable,
}

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
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
    pub fn variable() -> Self {
        Self {
            ident: "1".into(),
            ..Default::default()
        }
    }
    pub fn is_function(&self) -> bool {
        &self.ident.name == "fun"
    }
}

impl GetAttr for Type {
    fn attr(&self) -> Attr {
        self.attr
    }
    fn attr_mut(&mut self) -> &mut Attr {
        &mut self.attr
    }
}
