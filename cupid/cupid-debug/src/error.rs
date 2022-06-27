use std::{fmt, rc::Rc, backtrace::Backtrace};
use thiserror::Error;
use colored::*;
use cupid_lex::{token::Token, span::Position};

#[derive(Debug, Copy, Clone, Default)]
pub enum Severity {
    Warning,
    #[default]
    Error,
}

#[derive(Debug, Default, Clone, Error)]
pub enum ErrorCode {
    // Warnings
    #[error("unused variable")]
    UnusedVariable = 304,

    // Errors
    #[error("not found")]
    NotFound = 404,


    #[default]
    #[error("something went wrong")]
    SomethingWentWrong = 500,
}

#[allow(unused_variables)]
pub trait FmtReport {
    fn fmt_report(&self, severity: Severity) -> String { String::new() }
    fn fmt_report_source(&self, severity: Severity, source: String) -> String { String::new() }
}

impl FmtReport for Token<'_> {
    fn fmt_report(&self, _severity: Severity) -> String {
        // TODO
        format!("`{}`", self.source)
    }
}

#[derive(Debug, Error)]
pub struct Error {
    pub message: String,
    pub source: Source,
    pub context: ExprError,
    pub code: ErrorCode,
    pub hints: Vec<String>,
    pub backtrace: Backtrace,
}

#[derive(Debug)]
pub struct Source(Rc<String>);
impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Source {}

impl Default for Error {
    fn default() -> Self {
        Self {
            message: String::new(),
            context: ExprError::Empty,
            source: Source(Rc::new(String::from("could not find source"))),
            backtrace: Backtrace::capture(),
            code: ErrorCode::SomethingWentWrong,
            hints: vec![]
        }
        
    }
}

#[derive(Debug, Error)]
pub struct Warning(Error);

impl FmtReport for ErrorCode {
    fn fmt_report(&self, severity: Severity) -> String {
        let string = self.to_string();
        let title = match severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        let code = format!("[{}]", self.clone() as usize).dimmed();
        let string = match severity {
            Severity::Warning => format!("{title}: {}", string).yellow(),
            Severity::Error => format!("{title}: {}", string).red(),
        };
        format!("{code}\n{}", string.bold())
    }
}

fn message(m: &str) -> String {
    textwrap::wrap(m, 50)
        .into_iter()
        .map(|line| format!("   {}", line))
        .collect::<Vec<String>>()
        .join("\n")
}

fn line(c: &str, len: usize) -> String {
    (0..=len)
        .collect::<Vec<usize>>()
        .iter()
        .map(|_| c)
        .collect::<Vec<&str>>()
        .join("")
}

fn underline(len: usize) -> String {
    line("^", len)
}

impl FmtReport for Error {
    fn fmt_report(&self, severity: Severity) -> String {
        let title = format!("{}", self.code.fmt_report(severity));
        let message = message(&self.message);
        let source = self.context.fmt_report_source(severity, self.source.0.to_string());
        format!("\n{title}\n{message}\n{source}\n")
    }
}

impl FmtReport for Warning {
    fn fmt_report(&self, severity: Severity) -> String {
        self.0.fmt_report(severity)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fmt_report(Severity::Error))
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fmt_report(Severity::Warning))
    }
}

#[derive(Debug, Default, Clone)]
pub enum ExprError {
    Decl {
        token_let: Token<'static>,
        token_eq: Token<'static>,
        ident: Box<ExprError>,
        value: Box<ExprError>,
    },
    Expr(Box<ExprError>),
    Ident(Token<'static>),
    TraitDef {
        token_trait: Token<'static>,
        token_eq: Token<'static>,
        value: Box<ExprError>,
    },
    TypeDef {
        token_type: Token<'static>,
        token_eq: Token<'static>,
        value: Box<ExprError>,
    },
    Value(Vec<Token<'static>>),
    #[default]
    Empty
}

macro_rules! lines {
    ($string:expr) => {
        $string.lines().collect::<Vec<&str>>()
    };
}

fn line_capture(token: &Token<'static>, line_len: usize) -> (usize, usize) {
    let (start, end) = (token.span.start.line, token.span.end.line);
    (
        if start > 0 { start - 1 } else { start },
        if end > line_len { line_len + 1 } else { line_len },
    )
}

impl FmtReport for ExprError {
    fn fmt_report_source(&self, severity: Severity, source: String) -> String { 
        use ExprError::*;
        
        match self {
            Expr(expr) => expr.fmt_report_source(severity, source),
            Ident(token) => {
                let lines = lines!(source);
                
                let highlight = HighlightedLine {
                    lines: LineSet {
                        line: lines[token.span.start.line].to_string(),
                        ..Default::default()
                    },
                    line_num: token.span.start.line,
                    range: (token.span.start.character, token.span.end.character),
                    severity
                };
                highlight.bold_range().underline_range().line_set().lines.to_string()
            },
            _ => String::new()
        }
    }
}

/// # Examples
/// ```
/// LineSet {
///     overline:  String::from("    |                "),
///     line:      String::from(" 12 | let my_var = 1 "),
///     underline: String::from("    |     ^^^^^^     "),
/// }
/// ```
#[derive(Default)]
struct LineSet {
    overline: String,
    line: String,
    underline: String,
}

impl fmt::Display for LineSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}", self.overline, self.line, self.underline)
    }
}

#[derive(Default)]
struct HighlightedLine {
    lines: LineSet,
    line_num: usize,
    range: (usize, usize),
    severity: Severity,
}

impl HighlightedLine {
    fn bold_range(mut self) -> Self {
        self.lines.line = self.lines.line
            .chars()
            .enumerate()
            .map(|(i, c)| if i >= self.range.0 && i <= self.range.1 {
                format!("{}", c.to_string().bold())
            } else {
                c.to_string()
            })
            .collect();
        self
    }
    fn underline_range(mut self) -> Self {
        let underline = underline(self.range.1 - self.range.0); // e.g. "^^^^"
        let before = line(" ", self.range.0 - 1); // e.g. "  "
        let underline = format!("{before}{underline}"); // e.g. "  ^^^^"
        let underline = match self.severity {
            Severity::Error => underline.red(),
            Severity::Warning => underline.yellow(),
        }.bold().to_string();
        self.lines.underline = underline;
        self
    }
    fn line_set(mut self) -> Self {
        let line_num_str = self.line_num.to_string();
        let indent_len = line_num_str.len();
        let indent_str = line(" ", indent_len - 1);
        let dim_indent = |s: &str| format!(" {s} | ").dimmed();
        self.lines = LineSet { 
            overline: dim_indent(&indent_str).to_string(),
            line: format!("{}{}", dim_indent(&line_num_str), self.lines.line),
            underline: format!("{}{}", dim_indent(&indent_str), self.lines.underline)
        };
        self
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
        context: ExprError::Ident(Token::new(
            Position { line: 5, character: 4 },
            Position { line: 5, character: 5 },
            "me".to_string(),
            cupid_lex::token::TokenKind::Ident
        )),
        ..Default::default()
    };
    eprintln!("{msg}")
}