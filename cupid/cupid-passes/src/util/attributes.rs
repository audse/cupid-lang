use crate::{Source, ScopeId, Address};

pub trait AsNode {
	fn source(&self) -> Source;
	fn scope(&self) -> ScopeId;
	fn typ(&self) -> Address;
	fn set_source(&mut self, source: Source);
	fn set_scope(&mut self, scope: ScopeId);
	fn set_typ(&mut self, typ: Address);
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Attributes {
	pub source: Source, 
	pub scope: ScopeId, 
	pub typ: Address
}

impl Attributes {
	pub fn new(source: Source, scope: ScopeId, typ: Address) -> Self {
		Self { source, scope, typ }
	}
}

impl AsNode for Attributes {
	fn source(&self) -> Source { self.source }
	fn scope(&self) -> ScopeId { self.scope }
	fn typ(&self) -> Address { self.typ }
	fn set_source(&mut self, source: Source) { self.source = source; }
	fn set_scope(&mut self, scope: ScopeId) { self.scope = scope; }
	fn set_typ(&mut self, typ: Address) { self.typ = typ; }
}