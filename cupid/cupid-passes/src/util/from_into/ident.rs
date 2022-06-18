use cupid_util::Str;
use crate::Ident;

impl From<&'static str> for Ident {
	fn from(i: &'static str) -> Self {
		Self { name: i.into(), ..Default::default() }
	}
}

impl From<Str> for Ident {
	fn from(i: Str) -> Self {
		Self { name: i, ..Default::default() }
	}
}
