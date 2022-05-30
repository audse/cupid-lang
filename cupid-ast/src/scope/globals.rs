use crate::*;

impl Env {
	pub fn add_global<T: ToOwned<Owned = T> + UseAttributes + ToIdent + Into<Val> + Into<Value> + std::fmt::Display>(&mut self, global: &T) {
		let ident = global.to_ident();
		let value = SymbolValue::build()
			.from_type(global.to_owned())
			.build();
		self.global.set_symbol(&ident, value);
	}
}

type AnalyzeResult = Result<Vec<()>, ASTErr>;

pub fn add_globals(scope: &mut Env, mut types: Vec<Type>, mut traits: Vec<Trait>) -> Result<(), ASTErr> {
	types.iter().for_each(|t| scope.add_global(t));
	traits.iter().for_each(|t| scope.add_global(t));

	types.iter_mut().map(|t| t.analyze_scope(scope)).collect::<AnalyzeResult>()?;
	traits.iter_mut().map(|t| t.analyze_scope(scope)).collect::<AnalyzeResult>()?;

	types.iter_mut().map(|t| t.analyze_names(scope)).collect::<AnalyzeResult>()?;
	traits.iter_mut().map(|t| t.analyze_names(scope)).collect::<AnalyzeResult>()?;

	types.iter_mut().map(|t| t.analyze_scope(scope)).collect::<AnalyzeResult>()?;
	traits.iter_mut().map(|t| t.analyze_scope(scope)).collect::<AnalyzeResult>()?;
	Ok(())
}

#[macro_export]
macro_rules! global_vec {
	($($global:ident),*) => {
		vec![$($global.to_owned()),*]
	};
}