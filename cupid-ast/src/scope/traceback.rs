use crate::*;

pub trait TraceScope {
	fn t<S: ToString>(&mut self, message: S) {
		self.scope().trace(message)
	}
	fn scope_ref(&self) -> &Env;
	fn scope(&mut self) -> &mut Env;
	fn to_name(&self, closure: usize, ident: &Option<Ident>) -> String {
		let ident = fmt_option!(ident, |x| format!(" ({x})"));
		format!("{closure}{ident}")
	}
	fn name(&self, closure: usize) -> String {
		self.to_name(closure, &self.scope_ref().closures[closure].name)
	}
	fn current_name(&mut self) -> String {
		self.name(self.scope_ref().current_closure)
	}
	fn trace_closure_reset(&mut self, closure: usize) {
		let (prev, current) = (
			self.name(self.scope_ref().current_closure),
			self.name(closure)
		);
		self.t(format!("Resetting closure: {prev} => {current}"));
	}
	fn trace_closure<S: Into<String>>(&mut self, closure: usize, name: S) {
		let (prev, current) = (
			self.name(self.scope_ref().current_closure),
			self.name(closure)
		);
		self.t(format!("Accessing closure: {prev} => {current} <{}>", name.into()));
	}
	fn trace_get_symbol(&mut self, symbol: &Ident) {
		let current = self.current_name();
		self.t(format!("Getting symbol `{symbol}` from scope {current}"));
	}
	fn trace_get_type(&mut self, symbol: &Ident) {
		let current = self.current_name();
		self.t(format!("Getting type `{symbol}` from scope {current}"));
	}
	fn trace_set_symbol(&mut self, symbol: &Ident, value: &SymbolValue) {
		let current = self.current_name();
		self.t(format!("Setting symbol `{symbol}` in scope {current} to value: \n{value}"));
	}
	fn trace_modify_symbol(&mut self, symbol: &Ident) {
		let current = self.current_name();
		self.t(format!("Modifying symbol `{symbol}` in scope {current}"));
	}
	fn trace_check_has_symbol(&mut self, symbol: &Ident) {
		let current = self.current_name();
		self.t(format!("Making sure `{symbol}` exists in scope {current}"));
	}
	fn trace_check_no_symbol(&mut self, symbol: &Ident) {
		let current = self.current_name();
		self.t(format!("Making sure `{symbol}` does not exist in scope {current}"));
	}
	fn trace_no_symbol(&mut self, symbol: &Ident) {
		let current = self.current_name();
		self.t(format!("Could not find `{symbol}` in the current scope: {current}"));
	}
	fn trace_update_closure(&mut self, symbol: &Ident, closure: usize) {
		let closure = self.name(closure);
		self.t(format!("Updating `{symbol}` to use closure {closure}"))
	}
	fn trace_add(&mut self, ident: &Option<Ident>, context: Context) {
		let parent = self.current_name();
		let name = self.to_name(self.scope_ref().closures.len(), ident);
		self.t(format!("Adding closure {name} <{context}> as child of {parent}"))
	}
	fn trace_add_isolated(&mut self, ident: &Option<Ident>, context: Context) {
		let name = self.to_name(self.scope_ref().closures.len(), ident);
		self.t(format!("Adding closure {name} <{context}> as decoupled"))
	}
}

impl TraceScope for Env {
	fn scope_ref(&self) -> &Env { self }
	fn scope(&mut self) -> &mut Env { self }
}