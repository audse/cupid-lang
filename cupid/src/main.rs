use cupid::*;
// mod lib;
// use lib::{FileHandler}

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    
    #[clap(default_value_t = String::from("refactor.cupid"))]
    path: String,
    
    #[clap(short, long)]
    debug: bool,
    
    #[clap(short, long)]
    generate: bool,
}

fn main() {
    let args = Cli::parse();
    if args.generate {
        run_generator();
	} else {
        match run_path(&args.path, args.debug) {
			Err(e) => eprintln!("{e}"),
			_ => ()
		}
    }
}

fn run_path(path: &str, debug: bool)-> Result<(), Error> {
	let mut file_handler = FileHandler::new(format!("src/tests/{}", path).as_str());
	if debug {
		file_handler.run_debug()
	} else {
		file_handler.run()
	}
}

fn run_generator() {
    test_generator();
}