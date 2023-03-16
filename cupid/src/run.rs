use error::CupidErr;
use std::fs;
use std::process;
use vm::Vm;

use crate::analyze::infer::Infer;
use crate::analyze::resolve::Resolve;
use crate::parse::bytecode::BytecodeCompiler;
use crate::parse::parser::Parser;

pub fn run_file(vm: &mut Vm, path: &str) {
    let code = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprint!("Unable to read file {}: {}", path, error);
            process::exit(74);
        }
    };
    let mut parser = Parser::new(&*code);
    let expr = parser.parse(&mut vm.gc);

    match expr {
        Ok(expr) => {
            let expr = match expr.resolve() {
                Ok(expr) => expr,
                Err(err) => panic!("{}", err),
            };
            let expr = match expr.infer() {
                Ok(expr) => expr,
                Err(err) => panic!("{}", err),
            };
            // println!("{expr:#?}");
            let compiler = BytecodeCompiler::new(expr, &mut vm.gc);
            let function = compiler.compile();
            match vm.interpret_function(function) {
                Err(CupidErr::CompileError) => process::exit(65),
                Err(CupidErr::RuntimeError) => process::exit(70),
                Ok(_) => println!("Process exited successfully."),
            }
        }
        Err(error) => eprintln!("{error}"),
    }

    // if let Err(error) = vm.interpret(&code) {
    //     match error {
    //         CupidErr::CompileError => process::exit(65),
    //         CupidErr::RuntimeError => process::exit(70),
    //     }
    // }
}
