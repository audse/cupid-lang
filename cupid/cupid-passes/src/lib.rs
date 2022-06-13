#![feature(derive_default_enum)]

pub mod linting;
pub mod name_resolution;
pub mod package_resolution;
pub mod pre_analysis;
pub mod scope_analysis;
pub mod states;
pub mod type_checking;
pub mod type_inference;
pub mod type_name_resolution;

pub type Closure = usize;
pub type Address = usize;
pub type Source = usize;
pub type ErrCode = usize;

pub type PassResult<T> = Result<T, (Source, ErrCode)>;

#[derive(Debug, Default, Clone)]
pub struct SemanticNode<T> {
	pub data: T,
	pub source: crate::Source,
	pub closure: crate::Closure,
    pub type_address: crate::Address,
}

pub trait AsNode {
	fn source(&self) -> Source;
	fn closure(&self) -> Closure;
	fn typ(&self) -> Address;
}

#[derive(Debug, Default, Clone)]
pub enum Value {
	VBoolean(bool),
	VChar(char),
	VInteger(i32),
	VString(std::borrow::Cow<'static, str>),

    #[default]
	VNone,
}
