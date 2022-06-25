#![feature(derive_default_enum)]

pub(self) type Address = usize;

pub mod attr;
pub mod expr;
pub mod stmt;
pub mod types;