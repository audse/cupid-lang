use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    Assign,
    Hashtag,
    Minus,
    Plus,
    Slash,
    Asterisk,
    Bang,

    // Comparison
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    StringLiteral,
    NumberLiteral,

    Eof,

    Identifier,

    Keyword(Keyword)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Keyword {
    // Flow
    If,
    Else,

    // Constants
    True,
    False,
    None,

    Is,
    Not,
    And,
    Or
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub source_string: String,
    pub line: i32,
}


impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: i32) -> Token {
        Token {
            token_type: token_type,
            source_string: lexeme,
            line: line,
        }
    }

    pub fn new_copy(token: &Token) -> Self {
        Self {
            token_type: token.token_type,
            source_string: token.source_string.clone(),
            line: token.line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
    }
}