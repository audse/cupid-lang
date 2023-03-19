use std::fmt::{self, Debug, Display, Formatter};

use cupid_fmt::color;

use crate::token::StaticToken;

#[derive(Debug, Copy, Clone)]
pub enum CupidErr {
    CompileError,
    RuntimeError,
}

#[derive(Debug)]
pub enum Kind {
    Parse,
    Type,
    Name,
    Syntax,
    Runtime,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Type => write!(f, "type"),
            Self::Parse => write!(f, "parse"),
            Self::Name => write!(f, "name"),
            Self::Syntax => write!(f, "syntax"),
            Self::Runtime => write!(f, "runtime"),
        }
    }
}

#[derive(Debug)]
pub enum Severity {
    Error,
    Warning,
    Lint,
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Lint => write!(f, "hint"),
        }
    }
}

pub trait Reportable: Display + Debug {}

impl<T: Display + Debug> Reportable for T {}

#[derive(Debug)]
pub struct CupidError {
    pub kind: Kind,
    pub severity: Severity,
    pub message: String,
    pub data: Vec<Box<dyn Reportable>>,
}

impl Display for CupidError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            color(format!("{} {}: ", self.kind.to_string(), self.severity.to_string()))
                .bold()
                .red()
                .ok()
        )?;
        writeln!(f, "{}", color(&self.message).red().ok())?;
        for item in &self.data {
            writeln!(f, "{item}")?;
        }
        Ok(())
    }
}

impl CupidError {
    pub fn parse_error(msg: impl ToString, token: Option<StaticToken>) -> Self {
        let data: Vec<Box<dyn Reportable>> = match token {
            Some(token) => vec![Box::new(token)],
            None => vec![],
        };
        Self {
            kind: Kind::Parse,
            severity: Severity::Error,
            message: msg.to_string(),
            data,
        }
    }

    pub fn name_error(msg: impl ToString, token: impl Reportable + 'static) -> Self {
        Self {
            kind: Kind::Parse,
            severity: Severity::Error,
            message: msg.to_string(),
            data: vec![Box::new(token)],
        }
    }

    pub fn type_error(msg: impl ToString, data: impl Reportable + 'static) -> Self {
        Self {
            kind: Kind::Type,
            severity: Severity::Error,
            message: msg.to_string(),
            data: vec![Box::new(data)],
        }
    }
}
