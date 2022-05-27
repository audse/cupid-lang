use crate::*;

#[derive(Debug, Clone, Tabled)]
pub struct Env {
	pub global: Scope,

	#[tabled(display_with="fmt_vec")]
	pub closures: Vec<Closure>,

	#[tabled(skip)]
	pub current_closure: usize,

	#[tabled(skip)]
	pub prev_closure: Option<usize>,

	#[tabled(skip)]
	pub source_data: Vec<ParseNode>,

	#[tabled(display_with="fmt_vec")]
	pub traceback: Vec<String>,
}

impl Default for Env {
	fn default() -> Self {
    	Self {
			global: Scope::new(Context::Global),
			closures: vec![Closure { 
				parent: None, 
				scopes: vec![Scope::new(Context::Block)] 
			}],
			current_closure: 0,
			prev_closure: None,
			source_data: vec![],
			traceback: vec![],
		}
	}
}

impl Env {
	pub fn add_source(&mut self, source: &mut ParseNode) -> usize {
		self.source_data.push(source.to_owned());
		self.source_data.len() - 1
	}
	pub fn use_closure(&mut self, closure: usize) {
		self.prev_closure = Some(self.current_closure);
		self.current_closure = closure;
	}
	pub fn reset_closure(&mut self) {
		if let Some(prev_closure) = self.prev_closure {
			self.current_closure = prev_closure;
			self.prev_closure = None;
		}
	}
	pub fn add_closure(&mut self) -> usize {
		if let Some(closure_index) = self.prev_closure {
			self.closures.push(Closure::new_child(closure_index));
		} else {
			self.closures.push(Closure::new());
		}
		self.closures.len() - 1
	}
	pub fn add_isolated_closure(&mut self) -> usize {
		self.traceback.push(format!("Add isolated closure {}", self.closures.len()));

		self.closures.push(Closure::new());
		self.closures.len() - 1
	}
	pub fn pop_closure(&mut self) -> Option<Closure> {
		self.closures.pop()
	}
	pub fn add(&mut self, context: Context) {
		self.traceback.push(format!("Add closure {}", self.closures.len()));
		self.closures.last_mut().unwrap().add(context);
	}
	pub fn pop(&mut self) -> Option<Scope> {
		self.closures.last_mut().unwrap().pop()
	}
	pub fn has_symbol(&mut self, symbol: &Ident) -> Result<(), (Source, ErrCode)> {
		if self.get_symbol(symbol).is_ok() {
			Ok(())
		} else {
			Err((symbol.src(), 404))
		}
	}
	pub fn no_symbol(&mut self, symbol: &Ident) -> Result<(), (Source, ErrCode)> {
		self.traceback.push(format!("Making sure {} doesn't exist", symbol));
		if self.get_symbol(symbol).is_ok() {
			Err((symbol.src(), ERR_ALREADY_DEFINED))
		} else {
			Ok(())
		}
	}
	pub fn get_symbol_from(&mut self, symbol: &Ident, closure_index: usize) -> Result<SymbolValue, (Source, ErrCode)> {
		self.traceback.push(format!("Getting symbol {} from closure {}", symbol, closure_index));
		let closure = &mut self.closures[closure_index];
		let parent = closure.parent();
		if let Ok(value) = closure.get_symbol(symbol) {
			return Ok(value);
		}
		if let Some(parent_index) = parent {
			self.get_symbol_from(symbol, parent_index)
		} else {
			Err((symbol.src(), 404))
		}
	}
	pub fn add_global<T: ToOwned<Owned = T> + UseAttributes + ToIdent + Into<Val> + std::fmt::Display>(&mut self, global: &T) {
		self.traceback.push(format!("Adding global\n{global}"));
		let mut global = global.to_owned();
		let ident = global.to_ident();
		let attributes = global.attributes().to_owned();
		let value = SymbolValue {
			value: Some(Value { val: Typed::Untyped(global.into()), attributes }),
			type_hint: ident.to_owned(),
			mutable: false,
		};
		self.global.set_symbol(&ident, value);
	}
	pub fn debug_find_by_source(&mut self, source: usize) -> Option<(&Ident, &SymbolValue)> {
		for closure in self.closures.iter_mut() {
			for scope in closure.scopes.iter_mut() {
				for (symbol, val) in scope.symbols.iter() {
					if symbol.attributes.source == Some(source) {
						return Some((symbol, val));
					}
					if let Some(value) = &val.value {
						if value.attributes.source == Some(source) {
							return Some((symbol, val))
						}
					}
				}
			}
		}
		for (symbol, val) in self.global.symbols.iter() {
			if symbol.attributes.source == Some(source) {
				return Some((symbol, val));
			}
			if let Some(value) = &val.value {
				if value.attributes.source == Some(source) {
					return Some((symbol, val))
				}
			}
		}
		None
	}
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
			if let Ok(value) = closure.get_type(symbol) {
				return Ok(value)
			}
		}
		self.global.get_type(symbol)
	}
	fn set_symbol(&mut self, symbol: &Ident, value: SymbolValue) {
		if let Some(closure) = self.closures.get_mut(self.current_closure) {
			closure.set_symbol(symbol, value);
		}
	}
	fn modify_symbol(&mut self, symbol: &Ident, function: &dyn Fn(&mut SymbolValue)) {
		if let Some(closure) = self.closures.get_mut(self.current_closure) {
			closure.modify_symbol(symbol, function);
		}
	}
}

pub fn add_globals(scope: &mut Env, mut types: Vec<Type>, mut traits: Vec<Trait>) -> Result<(), (Source, ErrCode)> {
	for type_global in &types {
		scope.add_global(type_global);
	}
	for trait_global in &traits {
		scope.add_global(trait_global);
	}
	for type_global in types.iter_mut() {
		type_global.analyze_names(scope)?;
	}
	for trait_global in traits.iter_mut() {
		trait_global.analyze_names(scope)?;
	}
	for type_global in types.iter_mut() {
		type_global.analyze_types(scope)?;
	}
	for trait_global in traits.iter_mut() {
		trait_global.analyze_types(scope)?;
	}
	for type_global in types.iter_mut() {
		type_global.check_types(scope)?;
	}
	for trait_global in traits.iter_mut() {
		trait_global.check_types(scope)?;
	}
	Ok(())
}

#[macro_export]
macro_rules! global_vec {
	($($global:ident),*) => {
		vec![$($global.to_owned()),*]
	};
}