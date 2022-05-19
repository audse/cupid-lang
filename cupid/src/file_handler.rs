use crate::*;

pub struct FileHandler {
	pub path: String,
	pub contents: String,
	pub parser: CupidParser,
	pub scope: LexicalScope,
	pub errors: Vec<Error>,
	pub warnings: Vec<Warning>,
	pub current_file: usize,
	pub files: Vec<String>,
}

impl FileHandler {
	
	pub fn new(path: &str) -> Self {
		let contents = std::fs::read_to_string(path)
				.unwrap_or_else(|_| String::from("Unable to find file"));
		let (parser, scope, errors, warnings) = FileHandler::build(&contents);
		Self {
			path: path.to_string(),
			contents,
			parser,
			scope,
			errors,
			warnings,
			current_file: 1,
			files: vec![path.to_string()],
		}
	}
	
	pub fn from(string: &str) -> Self {
		let contents = string.to_string();
		let (parser, scope, errors, warnings) = FileHandler::build(&contents);
		Self {
			path: String::new(),
			contents,
			parser,
			scope,
			errors,
			warnings,
			current_file: 1,
			files: vec![],
		}
	}
	
	pub fn build(contents: &str) -> (CupidParser, LexicalScope, Vec<Error>, Vec<Warning>) {
		let parser = CupidParser::new(contents.to_string(), 1);
		let scope = LexicalScope::default();
		(parser, scope, vec![], vec![])
	}
	
	pub fn run_package(&mut self, package: PackageContents) -> Result<(), Error> {
		self.report_loading_package(&package.path);
		
		self.current_file += 1;
		self.files.push(package.path.to_owned());
		let mut parser = CupidParser::new(package.contents.to_owned(), self.current_file);
		let parse_tree = parser._file();
		if let Some((mut tree, _)) = parse_tree {
			let semantics = match parse(&mut tree) {
				Ok(semantics) => semantics,
				Err(e) => {
					// println!("{tree}");
					panic!("{}", self.make_error_string(&e))
				}
			};
			if let Err(e) = semantics.resolve(&mut self.scope) {
				panic!("{}", self.make_error_string(&e));
			}
			self.report_loading_package_success();
		}
		Ok(())
	}
	
	pub fn preload_contents<S>(&mut self, string: S) -> Result<(), Error> where S: Into<String> {
		let mut parser = CupidParser::new(string.into(), self.current_file);
		let parse_tree = parser._file();
		let semantics = parse(&mut parse_tree.unwrap().0)?;
		semantics.resolve(&mut self.scope)?;
		Ok(())
	}
	
	pub fn run_debug(&mut self) -> Result<(), Error> {
		println!("Contents: {:?}", self.contents);
		
		self.use_packages(self.contents.to_owned())?;
		
		let parse_tree = self.parser._file();
		println!("Parse Tree: {}", if let Some((parse_tree, _)) = &parse_tree {
			parse_tree.to_string()
		} else {
			String::new()
		});
		
		let semantics = parse(&mut parse_tree.unwrap().0)?;
		println!("Semantics: {:#?}", semantics);
		
		self.scope.add(Context::Block);
		let result = match semantics.resolve(&mut self.scope) {
			Err(e) => {
				panic!("{}", self.make_error_string(&e))
			},
			Ok(val) => val,
		};
		println!("Scope: {}", self.scope);
		print!("\n\nResults: {}", result);
		Ok(())
	}
	
	pub fn parse(&mut self) -> Result<BoxAST, Error> {
		let parse_tree = self.parser._file();
		parse(&mut parse_tree.unwrap().0)
	}
	
	pub fn use_packages(&mut self, contents: String) -> Result<(), Error> {
		let mut package_parser = PackageParser::new(contents.to_owned(), self.current_file);
		if let Some((mut tree, _)) = package_parser._packages() {
			let package_semantics = parse_import(&mut tree);
			let packages: Vec<PackageContents> = package_semantics.use_packages();
			for package in packages {
				self.use_packages(package.contents.to_owned())?;
				self.run_package(package)?;
			}
		}
		Ok(())
	}
	
	pub fn run(&mut self) -> Result<(), Error> {
		self.report_build_started();
		
		self.use_packages(self.contents.to_owned())?;
		let parse_tree = self.parser._file();
		
		let semantics = match parse(&mut parse_tree.unwrap().0) {
			Ok(semantics) => semantics,
			Err(e) => panic!("{}", self.make_error_string(&e))
		};
		
		self.scope.add(Context::Block);
		match semantics.resolve(&mut self.scope) {
			Err(e) => {
				println!("{}", self.scope);
				panic!("{}", self.make_error_string(&e))
			},
			Ok(val) => val,
		};
		self.report_build_complete();
		Ok(())
	}
	
	pub fn get_line(&self, file: usize, index: usize) -> String {
		let contents = std::fs::read_to_string(self.files[file].to_owned()).unwrap();
		Self::get_line_of(contents, index)
	}
	
	pub fn get_line_of(contents: String, index: usize) -> String {
		contents
			.lines()
			.enumerate()
			.find(|(i, _l)| *i == index - 1)
			.unwrap_or((0, "unable to find line"))
			.1
			.to_string()
	}
	
	pub fn report_errors(&self) {
		self.errors
			.iter()
			.for_each(|e| println!("{}", self.make_error_string(e)));
	}
	
	pub fn make_error_string(&self, e: &Error) -> String {
		let path = &self.files[e.file - 1];
		let line = &self.get_line(e.file - 1, e.line);
		let underline_indent = vec![" "; e.index].join("");
		let number_indent = vec![" "; e.line.to_string().len()].join("");
		
		let overline = format!(
			"{number_indent} |",
			number_indent = number_indent
		);
		let line = format!(
			"{line_number} | {line}",
			line_number = e.line,
			line = line.trim()
		);
		let underline = format!(
			"{number_indent} | {underline_indent}{underline}",
			number_indent = number_indent,
			underline_indent = underline_indent,
			underline = vec!["^"; e.source.len()].join("").red().bold()
		);
		let context = format!("\n\t{} {}",
			"additional context:".to_string().bold(),
			if !e.context.is_empty() { 
				e.context.to_string()
			} else { 
				"none provided".to_string()
			}.italic()
		);
		return format!(
			"\n{error}\n\t{overline}\n\t{line}\n\t{underline}\n{context}\n\n",
			error = e.string(&path),
			overline = overline,
			line = line,
			underline = underline,
			context = context
		);
	}
	
	pub fn report_build_started(&self) {
		println!("\n{}\n", vec!["-"; 60].join(""));
		println!("{} {}\n", "Building".green().bold(), format!("{}...\n", self.path).bold());
	}
	
	pub fn report_loading_package(&self, path: &str) {
		print!("{} {}", "Loading".bold().italic(), format!("{path}...").italic());
	}
	pub fn report_loading_package_success(&self) {
		print!("{}\n", "success!".bold().italic().green());
	}
	
	pub fn report_build_complete(&self) {
		let num_errors = self.errors.len();
		let num_warnings = self.warnings.len();
		let error_message = format!("{} {}", num_errors, pluralize(num_errors, "error")).red();
		let warning_message = format!("{} {}", num_warnings, pluralize(num_warnings, "warning")).yellow();
		let build_message = format!(
			"Build finished with {} and {}.", 
			error_message, 
			warning_message
		).bold();
		println!("\n\n\n{}", build_message);
		println!("\n{}\n", vec!["-"; 60].join(""));
	}
}