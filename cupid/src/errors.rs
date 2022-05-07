use std::fmt::{Display, Formatter, Result};
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Error {
    pub line: usize,
    pub index: usize,
    pub message: String,
    pub source: String,
    pub context: String,
}

impl Error {
    
    pub fn from_token(token: &Token, message: &str, context: &str) -> Error {
        Error {
            line: token.line,
            index: token.index,
            source: token.source.clone(),
            message: String::from(message),
            context: String::from(context),
        }
    }
    
    pub fn string(&self, path: &str) -> String {
        let header = "error:".bright_red().bold();
        let message = self.message.bold();
        let arrow = "  -->  ".dimmed().bold();
        let token = format!("`{}`", self.source).bold().yellow();
        let meta = format!("{}{} - line {}:{} (at {})", arrow, path, self.line, self.index, token).italic();
        format!("\n{} {}\n{}\n", header, message, meta)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let header = "error:".bright_red().bold();
        let message = self.message.bold();
        let arrow = "  -->  ".dimmed().bold();
        let token = format!("`{}`", self.source).bold().yellow();
        let meta = format!("{}line {}:{} (at {})", arrow, self.line, self.index, token).italic();
        write!(f, "\n{} {}\n{}\n", header, message, meta)
    }
}

pub struct Warning {
    pub line: usize,
    pub index: usize,
    pub message: String,
    pub source: String,
    pub context: String,
}

pub trait ErrorHandler {
    fn get_token(&self) -> &Token;
    fn get_context(&self) -> String;
    
    fn type_error(&self, value: &Value, expected: TypeKind) -> Value {
        self.error(format!(
            "type mismatch: expected {}, found {}",
            expected,
            TypeKind::infer(value)
        ))
    }
    fn undefined_error(&self, identifier: String) -> Value {
        self.error(format!(
            "undefined: {} wasn't found in the current scope",
            identifier
        ))
    }
    fn unable_to_assign_error(&self, identifier: String, assign_value: Value) -> Value {
        self.error(format!(
            "unable to assign {} to `{}`",
            assign_value,
            identifier
        ))
    }
    fn error<S>(&self, message: S) -> Value where S: Into<String> {
        Value::error(self.get_token(), message.into(), self.get_context())
    }
    fn error_context<S>(&self, message: S, context: S) -> Value where S: Into<String> {
        Value::error(self.get_token(), message.into(), context.into())
    }
	fn error_raw<S>(&self, message: S) -> Error where S: Into<String> {
		let token = self.get_token();
		Error {
			line: token.line,
			index: token.index,
			message: message.into(),
			context: self.get_context(),
			source: token.source.clone()
		}
	}
	fn error_raw_context<S>(&self, message: S, context: S) -> Error where S: Into<String> {
		let token = self.get_token();
		Error {
			line: token.line,
			index: token.index,
			message: message.into(),
			context: context.into(),
			source: token.source.clone()
		}
	}
}