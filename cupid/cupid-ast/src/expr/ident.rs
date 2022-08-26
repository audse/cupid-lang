use crate::{
    attr::{Attr, GetAttr},
    Address,
};
use std::borrow::Cow;

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    pub IdentBuilder => pub Ident {
        pub name: Cow<'static, str>,
        // pub namespace: Option<Box<Ident>>,
        pub generics: Vec<Ident>,
        pub address: Option<Address>,
        pub attr: Attr,
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.address.is_some() && self.address == other.address
            || (self.name == other.name && self.generics.len() == other.generics.len())
    }
}

impl Eq for Ident {}

impl PartialOrd for Ident {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for Ident {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl From<&'static str> for Ident {
    fn from(s: &'static str) -> Self {
        Self {
            name: s.into(),
            ..Self::default()
        }
    }
}

impl GetAttr for Ident {
    fn attr(&self) -> Attr {
        self.attr
    }
    fn attr_mut(&mut self) -> &mut Attr {
        &mut self.attr
    }
}
