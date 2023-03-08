use crate::span::Position;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    NewLine,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    ThickArrow,

    // Literals.
    Identifier,
    String,
    Float,
    Int,

    // Keywords.
    And,
    In,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Log,
    Return,
    Super,
    This,
    True,
    Let,
    While,
    Role,
    Impl,

    Error,
    Eof,
}

#[derive(Copy, Clone, Debug)]
pub struct Token<'src> {
    pub kind: TokenType,
    pub lexeme: &'src str,
    pub position: Position,
}

impl<'src> Token<'src> {
    pub fn synthetic(text: &'src str) -> Token<'src> {
        Token {
            kind: TokenType::Error,
            lexeme: text,
            position: Position::synthetic(),
        }
    }
}
