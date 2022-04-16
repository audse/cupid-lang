use cupid::*;

use clap::Parser;


#[derive(Parser)]
struct Cli {
    pattern: String,
    
    #[clap(default_value_t = String::from("library.cupid"))]
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
        run_path(&args.path, args.debug);
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