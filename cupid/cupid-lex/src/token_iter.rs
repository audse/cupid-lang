use cupid_util::BiDirectionalIterator;
use super::token::{Token, TokenKind};
use IsOptional::*;

pub const BRACKETS: [&str; 2] = ["[", "]"];
pub const BRACES: [&str; 2] = ["{", "}"];
pub const PARENS: [&str; 2] = ["(", ")"];
pub const QUOTES: [&str; 2] = ["'", "'"];
pub const DOUBLE_QUOTES: [&str; 2] = ["\"", "\""];
pub const OPEN_DELIM: [&str; 5] = ["[", "{", "(", "'", "\""];
pub const CLOSE_DELIM: [&str; 5] = ["]", "}", ")", "'", "\""];

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum IsOptional {
    Optional,
    #[default]
    Required,
}

#[derive(Debug, Default)]
pub struct TokenIterator(pub BiDirectionalIterator<Token<'static>>);

#[derive(Debug)]
pub struct TokenListBuilder<'tokens> {
    pub tokens: &'tokens mut TokenIterator,
    list: Vec<Token<'static>>,
}

pub trait ConditionFn = Fn(&Token<'static>) -> bool + Copy;

impl<'tokens> TokenListBuilder<'tokens> {
    pub fn start(tokens: &'tokens mut TokenIterator) -> Self {
        Self { tokens, list: vec![] }
    }

    pub fn done(self) -> Vec<Token<'static>> { 
        self.list
    }

    pub fn and(self, mut closure: impl FnMut(Self) -> Option<Self>) -> Option<Self> {
        closure(self)
    }

    fn next(self, next_fn: impl Fn(Self) -> (Option<Token<'static>>, Self), optional: IsOptional) -> Option<Self> {
        match next_fn(self) {
            (Some(token), mut next) => {
                next.list.push(token);
                Some(next)
            },
            (None, next) => if optional == Required {
                None
            } else {
                Some(next)
            }
        }
    }

    pub fn assign(mut self, var: &mut Option<Token<'static>>) -> Self {
        *var = self.list.pop();
        self
    }

    pub fn one_of<S: Into<Vec<&'static str>>>(mut self, string: S, optional: IsOptional) -> Option<Self> {
        let mut fulfilled = false;
        for item in string.into() {
            if (&self.tokens).0.peek(0)?.source == item { fulfilled = true; }
            self = self.string(item, Optional)?;
        }
        if fulfilled || optional == Optional { Some(self) } else { None }
    }

    pub fn string(self, string: &'static str, optional: IsOptional) -> Option<Self> {
        self.next(|builder| (builder.tokens.string(string), builder), optional)
    }

    pub fn kind(self, kind: TokenKind, optional: IsOptional) -> Option<Self> {
        self.next(|builder| (builder.tokens.kind(kind), builder), optional)
    }

    pub fn any(self, optional: IsOptional) -> Option<Self> {
        self.next(|builder| (builder.tokens.any(), builder), optional)
    }

    pub fn any_but(self, string: &'static str, optional: IsOptional) -> Option<Self> {
        self.next(|builder| (builder.tokens.any_but(string), builder), optional)
    }

    pub fn repeat(mut self, condition: impl Fn(&Token<'static>) -> bool, mut inner: impl FnMut(Self) -> Option<Self>) -> Option<Self> {
        loop {
            let next = self.tokens.0.peek(0);
            if let Some(next) = next { 
                if !condition(next) { break }
            } else {
                break
            }
            self = inner(self)?;
        }
        Some(self)
    }

    pub fn repeat_collect<T>(self, condition: impl Fn(&Token<'static>) -> bool, mut inner: impl FnMut(Self) -> Option<(T, Self)>, with_result: impl FnOnce(Vec<T>), sep: Option<&'static str>) -> Option<Self> {
        let mut items: Vec<T> = vec![];
        let builder = self
            .repeat(
                condition,
                |builder| {
                    let (item, builder) = inner(builder)?;
                    items.push(item);
                    if let Some(sep) = sep {
                        builder.string(sep, Optional)
                    } else {
                        Some(builder)
                    }
                }
            )?;
        with_result(items);
        Some(builder)
    }

    pub fn list(self, delim: [&'static str; 2], mut inner: impl FnMut(Self) -> Option<Self>, sep: &'static str) -> Option<Self> {
        self
            .string(delim[0], Required)?
            .repeat(
                |token| token.source != delim[1], 
                |builder| inner(builder)?.string(sep, Optional)
            )?
            .string(delim[1], Required)
    }

    pub fn list_collect<T>(self, delim: [&'static str; 2], inner: impl FnMut(Self) -> Option<(T, Self)>, with_result: impl FnOnce(Vec<T>), sep: Option<&'static str>) -> Option<Self> {
        self
            .string(delim[0], Required)?
            .repeat_collect(
                |token| token.source != delim[1],
                inner,
                with_result,
                sep
            )?
            .string(delim[1], Required)
    }

    pub fn paren_list(self, inner: impl FnMut(Self) -> Option<Self>, sep: &'static str) -> Option<Self> {
        self.list(PARENS, inner, sep)
    }

    pub fn paren_list_collect<T>(self, inner: impl FnMut(Self) -> Option<(T, Self)>, with_result: impl FnOnce(Vec<T>), sep: Option<&'static str>) -> Option<Self> {
        self.list_collect(PARENS, inner, with_result, sep)
    }

    pub fn bracket_list(self, inner: impl FnMut(Self) -> Option<Self>, sep: &'static str) -> Option<Self> {
        self.list(BRACKETS, inner, sep)
    }

    pub fn bracket_list_collect<T>(self, inner: impl FnMut(Self) -> Option<(T, Self)>, with_result: impl FnOnce(Vec<T>), sep: Option<&'static str>) -> Option<Self> {
        self.list_collect(BRACKETS, inner, with_result, sep)
    }

    pub fn brace_list(self, inner: impl FnMut(Self) -> Option<Self>, sep: &'static str) -> Option<Self> {
        self.list(BRACES, inner, sep)
    }

    pub fn brace_list_collect<T>(self, inner: impl FnMut(Self) -> Option<(T, Self)>, with_result: impl FnOnce(Vec<T>), sep: Option<&'static str>) -> Option<Self> {
        self.list_collect(BRACES, inner, with_result, sep)
    }
}

impl TokenIterator {

    pub fn index(&mut self) -> usize { self.0.index() }
    pub fn goto(&mut self, index: usize) { self.0.goto(index) }

    pub fn mark<T>(&mut self, closure: impl FnOnce(&mut TokenIterator) -> Option<T>) -> Option<T> {
        let start = self.index();
        if let Some(val) = closure(self) {
            Some(val)
        } else {
            self.goto(start);
            None
        }
    }

    fn advance(&mut self) -> Token<'static> {
        self.0.next().expect("Expected token")
    }

    pub fn advance_if(&mut self, closure: impl Fn(&Token<'static>) -> bool) -> Option<Token<'static>> {
        if self.0.peek(0).map(closure)? { 
            Some(self.advance()) 
        } else { 
            None 
        }
    }

    pub fn string(&mut self, string: &str) -> Option<Token<'static>> {
        self.advance_if(|token| token.source == string)
    }

    pub fn kind(&mut self, kind: TokenKind) -> Option<Token<'static>> {
        self.advance_if(|token| token.kind == kind)
    }

    pub fn any(&mut self) -> Option<Token<'static>> {
        self.0.next()
    }

    pub fn any_but(&mut self, string: &str) -> Option<Token<'static>> {
        self.advance_if(|token| token.source != string)
    }
}
