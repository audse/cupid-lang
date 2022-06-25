#![feature(derive_default_enum)]

pub(self) type Address = usize;
pub(self) type ScopeId = usize;

pub mod database;
pub mod environment;
pub mod scope;
