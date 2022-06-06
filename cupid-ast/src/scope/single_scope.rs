use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Default, Tabled)]
pub struct Scope<'scope> {
	pub context: Context,
	#[tabled(display_with="fmt_map")]
	pub symbols: HashMap<Ident, SymbolValue<'scope>>,
}

impl Scope<'_> {
	pub fn new(context: Context) -> Self {
		Self {
			context,
			symbols: HashMap::default()
		}
	}
}