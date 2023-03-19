use crate::span::{Position, Span};

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
    Colon,

    // Literals.
    Identifier,
    String,
    Float,
    Int,

    // Keywords.
    And,
    Break,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Impl,
    In,
    Nil,
    Or,
    Log,
    Loop,
    Return,
    Super,
    This,
    True,
    Let,
    While,
    Role,

    Error,
    Eof,
}

pub static INFIX_OPS: &[TokenType] = &[
    TokenType::And,
    TokenType::Or,
    TokenType::Equal,
    TokenType::EqualEqual,
    TokenType::Greater,
    TokenType::GreaterEqual,
    TokenType::Less,
    TokenType::LessEqual,
    TokenType::BangEqual,
    TokenType::Plus,
    TokenType::Minus,
    TokenType::Star,
    TokenType::Slash,
    TokenType::Dot,
];

pub static PREFIX_OPS: &[TokenType] = &[TokenType::Minus, TokenType::Bang];
pub static POSTFIX_OPS: &[TokenType] = &[TokenType::LeftParen];

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Token<'src> {
    pub kind: TokenType,
    pub lexeme: &'src str,
    pub span: Span,
}

impl<'src> Token<'src> {
    pub fn ident(text: &'src str) -> Token<'src> {
        Token {
            kind: TokenType::Identifier,
            lexeme: text,
            span: Span {
                start: Position::synthetic(),
                end: Position::synthetic(),
            },
        }
    }

    pub fn synthetic(text: &'src str) -> Token<'src> {
        Token {
            kind: TokenType::Error,
            lexeme: text,
            span: Span {
                start: Position::synthetic(),
                end: Position::synthetic(),
            },
        }
    }

    pub fn to_static(&self) -> StaticToken {
        StaticToken {
            lexeme: self.lexeme.to_string(),
            kind: self.kind,
            span: self.span,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StaticToken {
    pub kind: TokenType,
    pub lexeme: String,
    pub span: Span,
}

impl std::fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token '{}' ({:?}, {}:{})",
            self.lexeme, self.kind, self.span.start.line, self.span.start.col
        )
    }
}

impl std::fmt::Display for StaticToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}
