#![feature(derive_default_enum, let_chains)]

#[doc(hidden)]
use colored::Colorize;
use cupid_util::*;

pub mod errors;
pub mod lexer;
pub mod node;
pub mod token;
pub mod tokenizer;