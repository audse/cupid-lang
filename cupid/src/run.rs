use cupid_fmt::reindent::{Multiline, Reindent};
use error::CupidErr;
use std::fs;
use std::process;
use vm::Vm;

use crate::analyze::{infer::Infer, pretty::PrettyPrint, resolve::Resolve};
use crate::arena::ExprArena;
use crate::ast::expr::Expr;
use crate::error::CupidError;
use crate::parse::{bytecode::BytecodeCompiler, parser::Parser};

/// 1. Resolve symbols (classes, functions, local variables, etc.)
/// 2. Infer types
/// 3. Resolve properties & methods
/// 4. Infer properties & methods
fn do_passes<'src>(
    expr: Vec<Expr<'src>>,
    arena: &mut ExprArena<'src>,
) -> Result<Vec<Expr<'src>>, CupidError> {
    expr.resolve(arena)?.infer(arena)?.resolve(arena)?.infer(arena)
}

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
            let expr = match do_passes(expr, &mut parser.arena) {
                Ok(expr) => expr,
                Err(err) => panic!("{}", err),
            };
            // println!("{expr:#?}");
            println!("{}", expr.pretty_print(&parser.arena).multiline(40).reindent(3));
            let compiler = BytecodeCompiler::new(expr, parser.arena, &mut vm.gc);
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
