use crate::Attributes;
use Value::*;

#[derive(Debug, Clone)]
pub enum Value {
	VBoolean(bool, Attributes),
	VChar(char, Attributes),
	VDecimal(i32, u32, Attributes),
	VInteger(i32, Attributes),
	VString(cupid_util::Str, Attributes),
	VType(crate::Typ),
	VNone(Attributes),
}


impl Default for Value {
	fn default() -> Self {
		Self::VNone(Attributes::default())
	}
}

impl Value {
	pub fn attr(&self) -> Attributes {
		match self {
			VBoolean(_, attr)
			| VChar(_, attr)
			| VDecimal(_, _, attr)
			| VInteger(_, attr)
			| VString(_, attr)
			| VNone(attr) => *attr,
			VType(t) => t.attr,
		}
	}
	pub fn attr_mut(&mut self) -> &mut Attributes {
		match self {
			VBoolean(_, attr)
			| VChar(_, attr)
			| VDecimal(_, _, attr)
			| VInteger(_, attr)
			| VString(_, attr)
			| VNone(attr) => attr,
			VType(t) => &mut t.attr,
		}
	}
}

impl crate::AsNode for Value {
	fn source(&self) -> crate::Source { self.attr().source() }
	fn scope(&self) -> crate::ScopeId { self.attr().scope() }
	fn typ(&self) -> crate::Address { self.attr().typ() }
	fn set_source(&mut self, source: crate::Source) { self.attr_mut().source = source }
	fn set_scope(&mut self, scope: crate::ScopeId) { self.attr_mut().scope = scope }
	fn set_typ(&mut self, typ: crate::Address) { self.attr_mut().typ = typ }
}