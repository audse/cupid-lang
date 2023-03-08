pub mod chunk;
pub mod compiler;
pub mod error;
pub mod expose;
pub mod gc;
pub mod objects;
pub mod parser;
pub mod repl;
pub mod run;
pub mod scanner;
pub mod span;
pub mod table;
pub mod token;
pub mod value;
pub mod vm;

use std::env;
use std::process;

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
