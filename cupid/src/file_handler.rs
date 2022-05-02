use crate::*;

pub struct FileHandler {
	pub path: String,
	pub contents: String,
	pub parser: CupidParser,
	pub scope: LexicalScope,
	pub errors: Vec<Error>,
	pub warnings: Vec<Warning>,
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
			warnings
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
			warnings
		}
	}
	
	pub fn build(contents: &str) -> (CupidParser, LexicalScope, Vec<Error>, Vec<Warning>) {
		let parser = CupidParser::new(contents.to_string());
		let scope = LexicalScope::default();
		(parser, scope, vec![], vec![])
	}
	
	pub fn use_stdlib(&mut self) {
		let packages = vec![
			"./../stdlib/typedef.cupid",
			"./../stdlib/decimal.cupid",
			"./../stdlib/integer.cupid",
		];
		let stdlib: Vec<String> = packages
			.iter()
			.map(|p| std::fs::read_to_string(p).unwrap_or_else(|_| String::from("Unable to find file")))
			.collect();
		let stdlib = stdlib.join("\n");
		
		let mut parser = CupidParser::new(stdlib);
		let parse_tree = parser._file(None);
		let semantics = to_tree(&parse_tree.unwrap().0);
		
		semantics.resolve_file(&mut self.scope);
	}
	
	pub fn preload_contents<S>(&mut self, string: S) where S: Into<String> {
		let mut parser = CupidParser::new(string.into());
		let parse_tree = parser._file(None);
		let semantics = to_tree(&parse_tree.unwrap().0);
		semantics.resolve_file(&mut self.scope);
	}
	
	pub fn run_debug(&mut self) {
		self.use_stdlib();
		
		println!("Contents: {:?}", self.contents);
		
		let parse_tree = self.parser._file(None);
		println!("Parse Tree: {:#?}", parse_tree);
		
		let semantics = to_tree(&parse_tree.unwrap().0);
		println!("Semantics: {:#?}", semantics);
		
		self.scope.add(ScopeContext::Block);
		let result = semantics.resolve_file(&mut self.scope);
		print!("\n\nResults:");
		result.iter().for_each(|r| println!("\n{}", r));
	}
	
	pub fn run_and_return(&mut self) -> Vec<Value> {
		self.use_stdlib();
		
		let parse_tree = self.parser._file(None);        
		let semantics = to_tree(&parse_tree.unwrap().0);
		
		self.scope.add(ScopeContext::Block);
		semantics.resolve_file(&mut self.scope)
	}
	
	pub fn parse(&mut self) -> Expression {
		let parse_tree = self.parser._file(None);
		to_tree(&parse_tree.unwrap().0)
	}
	
	pub fn run(&mut self) {
		self.report_build_started();
		
		self.use_stdlib();
		
		let parse_tree = self.parser._file(None);        
		let semantics = to_tree(&parse_tree.unwrap().0);
		
		self.scope.add(ScopeContext::Block);
		let result = semantics.resolve_file(&mut self.scope);
		self.errors = result
			.iter()
			.filter_map(|r| match r {
				Value::Error(e) => Some(e),
				_ => None
			})
			.cloned()
			.collect();
		self.report_errors();
		
		self.report_build_complete()
	}
	
	pub fn get_line(&self, index: usize) -> &str {
		self.contents
			.lines()
			.enumerate()
			.find(|(i, _l)| *i == index - 1)
			.unwrap_or((0, "unable to find line"))
			.1
	}
	
	pub fn report_errors(&self) {
		self.errors
			.iter()
			.for_each(|e| println!("{}", self.make_error_string(e)));
	}
	
	pub fn make_error_string(&self, e: &Error) -> String {
		let line = self.get_line(e.line).trim();
		let underline_indent = vec![" "; e.index].join("");
		let number_indent = vec![" "; e.line.to_string().len()].join("");
		
		let overline = format!(
			"{number_indent} |",
			number_indent = number_indent
		);
		let line = format!(
			"{line_number} | {line}",
			line_number = e.line,
			line = line
		);
		let underline = format!(
			"{number_indent} | {underline_indent}{underline}",
			number_indent = number_indent,
			underline_indent = underline_indent,
			underline = vec!["^"; e.source.len()].join("").red().bold()
		);
		let context = format!("\n\t{} {}",
			"additional context:".to_string().bold(),
			if e.context.len() > 0 { 
				e.context.to_string()
			} else { 
				"none provided".to_string()
			}.italic()
		);
		return format!(
			"{error}\n\t{overline}\n\t{line}\n\t{underline}\n{context}",
			error = e.string(&self.path),
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
