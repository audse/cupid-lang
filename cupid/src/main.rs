// use cupid::*;
// use clap::Parser;

// use cupid_parse::{
// 	create::CreateAST,
// 	parsers::CupidParser,
// 	run::use_generator,
// };
// use cupid_ast::{
// 	ASTResult,
// 	Exp,
// 	UseAttributes,
// };
// use cupid_analysis::Analyze;
// use cupid_debug::ErrorContext;

// #[derive(Parser)]
// struct Cli {
//     pattern: String,
    
//     #[clap(default_value_t = String::from("main.cupid"))]
//     path: String,
    
//     #[clap(short, long)]
//     debug: bool,
    
//     #[clap(short, long)]
//     generate: Option<i32>,
	
// 	#[clap(short, long)]
// 	repl: bool,
// }

// fn main() -> Result<(), String> {
//     let args = Cli::parse();
	
// 	if args.repl {
// 		let mut parser = CupidParser::new(String::new(), 1);
// 		let mut env = cupid_scope::Env::default();
// 		match run_repl(&mut parser, &mut env) {
// 			Ok(()) => (),
// 			Err((src, code)) => {
// 				let node = env.get_source_node(src.source());
// 				eprintln!("{}", src.context(node, ""));
// 				panic!("{}", src.message(code));
// 			}
// 		}
// 	} else if let Some(which) = args.generate {
//         run_generator(which);
// 	} else {
// 		run_path(&format!("./../apps/{}", args.path), args.debug)
// 	}

// 	Ok(())
// }

// fn run_repl(parser: &mut CupidParser, env: &mut cupid_scope::Env) -> ASTResult<()> {
// 	loop {
// 		let mut line = String::new();
// 		std::io::stdin().read_line(&mut line).unwrap();
// 		parser.update(line.to_owned(), 1);
		
// 		let (mut parse_tree, _) = parser._expression().unwrap();
// 		let mut ast = Exp::create_ast(&mut parse_tree, env).map_err(|e| (Exp::Empty, e))?;
		
// 		ast.analyze_names(env)?;
// 		ast.analyze_types(env)?;
// 		ast.check_types(env)?;
// 	}
// }

// fn run_path(path: &str, debug: bool) {
// 	let mut file_handler = file_handler::FileHandler::build()
// 		.read(path)
// 		.debug(debug)
// 		.build();
// 	file_handler.run();
// }

// fn run_generator(which: i32) {
//     use_generator(which);
// }

fn main() {}