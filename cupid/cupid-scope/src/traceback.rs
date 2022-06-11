use crate::*;

#[trace_this]
impl Trace for Env {
	fn t<S: ToString>(&mut self, message: S) {
		self.trace(message)
	}
	fn to_name(&self, closure: usize, ident: &Option<Ident>) -> String {
		let ident = fmt_option!(ident => |x| format!(" ({})", x as &dyn Fmt));
		format!("{closure}{ident}")
	}
	fn name(&self, closure: usize) -> String {
		self.to_name(closure, &self.closures[closure].name)
	}
	fn current_name(&mut self) -> String {
		self.name(self.current_closure)
	}
	fn trace_closure_reset(&mut self, closure: usize) {
		let (prev, current) = (
			self.name(self.current_closure),
			self.name(closure)
		);
		self.t(format!("Resetting closure: {prev} => {current}"));
	}
	fn trace_closure<S: Into<String>>(&mut self, closure: usize, name: S) {
		let (prev, current) = (
			self.name(self.current_closure),
			self.name(closure)
		);
		self.t(format!("Accessing closure: {prev} => {current} <{}>", name.into()));
	}
	fn trace_get_symbol(&mut self, symbol: &Ident) {
		let current = self.current_name();
		self.t(format!("Getting symbol `{}` from scope {current}", symbol as &dyn Fmt));
	}
	fn trace_get_type(&mut self, symbol: &Ident) {
		let current = self.current_name();
		self.t(format!("Getting type `{}` from scope {current}", symbol as &dyn Fmt));
	}
	fn trace_set_symbol(&mut self, symbol: &Ident, value: &SymbolValue) {
		let current = self.current_name();
		self.t(format!("Setting symbol `{}` in scope {current} to value: \n{}", symbol as &dyn Fmt, value as &dyn Fmt));
	}
	fn trace_modify_symbol(&mut self, symbol: &Ident) {
		let current = self.current_name();
		self.t(format!("Modifying symbol `{}` in scope {current}", symbol as &dyn Fmt));
	}
	fn trace_check_has_symbol(&mut self, symbol: &Ident) {
		let current = self.current_name();
		self.t(format!("Making sure `{}` exists in scope {current}", symbol as &dyn Fmt));
	}
	fn trace_check_no_symbol(&mut self, symbol: &Ident) {
		let current = self.current_name();
		self.t(format!("Making sure `{}` does not exist in scope {current}", symbol as &dyn Fmt));
	}
	fn trace_no_symbol(&mut self, symbol: &Ident) {
		let current = self.current_name();
		self.t(format!("Could not find `{}` in the current scope: {current}", symbol as &dyn Fmt));
	}
	fn trace_update_closure(&mut self, symbol: &Ident, closure: usize) {
		let closure = self.name(closure);
		self.t(format!("Updating `{}` to use closure {closure}", symbol as &dyn Fmt))
	}
	fn trace_add(&mut self, ident: &Option<Ident>, context: Context) {
		let parent = self.current_name();
		let name = self.to_name(self.closures.len(), ident);
		self.t(format!("Adding closure {name} <{context}> as child of {parent}"))
	}
	fn trace_add_isolated(&mut self, ident: &Option<Ident>, context: Context) {
		let name = self.to_name(self.closures.len(), ident);
		self.t(format!("Adding closure {name} <{context}> as decoupled"))
	}
}