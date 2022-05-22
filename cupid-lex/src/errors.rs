use crate::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Error {
    pub line: usize,
    pub index: usize,
    pub message: String,
    pub source: Cow<'static, str>,
    pub context: String,
	pub file: usize,
}

impl std::error::Error for Error {}

impl Error {
    
    pub fn from_token(token: &Token, message: &str, context: &str) -> Error {
        Error {
            line: token.line,
            index: token.index,
			file: token.file,
            source: token.source.to_owned(),
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
    fn fmt(&self, f: &mut Formatter) -> DisplayResult {
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

#[macro_export]
#[allow(unused_macros)]
macro_rules! pretty {
	($arg:tt) => {{
		let mut string = format!("{:#?}", $arg);
		string.remove(0);
		string.pop();
		string.replace("\"", "")
	}};
}