use std::collections::HashMap;

use crate::{
    span::Position,
    token::{Token, TokenType},
};

macro_rules! keyword_map {
    ( $capacity:literal, { $( $key:tt : $val:expr ),* $(,)? } ) => {{
        (|| {
            let mut map = HashMap::with_capacity_and_hasher($capacity, Default::default());
            $(
                map.insert($key, $val);
            )*
            map
        })()
    }};
}

pub struct Scanner<'src> {
    keywords: HashMap<&'static str, TokenType>,
    code: &'src str,
    start: usize,
    position: Position,
}

impl<'src> Scanner<'src> {
    pub fn new(code: &'src str) -> Scanner {
        let keywords: HashMap<&str, TokenType> = keyword_map!(17, {
            "and": TokenType::And,
            "break": TokenType::Break,
            "class": TokenType::Class,
            "else": TokenType::Else,
            "false": TokenType::False,
            "for": TokenType::For,
            "fun": TokenType::Fun,
            "if": TokenType::If,
            "impl": TokenType::Impl,
            "in": TokenType::In,
            "none": TokenType::Nil,
            "or": TokenType::Or,
            "log": TokenType::Log,
            "loop": TokenType::Loop,
            "return": TokenType::Return,
            "super": TokenType::Super,
            "self": TokenType::This,
            "true": TokenType::True,
            "let": TokenType::Let,
            "while": TokenType::While,
            "trait": TokenType::Role,
        });

        Scanner {
            keywords,
            code,
            start: 0,
            position: Position::default(),
        }
    }

    pub fn peek_token(&mut self) -> Token<'src> {
        let start = self.start;
        let position = self.position;
        let token = self.scan_token();
        self.start = start;
        self.position = position;
        token
    }

    pub fn scan_token(&mut self) -> Token<'src> {
        self.skip_whitespace();
        self.start = self.position.index;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        match self.advance() {
            b'\n' => {
                self.position.increment_line();
                self.make_token(TokenType::NewLine)
            }
            b'(' => self.make_token(TokenType::LeftParen),
            b')' => self.make_token(TokenType::RightParen),
            b'{' => self.make_token(TokenType::LeftBrace),
            b'}' => self.make_token(TokenType::RightBrace),
            b'[' => self.make_token(TokenType::LeftBracket),
            b']' => self.make_token(TokenType::RightBracket),
            b';' => self.make_token(TokenType::Semicolon),
            b',' => self.make_token(TokenType::Comma),
            b'.' => self.make_token(TokenType::Dot),
            b'-' => self.make_token(TokenType::Minus),
            b'+' => self.make_token(TokenType::Plus),
            b'/' => self.make_token(TokenType::Slash),
            b'*' => self.make_token(TokenType::Star),
            b'!' if self.matches(b'=') => self.make_token(TokenType::BangEqual),
            b'!' => self.make_token(TokenType::Bang),
            b'=' if self.matches(b'=') => self.make_token(TokenType::EqualEqual),
            b'=' if self.matches(b'>') => self.make_token(TokenType::ThickArrow),
            b'=' => self.make_token(TokenType::Equal),
            b'<' if self.matches(b'=') => self.make_token(TokenType::LessEqual),
            b'<' => self.make_token(TokenType::Less),
            b'>' if self.matches(b'=') => self.make_token(TokenType::GreaterEqual),
            b'>' => self.make_token(TokenType::Greater),
            b'\'' => self.string(),
            c if is_digit(c) => self.number(),
            c if is_alpha(c) => self.identifier(),
            _ => self.error_token("Unexpected character."),
        }
    }

    fn is_at_end(&self) -> bool {
        self.position.index == self.code.len()
    }

    fn lexeme(&self) -> &'src str {
        &self.code[self.start..self.position.index]
    }

    fn make_token(&self, kind: TokenType) -> Token<'src> {
        Token {
            kind,
            lexeme: self.lexeme(),
            position: self.position,
        }
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            0
        } else {
            self.code.as_bytes()[self.position.index]
        }
    }

    fn peek_next(&self) -> u8 {
        if self.position.index > self.code.len() - 2 {
            b'\0'
        } else {
            self.code.as_bytes()[self.position.index + 1]
        }
    }

    fn error_token(&self, message: &'static str) -> Token<'static> {
        Token {
            kind: TokenType::Error,
            lexeme: message,
            position: self.position,
        }
    }

    fn advance(&mut self) -> u8 {
        let char = self.peek();
        self.position.increment();
        char
    }

    fn matches(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.position.increment();
            true
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                b' ' | b'\r' | b'\t' => {
                    self.advance();
                }
                b'-' if self.peek_next() == b'-' => {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    fn string(&mut self) -> Token<'src> {
        while self.peek() != b'\'' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.position.increment_line();
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated string.")
        } else {
            self.advance();
            self.make_token(TokenType::String)
        }
    }

    fn number(&mut self) -> Token<'src> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == b'.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
            self.make_token(TokenType::Float)
        } else {
            self.make_token(TokenType::Int)
        }
    }

    fn identifier(&mut self) -> Token<'src> {
        while is_ident(self.peek()) {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        self.keywords
            .get(self.lexeme())
            .cloned()
            .unwrap_or(TokenType::Identifier)
    }
}

fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}

fn is_alpha(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}

fn is_ident(c: u8) -> bool {
    is_alpha(c) || is_digit(c) || c == b'-'
}
