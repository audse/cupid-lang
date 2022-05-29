use crate::*;

#[derive(Debug, Clone, Tabled)]
pub struct Env {
	pub global: Scope,

	#[tabled(display_with="fmt_closures")]
	pub closures: Vec<(Option<Ident>, Closure)>,

	#[tabled(skip)]
	pub current_closure: usize,

	#[tabled(skip)]
	pub prev_closures: Vec<usize>,

	#[tabled(skip)]
	pub source_data: Vec<ParseNode>,

	#[tabled(display_with="fmt_vec")]
	pub traceback: Vec<String>,
}

fn fmt_closures(closures: &[(Option<Ident>, Closure)]) -> String {
	fmt_list!(closures, |(i, c)| format!("{} :\n{}", fmt_option!(i), c), "\n")
}

impl Default for Env {
	fn default() -> Self {
    	Self {
			global: Scope::new(Context::Global),
			closures: vec![(None, Closure { 
				parent: None, 
				scopes: vec![Scope::new(Context::Block)] 
			})],
			current_closure: 0,
			prev_closures: vec![0],
			source_data: vec![],
			traceback: vec![],
		}
	}
}

impl Env {
	fn fmt_current(&self) -> String {
		fmt_option!(&self.closures[self.current_closure].0, |x| format!("({x})"))
	}
	pub fn add_source(&mut self, source: &mut ParseNode) -> usize {
		self.source_data.push(source.to_owned());
		self.source_data.len() - 1
	}
	pub fn use_closure(&mut self, closure: usize) {
		self.prev_closures.push(self.current_closure);
		self.current_closure = closure;
	}
	pub fn reset_closure(&mut self) {
		if let Some(prev_closure) = self.prev_closures.pop() {
			self.current_closure = prev_closure;
		}
	}
	pub fn add_closure(&mut self, ident: Option<Ident>, context: Context) -> usize {
		if let Some(closure_index) = self.prev_closures.last() {
			self.closures.push((ident, Closure::new_child(*closure_index, context)));
		} else {
			self.closures.push((ident, Closure::new(context)));
		}
		self.closures.len() - 1
	}
	pub fn add_isolated_closure(&mut self, ident: Option<Ident>, context: Context) -> usize {
		self.closures.push((ident, Closure::new(context)));
		self.closures.len() - 1
	}
	pub fn has_symbol(&mut self, symbol: &Ident) -> Result<(), (Source, ErrCode)> {
		if self.get_symbol(symbol).is_ok() {
			Ok(())
		} else {
			self.traceback.push(format!("Could not find {symbol} in the current scope: {} {}", self.current_closure, self.fmt_current()));
			Err((symbol.src(), ERR_NOT_FOUND))
		}
	}
	pub fn no_symbol(&mut self, symbol: &Ident) -> Result<(), (Source, ErrCode)> {
		if self.get_symbol(symbol).is_ok() {
			Err((symbol.src(), ERR_ALREADY_DEFINED))
		} else {
			Ok(())
		}
	}
	pub fn get_symbol_from(&mut self, symbol: &Ident, closure_index: usize) -> Result<SymbolValue, (Source, ErrCode)> {
		let closure = &mut self.closures[closure_index];
		let parent = closure.1.parent();
		if let Ok(value) = closure.1.get_symbol(symbol) {
			return Ok(value);
		}
		if let Some(parent_index) = parent {
			self.get_symbol_from(symbol, parent_index)
		} else {
			self.traceback.push(format!("Could not find {symbol} in the current scope: {} {}", self.current_closure, self.fmt_current()));
			Err((symbol.src(), ERR_NOT_FOUND))
		}
	}
	pub fn add_global<T: ToOwned<Owned = T> + UseAttributes + ToIdent + Into<Val> + Into<Value> + std::fmt::Display>(&mut self, global: &T) {
		let ident = global.to_ident();
		let value = SymbolValue::build()
			.from_type(global.to_owned())
			.build();
		self.global.set_symbol(&ident, value);
	}
	pub fn update_closure(&mut self, symbol: &Ident, closure: usize) -> Result<(), (Source, ErrCode)> {
		let mut value = self.get_symbol(symbol)?;
		value.value.map_mut(|a| a.attributes.closure = closure);
		self.set_symbol(symbol, value);
		Ok(())
	}
	// pub fn debug_find_by_source(&mut self, source: usize) -> Option<(&Ident, &SymbolValue)> {
	// 	for closure in self.closures.iter_mut() {
	// 		for scope in closure.1.scopes.iter_mut() {
	// 			for (symbol, val) in scope.symbols.iter() {
	// 				if symbol.attributes.source == Some(source) {
	// 					return Some((symbol, val));
	// 				}
	// 				if let Some(value) = &val.value {
	// 					if value.attributes.source == Some(source) {
	// 						return Some((symbol, val))
	// 					}
	// 				}
	// 			}
	// 		}
	// 	}
	// 	for (symbol, val) in self.global.symbols.iter() {
	// 		if symbol.attributes.source == Some(source) {
	// 			return Some((symbol, val));
	// 		}
	// 		if let Some(value) = &val.value {
	// 			if value.attributes.source == Some(source) {
	// 				return Some((symbol, val))
	// 			}
	// 		}
	// 	}
	// 	None
	// }
}

impl ScopeSearch for Env {
	fn get_symbol(&mut self, symbol: &Ident) -> Result<SymbolValue, (Source, ErrCode)> {
		if let Ok(value) = self.get_symbol_from(symbol, self.current_closure) {
			return Ok(value);
		}
		self.global.get_symbol(symbol)
	}
	fn get_type(&mut self, symbol: &Ident) -> Result<Type, (Source, ErrCode)> {
		if let Some(closure) = self.closures.get_mut(self.current_closure) {
			if let Ok(value) = closure.1.get_type(symbol) {
				return Ok(value)
			}
		}
		self.global.get_type(symbol)
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		if let Some(closure) = self.closures.get_mut(self.current_closure) {
			closure.1.set_symbol(symbol, value);
		}
	}
	fn modify_symbol(&mut self, symbol: &Ident, function: impl FnMut(&mut SymbolValue) -> Result<(), (Source, ErrCode)>) -> Result<(), (Source, ErrCode)> {
		if let Some(closure) = self.closures.get_mut(self.current_closure) {
			closure.1.modify_symbol(symbol, function)
		} else {
			Ok(())
		}
	}
}

type AnalyzeResult = Result<Vec<()>, (Source, ErrCode)>;

pub fn add_globals(scope: &mut Env, mut types: Vec<Type>, mut traits: Vec<Trait>) -> Result<(), (Source, ErrCode)> {
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