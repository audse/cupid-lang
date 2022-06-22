use std::{borrow::Cow, vec::IntoIter, iter::Peekable};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Position {
	pub line: usize,
	pub character: usize,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Span {
	pub start: Position,
	pub end: Position,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Token<'token> {
    pub span: Span,
    pub source: Cow<'token, str>,
    pub document: usize,
    pub kind: TokenKind,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Ident,
    Number,
    Decimal,
    String,
    #[default]
    Symbol,
}

impl Token<'_> {
    fn new(start: Position, end: Position, source: String, kind: TokenKind) -> Self {
        Self { span: Span { start, end}, source: Cow::Owned(source), document: 0, kind }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Lexer<'lex> {
    pub source: String,
    pub tokens: Vec<Token<'lex>>,
    pub prev_pos: Position,
    pub current_pos: Position,
}

type Chars<'c> = &'c mut Peekable<IntoIter<char>>;

impl Lexer<'_> {
    pub fn new(source: String) -> Self {
        Self { source, ..Default::default() }
    }
    pub fn add_token(&mut self, source: String, kind: TokenKind) {
        self.current_pos.character += source.len();
        self.tokens.push(Token::new(self.prev_pos, self.current_pos, source, kind));
        self.prev_pos = self.current_pos;
    }
    pub fn lex(&mut self) {
        let mut chars = self.source
            .chars()
            .collect::<Vec<char>>()
            .into_iter()
            .peekable();
        
        while let Some(c) = chars.peek() {
            match c {
                'A'..='Z' | 'a'..='z' | '_' | '!' => {
                    let ident = lex_ident(&mut chars);
                    self.add_token(ident, TokenKind::Ident);
                },
                '0'..='9' => {
                    let (number, kind) = lex_number(&mut chars);
                    self.add_token(number, kind);
                }
                '"' | '\'' => {
                    let string = lex_string(&mut chars);
                    self.add_token(string, TokenKind::String);
                }
                '#' => {
                    while let Some(c) = chars.next() {
                        match c {
                            '\n' => {
                                self.current_pos.line += 1;
                                self.current_pos.character = 0;
                                break
                            },
                            _ => { self.current_pos.character += 1; }
                        }
                    }
                }
                '\n' => {
                    self.current_pos.line += 1;
                    self.current_pos.character = 0;
                },
                ' ' | '\r' | '\t' => { self.current_pos.character += 1; },
                _ => { self.add_token((&mut chars).next_string(), TokenKind::Symbol); }
            }
        }
    }
}

trait NextCharString {
    fn next_string(&mut self) -> String;
}

impl NextCharString for Chars<'_> {
    fn next_string(&mut self) -> String {
        self.next().unwrap_or_default().to_string()
    }
}

fn is_ident(c: char) -> bool {
    matches!(c, 'A'..='Z' | 'a'..='z' | '_' | '!')
}

fn lex_ident(mut chars: Chars) -> String {
    let mut ident = String::new();
    while let Some(c) = chars.peek() && is_ident(*c) {
        ident += &chars.next_string(); 
        break;
    }
    ident
}

fn is_number(c: char) -> bool {
    matches!(c, '0'..='9' | '_' | '.')
}

fn lex_number(mut chars: Chars) -> (String, TokenKind) {
    let mut number = String::new();
    let mut kind = TokenKind::Number;
    while let Some(c) = chars.peek() && is_number(*c) {
        if *c == '.' {
            kind = TokenKind::Decimal;
        }
        number += &chars.next_string();
    }
    (number, kind)
}

fn lex_string(mut chars: Chars) -> String {
    let open_quote = *chars.peek().unwrap();
    let mut string = chars.next_string();
    while let Some(c) = chars.peek() {
        if *c == open_quote {
            string += &chars.next_string();
            break;
        } else {
            string += &chars.next_string();
        }
    }
    string
}