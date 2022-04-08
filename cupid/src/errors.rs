use crate::Token;

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

    // pub fn report(&mut self) -> () {
    //     println!("Error at line {}: {}", self.line, self.message);
    // }

    pub fn to_string(&mut self) -> String {
        return String::from(format!(
            "Error at line {} / {} (\"{}\"): {}", 
            self.line, 
            self.index, 
            self.source, 
            self.message
        ));
    }

    // TODO fn get_line_string(line, file) -> String { }

}