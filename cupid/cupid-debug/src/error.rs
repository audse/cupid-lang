use std::{fmt, rc::Rc, backtrace::Backtrace};
use cupid_util::wrap_indent;
use thiserror::Error;
use crate::{code::*, source::*, hints::Hints};

#[derive(Debug, Error)]
pub struct Error {
    pub message: String,
    pub source: Source,
    pub context: Rc<ExprSource>,
    pub code: ErrorCode,
    pub hints: Hints,
    pub backtrace: Backtrace,
}

impl Error {
    pub fn new(context: Rc<ExprSource>, source: Rc<String>, code: ErrorCode) -> Self {
        Self {
            context,
            source: Source(source),
            message: code.message().to_string(),
            code,
            ..Default::default()
        }
    }
    pub fn with_message(mut self, message: impl ToString) -> Self {
        self.message = message.to_string();
        self
    }
    pub fn with_hint(mut self, hint: String) -> Self {
        self.hints.0.push(hint);
        self
    }
}

impl Default for Error {
    fn default() -> Self {
        Self {
            message: String::new(),
            context: Rc::new(ExprSource::Empty),
            source: Source(Rc::new(String::from("could not find source"))),
            backtrace: Backtrace::capture(),
            code: ErrorCode::SomethingWentWrong,
            hints: Hints(vec![])
        }
    }
}

struct ErrorString<'err>(&'err Error, String);
impl<'err> ErrorString<'err> {
    fn build(error: &'err Error) -> String {
        Self(error, String::new()).title().message().context().hints().finish()
    }
    fn title(mut self) -> Self {
        self.1 += &format!("\n{}\n", self.0.code);
        self
    }
    fn message(mut self) -> Self {
        self.1 += &(wrap_indent(&self.0.message, 50, 2) + "\n\n");
        self
    }
    fn context(mut self) -> Self {
        self.1 += &self.0.context.stringify(self.0.code.severity(), &self.0.source.0);
        self
    }
    fn hints(mut self) -> Self {
        self.1 += &format!("{}\n", self.0.hints);
        self
    }
    fn finish(self) -> String { self.1 }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ErrorString::build(self))
    }
}