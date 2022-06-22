#![feature(try_trait_v2, let_chains, derive_default_enum)]

pub(crate) use std::borrow::Cow;
use cupid_ast::*;
use cupid_lex::*;
use cupid_scope::*;
use cupid_util::*;

pub mod create;
pub mod generator;
pub mod parse;
pub mod parser;
pub mod parsers;
pub mod run;