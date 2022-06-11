use crate::*;

#[derive(Debug, Clone)]
pub struct Env {
	pub global: Scope,
	pub closures: Vec<Closure>,
	pub current_closure: usize,
	pub prev_closures: Vec<usize>,
	pub source_data: Vec<ParseNode>,
	pub token_data: Vec<Vec<Token>>,
	pub traceback: Vec<String>,
}

impl Default for Env {
	fn default() -> Self {
    	Self {
			global: Scope::new(Context::Global),
			closures: vec![Closure {
				name: None,
				parent: None, 
				scopes: vec![Scope::new(Context::Block)] 
			}],
			current_closure: 0,
			prev_closures: vec![0],
			source_data: vec![],
			token_data: vec![],
			traceback: vec![],
		}
	}
}

impl Env {
	pub fn trace<S: ToString>(&mut self, message: S) {
		self.traceback.push(message.to_string());
	}
	pub fn add_source(&mut self, source: ParseNode) -> usize {
		self.source_data.push(source);
		self.source_data.len() - 1
	}
	pub fn use_closure<S: Into<String>>(&mut self, closure: usize, type_name: S) {
		self.trace_closure(closure, type_name);
		self.prev_closures.push(self.current_closure);
		self.current_closure = closure;
	}
	pub fn reset_closure(&mut self) {
		if let Some(prev_closure) = self.prev_closures.pop() {
			self.trace_closure_reset(prev_closure);
			self.current_closure = prev_closure;
		}
	}
	pub fn add_closure(&mut self, ident: Option<Ident>, context: Context) -> usize {
		self.trace_add(&ident, context);
		if let Some(closure_index) = self.prev_closures.last() {
			self.closures.push(Closure::new_child(ident, *closure_index, context));
		} else {
			self.closures.push(Closure::new(ident, context));
		}
		self.closures.len() - 1
	}
	pub fn add_isolated_closure(&mut self, ident: Option<Ident>, context: Context) -> usize {
		self.trace_add_isolated(&ident, context);
		// always has access to top-level scope
		self.closures.push(Closure::new_child(ident, 0, context));
		self.closures.len() - 1
	}
	pub fn has_symbol(&mut self, symbol: &Ident) -> ASTResult<()> {
		self.trace_check_has_symbol(symbol);
		if self.get_symbol(symbol).is_ok() {
			Ok(())
		} else {
			self.trace_no_symbol(symbol);
			symbol.to_err(ERR_NOT_FOUND)
		}
	}
	pub fn no_symbol(&mut self, symbol: &Ident) -> ASTResult<()> {
		self.trace_check_no_symbol(symbol);
		if self.get_symbol(symbol).is_ok() {
			symbol.to_err(ERR_ALREADY_DEFINED)
		} else {
			Ok(())
		}
	}
	pub fn get_symbol_from(&mut self, symbol: &Ident, closure_index: usize) -> ASTResult<SymbolValue> {
		let closure = &mut self.closures[closure_index];
		let parent = closure.parent();
		if let Ok(value) = closure.get_symbol(symbol) {
			return Ok(value);
		}
		if let Some(parent_index) = parent {
			self.get_symbol_from(symbol, parent_index)
		} else {
			self.trace_no_symbol(symbol);
			symbol.to_err(ERR_NOT_FOUND)
		}
	}
	pub fn update_closure(&mut self, symbol: &Ident, closure: usize) -> ASTResult<()> {
		// TODO why does this infinite loop?
		// self.modify_symbol(symbol, |val| {
		// 	val.attributes_mut().closure = closure;
		// 	Ok(())
		// })
		self.trace_update_closure(symbol, closure);
		let mut value = self.get_symbol(symbol)?;
		value.value.map_mut(|a| a.attributes_mut().closure = closure);
		self.set_symbol(symbol, value);
		Ok(())
	}
	pub fn get_source_node(&self, source: usize) -> &ParseNode {
		&self.source_data[source]
	}
	pub fn get_type(&mut self, symbol: &Ident) -> ASTResult<Type> {
		let val = self.get_symbol(symbol)?;
		if let Some(VType(val)) = val.value {
			Ok(val)
		} else {
			val.to_err(ERR_EXPECTED_TYPE)
		}
	}
}
