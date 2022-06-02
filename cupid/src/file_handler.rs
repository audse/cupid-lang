use std::time::{
	Instant,
	Duration,
};
use crate::*;

build_struct! {
	#[derive(Debug, Clone, Default)]
	pub FileHandlerBuilder => pub FileHandler {
		pub path: String,
		pub contents: String,
		pub parser: CupidParser,
		pub scope: Env,
		pub files: Vec<String>,
		pub debug: bool,
		reporter: BuildReporter,
	}
}

impl FileHandlerBuilder {
	pub fn read(mut self, path: &str) -> Self {
		self.path = path.to_string();
		self.contents = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to find file: {path}"));
		self
	}
}

impl FileHandler {
	pub fn run(&mut self) {
		self.reporter.report_start(&self.path);

		self.parser.update(std::mem::take(&mut self.contents), 0);
		if let Err((src, code)) = self.parse_analyze() {
			if self.debug {
				eprintln!("{}", fmt_list!(self.scope.traceback, "\n"));
				// eprintln!("{}", self.scope);
			}
			panic!("{}", err_from_code(src, code, &mut self.scope))
		}

		self.reporter.report_complete();
	}
	pub fn parse_analyze(&mut self) -> Result<(), (Source, ErrCode)> {
		let (mut parse_tree, _) = self.parser._file().unwrap();
		let mut ast = create_file_ast(&mut parse_tree, &mut self.scope).map_err(|e| (0, e))?;

		if self.debug {
			println!("\nParsing...\n");
			for node in ast.iter_mut() {
				println!("{}", node.as_table())
			}
		}

		macro_rules! do_passes {
			($($method:ident),*) => {
				$(
					for exp in ast.iter_mut() {
						exp.$method(&mut self.scope)?;
					}
				)*
			};
		}

		do_passes! {
			pre_analyze_scope,
			pre_analyze_names,
			pre_analyze_types,
			analyze_scope,
			analyze_names,
			analyze_types,
			check_types
		}

		if self.debug {
			println!("\n\nAnalyzing...\n");
			for node in ast.iter_mut() {
				println!("{}", node.as_table())
			}
		}

		Ok(())
	}
}

#[derive(Debug, Clone)]
pub struct BuildReporter {
	start: Instant,
	stop: Duration,
	errors: Vec<Error>,
}

impl Default for BuildReporter {
	fn default() -> Self {
		Self { start: Instant::now(), stop: Duration::default(), errors: vec![] }
	}
}

impl BuildReporter {
	fn report_start(&mut self, path: &str) {
		self.start = Instant::now();
		println!("\n{:-60}\n", "");
		println!("{} {}\n", "Building".green().bold(), format!("{path}...\n").bold());
	}
	fn report_complete(&mut self) {
		self.stop = self.start.elapsed();

		let num_errors = self.errors.len();
		let error_message = format!("{} {}", num_errors, pluralize(num_errors, "error")).red();
		let warning_message = "0 warnings".yellow();

		let build_message = format!(
			"Build finished after {:.4?} with {} and {}.", 
			self.stop,
			error_message, 
			warning_message
		).bold();

		println!("\n\n\n{}", build_message);
		println!("\n{:-60}\n", "");
	}
}