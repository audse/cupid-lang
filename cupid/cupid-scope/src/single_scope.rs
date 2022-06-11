use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Scope {
	pub context: Context,
	pub symbols: HashMap<Ident, SymbolValue>,
}

impl Scope {
	pub fn new(context: Context) -> Self {
		Self {
			context,
			symbols: HashMap::default()
		}
	}
}