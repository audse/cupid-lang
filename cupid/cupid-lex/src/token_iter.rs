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
pub struct TokenListBuilder<'builder> {
    tokens: &'builder mut TokenIterator,
    list: Vec<Token<'static>>,
}

pub trait ConditionFn = Fn(&Token<'static>) -> bool + Copy;
pub trait TokenBuilderFn<'builder> = FnMut(TokenListBuilder<'builder>) -> Option<TokenListBuilder<'builder>>;
pub trait TokenListFn<'builder> = Fn(TokenListBuilder<'builder>) -> (Option<Token<'static>>, TokenListBuilder<'builder>);
pub trait EndTokenBuilder<T> {
    fn end(self, tokens: &mut TokenIterator) -> Option<T>;
}

impl<T> EndTokenBuilder<T> for Result<T, usize> {
    fn end(self, tokens: &mut TokenIterator) -> Option<T> {
        match self {
            Err(e) => {
                tokens.goto(e);
                None
            },
            Ok(t) => Some(t)
        }
    }
}

impl<'builder> TokenListBuilder<'builder> {
    pub fn start<R>(tokens: &'builder mut TokenIterator, mut closure: impl FnMut(TokenListBuilder<'builder>) -> Option<R>) -> Result<R, usize> {
        let start = tokens.index();
        let this = Self { tokens, list: vec![] };
        closure(this).ok_or(start)
    }

    pub fn done(self) -> Vec<Token<'static>> { 
        self.list
    }

    fn next(self, next_fn: impl TokenListFn<'builder>, optional: IsOptional) -> Option<Self> {
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
    fn and(self, mut next_fn: impl TokenBuilderFn<'builder>) -> Option<Self> {
        next_fn(self)
    }
    pub fn assign(mut self, var: &mut Token<'static>) -> Self {
        *var = self.list.pop().unwrap();
        self
    }
    pub fn assign_option(mut self, var: &mut Option<Token<'static>>) -> Self {
        *var = self.list.pop();
        self
    }
    pub fn then(self, mut closure: impl FnMut() -> Option<()>, optional: IsOptional) -> Option<Self> {
        match closure() {
            Some(_) => Some(self),
            None => if optional == Optional {
                Some(self)
            } else {
                None
            }
        }
    }
    pub fn then_assign<T>(self, var: &mut T, val: T) -> Self {
        *var = val;
        self
    }
    pub fn one_of<S: Into<Vec<&'static str>>>(mut self, source: S, optional: IsOptional) -> Option<Self> {
        let mut fulfilled = false;
        for item in source.into() {
            if self.tokens.peek_and(|next| &*next.source == item)? { fulfilled = true; }
            self = self.source(item, Optional)?;
        }
        if fulfilled || optional == Optional { Some(self) } else { None }
    }
    pub fn source(self, source: &'static str, optional: IsOptional) -> Option<Self> {
        self.next(|builder: Self| (builder.tokens.source(source), builder), optional)
    }
    pub fn kind(self, kind: TokenKind, optional: IsOptional) -> Option<Self> {
        self.next(|builder: Self| (builder.tokens.kind(kind), builder), optional)
    }
    pub fn any(self, optional: IsOptional) -> Option<Self> {
        self.next(|builder: Self| (builder.tokens.any(), builder), optional)
    }
    pub fn any_but(self, source: &'static str, optional: IsOptional) -> Option<Self> {
        self.next(|builder: Self| (builder.tokens.any_but(source), builder), optional)
    }
    pub fn delim(self, delim: [&'static str; 2], inner: impl TokenBuilderFn<'builder>) -> Option<Self> {
        self.source(delim[0], Required)?.and(inner)?.source(delim[1], Required)
    }
    pub fn parens(self, inner: impl TokenBuilderFn<'builder>) -> Option<Self> {
        self.delim(PARENS, inner)
    }
    pub fn brackets(self, inner: impl TokenBuilderFn<'builder>) -> Option<Self> {
        self.delim(BRACKETS, inner)
    }
    pub fn braces(self, inner: impl TokenBuilderFn<'builder>) -> Option<Self> {
        self.delim(BRACES, inner)
    }
    pub fn delim_list(self, delim: [&'static str; 2], mut inner: impl TokenBuilderFn<'builder>, sep: Option<&'static str>) -> Option<Self> {
        self.delim(
            delim, 
            |builder: Self| builder.repeat_while(
                |next_token: &Token| &*next_token.source != delim[1],
                |inner_builder: Self| {
                    let mut next = inner(inner_builder)?;
                    if let Some(sep) = sep {
                        next = next.source(sep, Optional)?;
                    }
                    Some(next)
                })
        )
    }
    pub fn bracket_list(self, inner: impl TokenBuilderFn<'builder>, sep: Option<&'static str>) -> Option<Self> {
        self.delim_list(BRACKETS, inner, sep)
    }
    pub fn repeat_while(mut self, condition: impl ConditionFn, mut inner: impl TokenBuilderFn<'builder>) -> Option<Self> {
        loop {
            if !self.tokens.peek_and(condition)? { return Some(self) }
            self = inner(self)?;
        }
    }
}

impl TokenIterator {

    pub fn index(&mut self) -> usize { self.0.index() }
    pub fn goto(&mut self, index: usize) { self.0.goto(index) }

    pub fn mark<T>(&mut self, mut closure: impl FnMut(&mut TokenIterator) -> Option<T>) -> Option<T> {
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

    pub fn peek_and<R>(&mut self, closure: impl Fn(&Token<'static>) -> R) -> Option<R> {
        self.0.peek(0).map(closure)
    }

    pub fn not(&mut self, source: &str) -> bool {
        self.peek_and(|token| token.source != source).unwrap_or(false)
    }

    pub fn source(&mut self, source: &str) -> Option<Token<'static>> {
        self.advance_if(|token| token.source == source)
    }

    pub fn kind(&mut self, kind: TokenKind) -> Option<Token<'static>> {
        self.advance_if(|token| token.kind == kind)
    }

    pub fn any(&mut self) -> Option<Token<'static>> {
        self.0.next()
    }

    pub fn any_but(&mut self, source: &str) -> Option<Token<'static>> {
        self.advance_if(|token| token.source != source)
    }

    pub fn expect_sequence(
        &mut self, 
        sequence: Vec<(IsOptional, &dyn Fn(&mut TokenIterator) -> Option<Token<'static>>)>
    ) -> Option<Vec<Token<'static>>> {
        self.mark(|tokens| {
            let mut all_tokens = vec![];
            for (optional, fun) in &sequence {
                if let Some(token) = fun(tokens) {
                    all_tokens.push(token);
                } else if *optional == Required {
                    return None;
                }
            }
            Some(all_tokens)
        })
    }
}
