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
	pub fn trace<S: Into<String>>(&mut self, message: S) {
		self.traceback.push(message.into());
	}
	pub fn fmt_current(&self) -> String {
		fmt_option!(&self.closures[self.current_closure].0, |x| format!("({x})"))
	}
	pub fn add_source(&mut self, source: ParseNode) -> usize {
		self.source_data.push(source);
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
		// always has access to top-level scope
		self.closures.push((ident, Closure::new_child(0, context)));
		self.closures.len() - 1
	}
	pub fn has_symbol(&mut self, symbol: &Ident) -> ASTResult<()> {
		if self.get_symbol(symbol).is_ok() {
			Ok(())
		} else {
			self.trace(format!("Could not find {symbol} in the current scope: {} {}", self.current_closure, self.fmt_current()));
			symbol.to_err(ERR_NOT_FOUND)
		}
	}
	pub fn no_symbol(&mut self, symbol: &Ident) -> ASTResult<()> {
		if self.get_symbol(symbol).is_ok() {
			symbol.to_err(ERR_ALREADY_DEFINED)
		} else {
			Ok(())
		}
	}
	pub fn get_symbol_from(&mut self, symbol: &Ident, closure_index: usize) -> ASTResult<SymbolValue> {
		let closure = &mut self.closures[closure_index];
		let parent = closure.1.parent();
		if let Ok(value) = closure.1.get_symbol(symbol) {
			return Ok(value);
		}
		if let Some(parent_index) = parent {
			self.get_symbol_from(symbol, parent_index)
		} else {
			self.trace(format!("Could not find {symbol} in the current scope: {} {}", self.current_closure, self.fmt_current()));
			symbol.to_err(ERR_NOT_FOUND)
		}
	}
	pub fn update_closure(&mut self, symbol: &Ident, closure: usize) -> ASTResult<()> {
		// TODO why does this infinite loop?
		// self.modify_symbol(symbol, |val| {
		// 	val.attributes_mut().closure = closure;
		// 	Ok(())
		// })
		let mut value = self.get_symbol(symbol)?;
		value.value.map_mut(|a| a.attributes.closure = closure);
		self.set_symbol(symbol, value);
		Ok(())
	}
	pub fn get_source_node(&self, source: usize) -> &ParseNode {
		&self.source_data[source]
	}
}
