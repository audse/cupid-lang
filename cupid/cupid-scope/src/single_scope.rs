use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Scope {
	pub id: usize,
	pub context: Context,
	pub symbols: HashMap<Ident, symbol_table::Address>,
}

impl Scope {
	pub fn new(id: usize, context: Context) -> Self {
		Self {
			id,
			context,
			symbols: HashMap::default()
		}
	}
}