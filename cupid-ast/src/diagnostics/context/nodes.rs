use crate::*;

impl ToError for Block {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Block(self.to_owned()), code)
	}
}

impl ErrorContext for Block {
	fn context(&self, scope: &mut Env, source: &str) -> String {
		let node = self.source_node(scope);
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
	fn context(&self, scope: &mut Env, source: &str) -> String {
		self.to_string()
	}
}

impl ToError for Exp {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(self.to_owned(), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Exp {
	fn context(&self, scope: &mut Env, source: &str) -> String {
		for_each_exp!(self, context, scope, source)
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
	fn context(&self, scope: &mut Env, source: &str) -> String {
		self.to_string()
	}
}

impl ToError for Function {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Function(self.to_owned()), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Function {
	fn context(&self, scope: &mut Env, source: &str) -> String {
		self.to_string()
	}
}

impl ToError for Ident {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Ident(self.to_owned()), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Ident {
	fn context(&self, scope: &mut Env, source: &str) -> String {
		let source_node = self.source_node(scope);
		let token = source_node.token(0);
		let lines = token.lines(source);
		let underlines = token.underline(lines);
		format!(
			"{}\nAccessing identifier `{}`\n   |\n{}\n", 
			scope.closures[scope.current_closure].1,
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
	fn context(&self, scope: &mut Env, source: &str) -> String {
		self.to_string()
	}
}

impl ToError for Property {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Property(Box::new(self.to_owned())), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Property {
	fn context(&self, scope: &mut Env, source: &str) -> String {
		self.to_string()
	}
}

impl ToError for PropertyTerm {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(self.to_owned().into(), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for PropertyTerm {
	fn context(&self, scope: &mut Env, source: &str) -> String {
		self.to_string()
	}
}

impl ToError for Method {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Function(self.to_owned().value), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Method {
	fn context(&self, scope: &mut Env, source: &str) -> String {
		self.to_string()
	}
}

impl ToError for TraitDef {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::TraitDef(self.to_owned()), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for TraitDef {
	fn context(&self, scope: &mut Env, source: &str) -> String {
		self.to_string()
	}
}

impl ToError for TypeDef {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::TypeDef(self.to_owned()), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for TypeDef {
	fn context(&self, scope: &mut Env, source: &str) -> String {
		self.to_string()
	}
}


impl ToError for Val {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Value(self.to_owned().into()), code)
	}
}
#[allow(unused_variables)]
impl ErrorContext for Val {
	fn context(&self, scope: &mut Env, source: &str) -> String {
		self.to_string()
	}
}

impl ToError for Value {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Value(self.to_owned()), code)
	}
}

#[allow(unused_variables)]
impl ErrorContext for Value {
	fn context(&self, scope: &mut Env, source: &str) -> String {
		self.to_string()
	}
}

impl<T: ToError + Default + std::fmt::Debug> ToError for Typed<T> {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		self.inner().as_err(code)
	}
}

#[allow(unused_variables)]
impl<T: ErrorContext + Default + std::fmt::Debug + std::fmt::Display> ErrorContext for Typed<T> {
	fn context(&self, scope: &mut Env, source: &str) -> String {
		self.to_string()
	}
}