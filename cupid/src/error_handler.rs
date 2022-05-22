use crate::*;

pub trait ErrorHandler {
	fn get_token<'a>(&'a self, scope: &'a mut LexicalScope) -> &'a Token;
	fn get_context(&self) -> String;
	
	fn error<S>(&self, message: S, scope: &mut LexicalScope) -> Error where S: Into<String> + Clone {
		// println!("{}", message.to_owned().into());
		let token = self.get_token(scope);
		Error {
			line: token.line,
			index: token.index,
			message: message.into(),
			context: self.get_context(),
			source: token.source.to_owned(),
			file: token.file,
		}
	}
	fn error_context<S>(&self, message: S, context: S, scope: &mut LexicalScope) -> Error where S: Into<String> + Clone {
		// println!("{} {scope}", message.to_owned().into());
		let token = self.get_token(scope);
		Error {
			line: token.line,
			index: token.index,
			message: message.into(),
			context: context.into(),
			source: token.source.to_owned(),
			file: token.file,
		}
	}
}
