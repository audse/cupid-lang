use std::collections::BTreeMap;
use crate::{expr::{ident::Ident, function::Function}, attr::Attr};

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
        pub name: Ident,
        pub fields: Vec<Field>,
        pub methods: BTreeMap<Ident, Function>,
        pub base: BaseType,
        pub attr: Attr,
    }
}

cupid_util::build_struct! {
	#[derive(Debug, Default, Clone)]
	pub FieldBuilder => pub Field {
		pub name: Ident,
		pub type_annotation: Option<Ident>,
	}
}

impl Type {
    pub fn none() -> Self {
        Self {
            name: "none".into(),
            ..Self::default()
        }
    }
    pub fn typ() -> Self {
        Self {
            name: "type".into(),
            ..Self::default()
        }
    }
}
