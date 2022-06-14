#![feature(derive_default_enum)]

pub mod pass;

pub mod linting;
pub mod name_resolution;
pub mod package_resolution;
pub mod pre_analysis;
pub mod scope_analysis;
pub mod type_checking;
pub mod type_inference;
pub mod type_name_resolution;

pub(crate) use Value::*;

pub type Scope = usize;
pub type Address = usize;
pub type Source = usize;
pub type ErrCode = usize;

pub type PassResult<T> = Result<T, (Source, ErrCode)>;

pub trait AsNode {
	fn source(&self) -> Source;
	fn scope(&self) -> Scope;
	fn typ(&self) -> Address;
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
}


// `Block` is always the same shape, so it does not need
// separate definitions in each pass
cupid_util::node_builder! {
    #[derive(Debug, Default, Clone)]
    pub BlockBuilder => pub Block<E: Default + Clone> {
        pub expressions: Vec<E>,
    }
}


#[derive(Debug, Clone)]
pub enum Value {
	VBoolean(bool, Attributes),
	VChar(char, Attributes),
	VDecimal(i32, u32, Attributes),
	VInteger(i32, Attributes),
	VString(cupid_util::Str, Attributes),
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
			| VNone(attr) => *attr
		}
	}
}

impl AsNode for Value {
	fn source(&self) -> Source { self.attr().source() }
	fn scope(&self) -> Scope { self.attr().scope() }
	fn typ(&self) -> Address { self.attr().typ() }
}
