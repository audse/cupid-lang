use std::borrow::Cow;
use crate::span::{Position, Span};

#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Token<'token> {
    pub span: Span,
    pub source: Cow<'token, str>,
    pub document: usize,
    pub kind: TokenKind,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

pub trait ReduceTokens {
    fn reduce_tokens(self) -> Option<Token<'static>>;
}

impl ReduceTokens for Vec<Token<'static>> {
    fn reduce_tokens(self) -> Option<Token<'static>> {
        self.into_iter().reduce(|mut prev, curr| {
            if curr.span.start < prev.span.start {
                prev.span.start = curr.span.start;
            }
            if curr.span.end > prev.span.end {
                prev.span.end = curr.span.end;
            }
            prev.source += curr.source;
            prev
        })
    }
}