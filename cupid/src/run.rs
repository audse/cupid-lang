use error::CupidError;
use std::fs;
use std::process;
use vm::Vm;

pub fn run_file(vm: &mut Vm, path: &str) {
    let code = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprint!("Unable to read file {}: {}", path, error);
            process::exit(74);
        }
    };

    if let Err(error) = vm.interpret(&code) {
        match error {
            CupidError::CompileError => process::exit(65),
            CupidError::RuntimeError => process::exit(70),
        }
    }
}
