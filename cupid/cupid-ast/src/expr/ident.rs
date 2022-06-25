use std::borrow::Cow;
use crate::{Address, attr::Attr};

cupid_util::build_struct! {
    #[derive(Debug, Default, Clone)]
    pub IdentBuilder => pub Ident {
        pub name: Cow<'static, str>,
        pub namespace: Option<Box<Ident>>,
        pub generics: Vec<Ident>,
        pub address: Option<Address>,
        pub attr: Attr,
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address || (
            self.name == other.name 
            && self.generics.len() == other.generics.len()
            && self.namespace == other.namespace
        )
    }
}

impl Eq for Ident {}

impl From<&'static str> for Ident {
    fn from(s: &'static str) -> Self {
        Self { name: s.into(), ..Self::default() }
    }
}