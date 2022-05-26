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

fn main() -> Result<(), String> {
    let args = Cli::parse();
	
	let mut parser = CupidParser::new(String::new(), 1);
	let mut env = Env::default();
	
	macro_rules! add_globals {
		($env:ident, $($global:ident),*) => {
			$(
				$env.add_global(&*$global);
			)*
		}
	}
	
	// core types
	add_globals!(env, BOOLEAN, DECIMAL, INTEGER, CHARACTER, STRING, FUNCTION, ARRAY, TUPLE, MAYBE, NOTHING);
	
	// core traits
	add_globals!(env, ADD, SUBTRACT, EQUAL, NOT_EQUAL, GET);
	
	if args.repl {
		match run_repl(&mut parser, &mut env) {
			Ok(()) => (),
			Err((src, code)) => panic!("{}", err_from_code(src, code, &mut env))
		}
	}
	
    if let Some(which) = args.generate {
        run_generator(which);
	}
	Ok(())
}

fn run_repl(parser: &mut CupidParser, env: &mut Env) -> Result<(), (Source, ErrCode)> {
	loop {
		let mut line = String::new();
		std::io::stdin().read_line(&mut line).unwrap();
		parser.update(line.to_owned(), 1);
		
		let (mut parse_tree, _) = parser._expression().unwrap();
		let mut ast = create_ast(&mut parse_tree, env).map_err(|e| (0, e))?;
		
		ast.analyze_names(env)?;
		ast.analyze_types(env)?;
		ast.check_types(env)?;
	}
}

// fn run_path(path: &str, debug: bool)-> Result<(), Error> {
// 	let mut file_handler = FileHandler::new(format!("./../apps/{}", path).as_str());
// 	if debug {
// 		file_handler.run_debug()
// 	} else {
// 		file_handler.run()
// 	}
// }

fn run_generator(which: i32) {
    use_generator(which);
}