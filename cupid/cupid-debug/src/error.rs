use std::{fmt, rc::Rc, backtrace::Backtrace};
use cupid_util::{wrap_indent, bullet_list, fmt_if_nonempty};
use thiserror::Error;
use colored::*;
use crate::{code::*, source::*};

#[cfg(test)]
use cupid_lex::{token::Token, span::Position};

#[derive(Debug, Copy, Clone, Default)]
pub enum Severity {
    Warning,
    #[default]
    Error,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
        }
    }
}

#[derive(Debug, Error)]
pub struct Error {
    pub message: String,
    pub source: Source,
    pub context: ExprSource,
    pub code: ErrorCode,
    pub hints: Hints,
    pub backtrace: Backtrace,
}

#[derive(Debug, Default, Clone)]
pub struct Hints(Vec<String>);

impl fmt::Display for Hints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", fmt_if_nonempty!(&self.0 => |_| format!(
            "\n\n{}\n{}", 
            "[help]".green().bold(), 
            bullet_list(&self.0, "*").join("\n")
        )))
    }
}

impl Default for Error {
    fn default() -> Self {
        Self {
            message: String::new(),
            context: ExprSource::Empty,
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

#[allow(unused)]
const TEST: &'static str = "
type person = [
    name : string,
    age : int
]
let me : person = [
    name : Audrey,
    age : 23
]
";

#[test]
fn test() {
    let source = TEST.to_string();
    let msg = Error {
        message: "An error has occurred! Something has happened that we did not expect. We are not really sure what to do now, to be honest.".to_string(),
        source: Source(Rc::new(source)),
        context: ExprSource::Ident(IdentSource {
            token_name: Token::new(
                Position { line: 5, character: 4 },
                Position { line: 5, character: 5 },
                "me".to_string(),
                cupid_lex::token::TokenKind::Ident
            ),
            ..Default::default()
        }),
        hints: Hints(vec![
            "Have you tried turning the program off and back on again? I hear that usually works.".to_string(),
            "I got nothing else. You're on your own.".to_string()
        ]),
        ..Default::default()
    };
    eprintln!("{msg}")
}