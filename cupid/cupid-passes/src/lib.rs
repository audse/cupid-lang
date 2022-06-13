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

pub(crate) use Value::*;

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

#[derive(Debug, Default, Copy, Clone)]
pub struct Attributes(Source, Closure, Address);
impl AsNode for Attributes {
	fn source(&self) -> Source { self.0 }
	fn closure(&self) -> Closure { self.1 }
	fn typ(&self) -> Address { self.2 }
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
	fn closure(&self) -> Closure { self.attr().closure() }
	fn typ(&self) -> Address { self.attr().typ() }
}

#[macro_export]
macro_rules! ast_pass_nodes {
    (
        Decl: $decl:item
        Ident: $ident:item
    ) => {
        #[derive(Debug, Default, Clone)]
        pub enum Expr {
            Decl(Decl),
            Ident(Ident),
			Value(crate::Value),

            #[default]
            Empty,
        }

        $decl
        $ident
    }
}