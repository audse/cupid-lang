pub mod analyze;
pub mod arena;
pub mod ast;
pub mod chunk;
pub mod compiler;
pub mod cst;
pub mod error;
pub mod expose;
pub mod gc;
pub mod objects;
pub mod parse;
pub mod pointer;
pub mod repl;
pub mod run;
pub mod scanner;
pub mod scope;
pub mod span;
pub mod table;
pub mod token;
pub mod ty;
pub mod value;
pub mod vm;

use std::env;
use std::process;

extern crate cupid_fmt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vm = vm::Vm::default();
    vm.initialize();
    match args.len() {
        1 => repl::repl(&mut vm),
        2 => run::run_file(&mut vm, &args[1]),
        _ => process::exit(64),
    }
}
