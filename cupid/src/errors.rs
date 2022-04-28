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
}

pub trait MapErrorHandler: ErrorHandler {
    fn not_map_error(&self, value: &Value) -> Value {
        self.error(format!(
            "type mismatch: expected dictionary, list, or tuple, not {} ({})",
            value,
            TypeKind::infer(value)
        ))
    }
    fn not_map_type_error(&self, other_type: TypeKind) -> Value {
        self.error(format!(
            "type mismatch: expected dictionary, list, or tuple, not {}",
            other_type
        ))
    }
    fn no_property_error(&self, identifier: &Value, property: &Value) -> Value {
        self.error(format!(
            "undefined: `{}` doesn't have property `{}`",
            identifier,
            property
        ))
    }
}

macro_rules! abort_on_error {
    ($val:ident) => {{ 
        if $val.is_poisoned() {
            return $val; 
        } 
    }};
    ($val:ident, $pre_abort:block) => {{ 
        if $val.is_poisoned() { 
            $pre_abort
            return $val; 
        } 
    }}
}

macro_rules! resolve_or_abort {
    ($val:expr, $scope:expr) => {{
        let val = $val.resolve($scope);
        if val.is_poisoned() {
            return val;
        } else {
            val
        }
    }};
    
    ($val:expr, $scope:ident, $pre_abort:block) => {{
        let val = $val.resolve($scope);
        if val.is_poisoned() {
            $pre_abort
            return val;
        } else {
            val
        }
    }}
}

pub(crate) use abort_on_error;
pub(crate) use resolve_or_abort;