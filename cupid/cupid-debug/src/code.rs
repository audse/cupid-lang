use std::fmt;
use thiserror::Error;
use colored::*;
use crate::error::Severity;

/// Error codes are where the bulk of the error reporting happens.
/// Each error is associated with a code that corresponds to a specific error title and message.
/// 
/// # Guidelines
/// ## Codes
/// Codes roughly follow HTTP error codes.
/// * `100` codes are for syntax errors
/// * `300` codes are for warnings/lints
/// * `400` codes are for semantic errors
/// * `500` codes are for system/compiler errors
/// 
/// ## Messages
/// * Should use "we" instead of "I" or "you" in most cases
/// * Should use friendly and natural language
///     * "cannot" instead of "unauthorized" or "bad"
/// * Prefer to phrase messages as instructions over descriptions
///     * "expected a type here" instead of "did not expect an identifier"
/// * Maximum 2 sentences
/// * Full punctuation
/// * Should provide as much context as possible, customized to each error

#[derive(Debug, Default, Clone, Error)]
pub enum ErrorCode {
    /// Syntax errors
    UnclosedDelimiter = 103,

    /// Warnings
    UnusedVariable = 304,
    
    /// Semantic errors
    TypeMismatch = 400,
    CannotInfer = 401,
    OutOfScope = 403,
    NotFound = 404,
    CannotAccess = 405,
    AlreadyDefined = 406,
    CannotUnify = 409,
    ExpectedType = 417,
    UnexpectedType = 418,
    ExpectedFunction = 419,
    ExpectedTrait = 420,
    ExpectedExpression = 421,

    /// System errors
    #[default]
    SomethingWentWrong = 500,
    Unreachable = 510,
}

impl ErrorCode {
    pub fn severity(&self) -> Severity {
        use ErrorCode::*;
        match self {
            UnusedVariable => Severity::Warning,
            _ => Severity::Error,
        }
    }
    fn num(&self) -> usize { self.clone() as usize }
    fn num_str(&self) -> impl fmt::Display {
        format!("[{}] {}", self.num(), self.link()).dimmed()
    }
    fn link(&self) -> impl fmt::Display {
        format!("https://add-docs-link-here.com#{}", self.num()).italic()
    }
    fn title(&self) -> &str {
        use ErrorCode::*;
        match self {
            // Warnings
            UnusedVariable => "unused variable",

            // Syntax errors
            UnclosedDelimiter => "unclosed delimiter",

            // Semantic errors
            TypeMismatch => "type mismatch",
            CannotInfer => "cannot infer type",
            OutOfScope => "variable defined outside of this scope",
            NotFound => "not found",
            CannotAccess => "cannot access",
            AlreadyDefined => "variable has already been defined",
            CannotUnify => "cannot unify types",
            ExpectedType => "expected a type",
            UnexpectedType => "unexpected type",
            ExpectedFunction => "expected a function",
            ExpectedTrait => "expected a trait",
            ExpectedExpression => "expected an expression",

            // System errors
            SomethingWentWrong => "something went wrong",
            Unreachable => "unreachable area reached",
        }
    }
    pub fn message(&self) -> &str {
        use ErrorCode::*;
        // TODO
        match self {
            UnclosedDelimiter => "You have an opening bracket that is not matched with a closing bracket.",
            Unreachable => "This program has reached a part of the code that should be unreachable.",
            _ => "Something went wrong, but we are not sure what."
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (code, title, severity) = (self.num_str(), self.title(), self.severity());
        let string = match severity {
            Severity::Warning => format!("[{severity}] {title}").yellow(),
            Severity::Error => format!("[{severity}] {title}").red(),
        };
        write!(f, "{code}\n{}", string.bold())
    }
}