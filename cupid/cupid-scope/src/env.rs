use crate::symbol_table::*;
use crate::*;

#[derive(Debug, Clone)]
pub struct Env {
	pub global: Scope,
	pub closures: Vec<Closure>,
	pub symbols: SymbolTable,
	pub current_closure: usize,
	pub prev_closures: Vec<usize>,
	pub source_data: Vec<ParseNode>,
	pub token_data: Vec<Vec<Token>>,
	pub traceback: Vec<String>,
}

impl Default for Env {
	fn default() -> Self {
    	Self {
			global: Scope::new(0, Context::Global),
			closures: vec![Closure {
				id: 1,
				name: None,
				parent: None, 
				scopes: vec![Scope::new(2, Context::Block)] 
			}],
			symbols: SymbolTable::default(),
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
			self.closures.push(Closure::new_child(
				self.closures.len(),
				ident,
				*closure_index,
				context
			));
		} else {
			self.closures.push(Closure::new(self.closures.len(), ident, context));
		}
		self.closures.len() - 1
	}
	pub fn add_scope(&mut self, context: Context) -> usize {
		self.closures.last_mut().map_mut(|closure| closure.add(context)).unwrap_or_default()
	}
	pub fn add_isolated_closure(&mut self, ident: Option<Ident>, context: Context) -> usize {
		self.trace_add_isolated(&ident, context);
		// always has access to top-level scope
		self.closures.push(Closure::new_child(self.closures.len(), ident, 0, context));
		self.closures.len() - 1
	}
	pub fn current_context(&self) -> Context {
		self.closures[self.current_closure].scopes.last().unwrap().context
	}
	pub fn has_address(&mut self, symbol: &Ident) -> ASTResult<()> {
		self.trace("Has address for {symbol:?}?");
		self.get_address(symbol)?;
		Ok(())
	}
	pub fn no_address(&mut self, symbol: &Ident) -> ASTResult<()> {
		if self.get_address(symbol).is_err() {
			Ok(())
		} else {
			symbol.to_err(ERR_ALREADY_DEFINED)
		}
	}
	pub fn get_address_from(&mut self, symbol: &Ident, closure_index: usize) -> ASTResult<Address> {
		let closure = &mut self.closures[closure_index];
		let parent = closure.parent;
		if let Ok(value) = closure.get_address(symbol) {
			Ok(value)
		} else if let Some(parent_index) = parent {
			self.get_address_from(symbol, parent_index)
		} else {
			symbol.to_err(ERR_NOT_FOUND)
		}
	}
	pub fn get_source_node(&mut self, source: usize) -> &ParseNode {
		&mut self.source_data[source]
	}
}
