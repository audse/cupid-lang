use std::{vec::IntoIter, iter::Peekable};
use crate::{token::{Token, TokenKind}, span::Position};

#[derive(Debug, Default, Clone)]
pub struct Lexer {
    pub prev_pos: Position,
    pub current_pos: Position,
}

type Chars<'c> = &'c mut Peekable<IntoIter<char>>;

impl Lexer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_token(&mut self, tokens: &mut Vec<Token>, source: String, kind: TokenKind) {
        self.current_pos.character += source.len();
        self.prev_pos = self.current_pos;
        tokens.push(Token::new(self.prev_pos, self.current_pos, source, kind));
    }
    pub fn lex<S: Into<String>>(&mut self, source: S) -> Vec<Token<'static>> {
        let mut tokens = vec![];
        let mut chars = source
            .into()
            .chars()
            .collect::<Vec<char>>()
            .into_iter()
            .peekable();
        
        while let Some(c) = chars.peek() {
            match c {
                'A'..='Z' | 'a'..='z' | '_' | '!' => {
                    let ident = lex_ident(&mut chars);
                    self.add_token(&mut tokens, ident, TokenKind::Ident);
                },
                '0'..='9' => {
                    let (number, kind) = lex_number(&mut chars);
                    self.add_token(&mut tokens, number, kind);
                }
                '"' | '\'' => {
                    let string = lex_string(&mut chars);
                    self.add_token(&mut tokens, string, TokenKind::String);
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
                    chars.next();
                },
                ' ' | '\r' | '\t' => { 
                    self.current_pos.character += 1; 
                    chars.next();
                },
                _ => { self.add_token(&mut tokens, (&mut chars).next_string(), TokenKind::Symbol); }
            }
        }
        tokens
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