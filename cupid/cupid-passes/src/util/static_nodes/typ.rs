use crate::{Ident, Field, tests::Attributes};

#[derive(Debug, Default, Copy, Clone)]
pub enum BaseTyp {
    Primitive,
    #[default]
    Struct,
    Sum,
    Array,
}

crate::util::node_builder! {
    #[derive(Debug, Clone)]
    pub TypBuilder => pub Typ {
        pub name: Ident,
        pub fields: Vec<Field<Ident>>,
        pub base: BaseTyp,
    }
}

impl Typ {
    pub fn none() -> Self {
        Self {
            name: "none".into(),
            ..Default::default()
        }
    }
    pub fn generic<S: Into<Ident>>(name: S) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

impl Default for Typ {
    fn default() -> Self {
        Self {
            name: "none".into(),
            fields: vec![],
            base: BaseTyp::Primitive,
            attr: Attributes::default()
        }
    }
}

impl Default for TypBuilder {
    fn default() -> Self {
        Typ::default().builder()
    }
}

impl From<Ident> for Typ {
    fn from(name: Ident) -> Self {
        Self {
            attr: name.attr,
            name,
            base: BaseTyp::Primitive,
            fields: vec![],
        }
    }
}