use std::borrow::Cow;
use crate::span::{Position, Span};

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
    pub fn new(start: Position, end: Position, source: String, kind: TokenKind) -> Self {
        Self { span: Span { start, end}, source: Cow::Owned(source), document: 0, kind }
    }
}