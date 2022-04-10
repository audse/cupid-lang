use crate::Token;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Error {
    pub line: usize,
    pub index: usize,
    pub message: String,
    pub source: String,
}

impl Error {
    
    pub fn from_token(token: &Token, message: &str) -> Error {
        Error {
            line: token.line,
            index: token.index,
            source: token.source.clone(),
            message: String::from(message),
        }
    }

    pub fn make_string(&mut self) -> String {
        return format!(
            "Error at line {} / {} (\"{}\"): {}", 
            self.line, 
            self.index, 
            self.source, 
            self.message
        );
    }

    // TODO fn get_line_string(line, file) -> String { }

}