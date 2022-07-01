use std::fmt;

#[derive(Debug, Copy, Clone, Default, serde::Serialize, serde::Deserialize)]
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