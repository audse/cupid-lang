use crate::{Source, ScopeId};

pub trait AsNode {
	fn source(&self) -> Source;
	fn scope(&self) -> ScopeId;
	fn set_source(&mut self, source: Source);
	fn set_scope(&mut self, scope: ScopeId);
	fn err(&self, code: crate::ErrCode) -> (Source, crate::ErrCode) {
		(self.source(), code)
	}
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Attributes {
	pub source: Source, 
	pub scope: ScopeId,
}

impl Attributes {
	pub fn new(source: Source, scope: ScopeId) -> Self {
		Self { source, scope }
	}
}

impl AsNode for Attributes {
	fn source(&self) -> Source { self.source }
	fn scope(&self) -> ScopeId { self.scope }
	fn set_source(&mut self, source: Source) { self.source = source; }
	fn set_scope(&mut self, scope: ScopeId) { self.scope = scope; }
}