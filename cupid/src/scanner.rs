use std::collections::HashMap;

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

    Error,
    Eof,
}

#[derive(Copy, Clone)]
pub struct Token<'src> {
    pub kind: TokenType,
    pub line: usize,
    pub index: usize,
    pub lexeme: &'src str,
}

impl<'src> Token<'src> {
    pub fn synthetic(text: &'src str) -> Token<'src> {
        Token {
            kind: TokenType::Error,
            lexeme: text,
            line: 0,
            index: 0,
        }
    }
}

pub struct Scanner<'src> {
    keywords: HashMap<&'static str, TokenType>,
    code: &'src str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'src> Scanner<'src> {
    pub fn new(code: &'src str) -> Scanner {
        let mut keywords = HashMap::with_capacity_and_hasher(16, Default::default());
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("in", TokenType::In);
        keywords.insert("none", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("log", TokenType::Log);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("self", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("let", TokenType::Let);
        keywords.insert("while", TokenType::While);

        Scanner {
            keywords,
            code,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token<'src> {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        match self.advance() {
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
        self.current == self.code.len()
    }

    fn lexeme(&self) -> &'src str {
        &self.code[self.start..self.current]
    }

    fn make_token(&self, kind: TokenType) -> Token<'src> {
        Token {
            kind,
            lexeme: self.lexeme(),
            line: self.line,
            index: self.current,
        }
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            0
        } else {
            self.code.as_bytes()[self.current]
        }
    }

    fn peek_next(&self) -> u8 {
        if self.current > self.code.len() - 2 {
            b'\0'
        } else {
            self.code.as_bytes()[self.current + 1]
        }
    }

    fn error_token(&self, message: &'static str) -> Token<'static> {
        Token {
            kind: TokenType::Error,
            lexeme: message,
            line: self.line,
            index: self.current,
        }
    }

    fn advance(&mut self) -> u8 {
        let char = self.peek();
        self.current += 1;
        char
    }

    fn matches(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                b' ' | b'\r' | b'\t' => {
                    self.advance();
                }
                b'\n' => {
                    self.line += 1;
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
                self.line += 1;
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
        while is_alpha(self.peek()) || is_digit(self.peek()) {
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
