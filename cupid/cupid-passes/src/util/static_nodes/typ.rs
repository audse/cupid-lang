use crate::{Ident, Field};

#[derive(Debug, Default, Copy, Clone)]
pub enum BaseTyp {
    Primitive(&'static str),
    #[default]
    Struct,
    Sum,
    Array,
}

cupid_util::node_builder! {
    #[derive(Debug, Default, Clone)]
    pub TypBuilder => pub Typ {
        pub name: Ident,
        pub fields: Vec<Field<Ident>>,
        pub base: BaseTyp,
    }
}