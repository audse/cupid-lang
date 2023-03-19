use crate::vm::Vm;
use std::io::{self, Write};

pub fn repl(_vm: &mut Vm) {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Unable to read line from the REPL");
        if line.is_empty() {
            break;
        }
        // vm.interpret(&line).ok();
    }
}
