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
		let parser = CupidParser::new(contents.clone());
		let mut scope = LexicalScope::default();
		use_builtin_types(&mut scope);
		
		Self {
			path: path.to_string(),
			contents,
			parser,
			scope,
			errors: vec![],
			warnings: vec![],
		}
	}
	
	pub fn run_debug(&mut self) {
		println!("Contents: {:?}", self.contents);
		
		let parse_tree = self.parser._file(None);
		println!("Parse Tree: {:#?}", parse_tree);
		
		let semantics = to_tree(&parse_tree.unwrap().0);
		println!("Semantics: {:#?}", semantics);
		
		let result = semantics.resolve_file(&mut self.scope);
		result.iter().for_each(|r| println!("{:?}", r));
	}
	
	pub fn run(&mut self) {
		self.report_build_started();
		
		let parse_tree = self.parser._file(None);        
		let semantics = to_tree(&parse_tree.unwrap().0);
		
		let result = semantics.resolve_file(&mut self.scope);
		// for error in result {
		//     
		// }
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
		return format!(
			"{error}\t{overline}\n\t{line}\n\t{underline}",
			error = e.string(&self.path),
			overline = overline,
			line = line,
			underline = underline,
		);
	}
	
	pub fn report_build_started(&self) {
		println!("\n{}\n", vec!["-"; 60].join(""));
		println!("{} {}\n", "Building".green().bold(), format!("{}...\n", self.path).bold());
	}
	
	pub fn report_build_complete(&self) {
		let error_message = format!("{} errors", self.errors.len()).red();
		let warning_message = format!("{} warnings", self.warnings.len()).yellow();
		let build_message = format!(
			"Build finished with {} and {}.", 
			error_message, 
			warning_message
		).bold();
		println!("\n\n\n{}", build_message);
		println!("\n{}\n", vec!["-"; 60].join(""));
	}
}
