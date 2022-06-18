use crate::{Ident, Field, tests::Attributes};

#[derive(Debug, Default, Copy, Clone)]
pub enum BaseType {
    Primitive,
    #[default]
    Struct,
    Sum,
    Array,
}

crate::util::node_builder! {
    #[derive(Debug, Clone)]
    pub TypBuilder => pub Type {
        pub name: Ident,
        pub fields: Vec<Field<Ident>>,
        pub base: BaseType,
    }
}

impl Type {
    pub fn none() -> Self {
        Self {
            name: "none".into(),
            ..Default::default()
        }
    }
    pub fn generic<S: Into<Ident>>(name: S) -> Self {
        let name = name.into();
        Self {
            attr: name.attr,
            name,
            ..Default::default()
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Self {
            name: "none".into(),
            fields: vec![],
            base: BaseType::Primitive,
            attr: Attributes::default()
        }
    }
}

impl Default for TypBuilder {
    fn default() -> Self {
        Type::default().builder()
    }
}