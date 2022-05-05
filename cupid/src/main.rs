use cupid::*;
// mod lib;
// use lib::{FileHandler}

use clap::Parser;

use cupid::parse;

#[derive(Parser)]
struct Cli {
    pattern: String,
    
    #[clap(default_value_t = String::from("refactor.cupid"))]
    path: String,
    
    #[clap(short, long)]
    debug: bool,
    
    #[clap(short, long)]
    generate: bool,
	
	#[clap(short, long)]
	refactor: bool,
}

fn main() {
    let args = Cli::parse();
    if args.generate {
        run_generator();
	} else if args.refactor {
		refactor_parse(&args.path, args.debug);
    } else {
        run_path(&args.path, args.debug);
    }
}

fn refactor_parse(path: &str, debug: bool) {
	let mut file_handler = RFileHandler::new(format!("src/tests/{}", path).as_str());
	if debug {
		file_handler.run_debug()
	} else {
		file_handler.run();
	}
}

fn run_path(path: &str, debug: bool) {
	
    let mut file_handler = FileHandler::new(format!("src/tests/{}", path).as_str());
    if debug {
        file_handler.run_debug()
    } else {
        file_handler.run();
    }
}

fn run_generator() {
    test_generator();
}