use clap::Parser;
use cupid::*;
use cupid_ast::*;

#[derive(Parser)]
struct Cli {
    pattern: String,
    
    #[clap(default_value_t = String::from("main.cupid"))]
    path: String,
    
    #[clap(short, long)]
    debug: bool,
    
    #[clap(short, long)]
    generate: Option<i32>,
	
	#[clap(short, long)]
	repl: bool,
}

fn main() {
    let args = Cli::parse();
	
	if args.repl {
		loop {
			let mut line = String::new();
			std::io::stdin().read_line(&mut line).unwrap();
			
			if &line == "exit" {
				break;
			}
			
			let mut parser = CupidParser::new(line, 1);
			let (mut parse_tree, _) = parser._expression().unwrap();
			let mut env = Env::default();
			let ast = to_ast(&mut parse_tree, &mut env);
			println!("{ast:?}");
		}
	}
	
    if let Some(which) = args.generate {
        run_generator(which);
	} else {
        match run_path(&args.path, args.debug) {
			Err(e) => eprintln!("{e}"),
			_ => ()
		}
    }
}

fn run_path(path: &str, debug: bool)-> Result<(), Error> {
	let mut file_handler = FileHandler::new(format!("./../apps/{}", path).as_str());
	if debug {
		file_handler.run_debug()
	} else {
		file_handler.run()
	}
}

fn run_generator(which: i32) {
    use_generator(which);
}