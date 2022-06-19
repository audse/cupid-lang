use crate::{Address, ScopeId};

pub trait AsNode {
	fn address(&self) -> Address;
	fn scope(&self) -> ScopeId;
	fn set_scope(&mut self, scope: ScopeId);
	fn err(&self, code: crate::ErrCode) -> (Address, crate::ErrCode) {
		(self.address(), code)
	}
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Attributes {
	pub address: Address, 
	pub scope: ScopeId,
}

impl Attributes {
	pub fn new(address: Address, scope: ScopeId) -> Self {
		Self { address, scope }
	}
}

impl AsNode for Attributes {
	fn address(&self) -> Address { self.address }
	fn scope(&self) -> ScopeId { self.scope }
	fn set_scope(&mut self, scope: ScopeId) { self.scope = scope; }
}