pub type Scope = (usize, usize);
pub type Address = usize;
pub type Source = usize;

pub trait AsNode {
	fn source(&self) -> Source;
	fn scope(&self) -> Scope;
	fn typ(&self) -> Address;
	fn set_source(&mut self, source: Source);
	fn set_scope(&mut self, scope: Scope);
	fn set_typ(&mut self, typ: Address);
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Attributes {
	pub source: Source, 
	pub scope: Scope, 
	pub typ: Address
}

impl AsNode for Attributes {
	fn source(&self) -> Source { self.source }
	fn scope(&self) -> Scope { self.scope }
	fn typ(&self) -> Address { self.typ }
	fn set_source(&mut self, source: Source) { self.source = source; }
	fn set_scope(&mut self, scope: Scope) { self.scope = scope; }
	fn set_typ(&mut self, typ: Address) { self.typ = typ; }
}