use colored::Colorize;
use crate::*;

impl ToError for Block {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Block(self.to_owned()), code)
	}
}

impl ErrorContext for Block {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		let (open_token, close_token) = (node.token(0), node.token(1));
		let underlines = match open_token.source() {
			"=" => {
				let delimiter = open_token.combine(close_token);
				let lines = delimiter.lines(source);
				delimiter.underline(lines)
			},
			"{" => {
				let (open_lines, close_lines) = (
					open_token.lines(source),
					close_token.lines(source)
				);
				let (mut open_underline, mut close_underline) = (
					open_token.underline(open_lines),
					close_token.underline(close_lines)
				);
				if close_token.line > open_token.line - 1 {
					open_underline.append(&mut vec![("...".to_string(), "".to_string())])
				}
				open_underline.append(&mut close_underline);
				open_underline
			},
			_ => exhaustive!("block")
		};
		underlines
			.iter()
			.map(|(line, underline)| line.to_owned() + "\n" + underline)
			.collect::<Vec<String>>()
			.join("\n")
	}
	fn message(&self, code: ErrCode) -> String {
		match code {
			ERR_UNCLOSED_DELIMITER => "Unclosed delimiter: we can't find a matching closing brace".to_string(),
			_ => exhaustive!("block")
		}
	}
}

impl ToError for Declaration {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Declaration(self.to_owned()), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Declaration {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		self.fmt_node()
	}
}

impl ToError for Exp {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(self.to_owned(), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Exp {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		for_each_exp!(self, context, node, source)
		// todo!()
	}
	fn message(&self, code: ErrCode) -> String {
		for_each_exp!(self, message, code)
	}
}

impl ToError for FunctionCall {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::FunctionCall(Box::new(self.to_owned())), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for FunctionCall {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		self.fmt_node()
	}
}

impl ToError for Function {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Function(self.to_owned()), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Function {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		self.fmt_node()
	}
}

impl ToError for Ident {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Ident(self.to_owned()), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Ident {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		let token = if node.tokens.is_empty() {
			if node.children[0].tokens.is_empty() {
				node.children[0].children[0].token(0)
			} else {
				node.children[0].token(0)
			}
		} else {
			node.token(0)
		};
		let lines = token.lines(source);
		let underlines = token.underline(lines);
		format!(
			"\nAccessing identifier `{}`\n\n{}\n", 
			(&*self.name).bold().yellow(),
			underlines
				.iter()
				.map(|(line, underline)| line.to_owned() + "\n" + underline)
				.collect::<Vec<String>>()
				.join("\n")
		)
	}
}

impl ToError for Implement {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Implement(self.to_owned()), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Implement {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		self.fmt_node()
	}
}

impl ToError for Property {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Property(Box::new(self.to_owned())), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Property {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		(self as &dyn Fmt).fmt_node()
	}
}

impl ToError for PropertyTerm {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(self.to_owned().into(), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for PropertyTerm {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		self.fmt_node()
	}
}

impl ToError for Method {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Function(self.to_owned().value), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Method {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		self.fmt_node()
	}
}

impl ToError for SymbolValue {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Value(self.value.to_owned().unwrap_or_default()), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for SymbolValue {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		self.fmt_node()
	}
}

impl ToError for Trait {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Value(VTrait(self.to_owned())), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Trait {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		self.fmt_node()
	}
}

impl ToError for TraitDef {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::TraitDef(self.to_owned()), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for TraitDef {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		self.fmt_node()
	}
}

impl ToError for Type {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Value(VType(self.to_owned())), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Type {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		self.fmt_node()
	}
}

impl ToError for TypeDef {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::TypeDef(self.to_owned()), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for TypeDef {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		self.fmt_node()
	}
}

impl ToError for Value {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Value(self.to_owned()), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Value {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		self.fmt_node()
	}
}

impl<T: ToError + Default + std::fmt::Debug> ToError for Typed<T> {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		self.inner().as_err(code)
	}
}

#[allow(unused_variables)]
impl<T: ErrorContext + Default + std::fmt::Debug + std::fmt::Display + Clone> ErrorContext for Typed<T> {
	fn context(&self, node: &ParseNode, source: &str) -> String {
		self.to_string()
	}
}