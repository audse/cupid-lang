use std::{ops::{ControlFlow, FromResidual}, borrow::Cow};

use cupid_lex::lexer::{Token, TokenKind};
use cupid_util::{BiDirectionalIterator, Bx};

use TryAlternate::*;
use IsOptional::*;

#[allow(unused_imports)]
use cupid_passes::{
    env::environment::Mut,
    pre_analysis::{Decl, Expr},
    util::{
        static_nodes::{value::Value, ident::Ident}, 
        attributes::Attributes
    },
};

pub struct BaseIdent(Cow<'static, str>, Attributes);

pub struct TokenList(BiDirectionalIterator<Token<'static>>);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum IsOptional {
    Optional,
    #[default]
    Required,
}

impl TokenList {

    fn advance<'list>(&'list mut self) -> Token<'static> {
        self.0.next().expect("Expected token")
    }

    pub fn expect_source<'list>(&'list mut self, source: &str) -> Option<Token<'static>> {
        match self.0.peek(0) {
            Some(token) if token.source == source => Some(self.advance()),
            _ => None
        }
    }

    pub fn expect_kind<'list>(&'list mut self, kind: TokenKind) -> Option<Token<'static>> {
        match self.0.peek(0) {
            Some(token) if token.kind == kind => Some(self.advance()),
            _ => None
        }
    }

    pub fn expect_any<'list>(&'list mut self) -> Option<Token<'static>> {
        self.0.next()
    }

    pub fn expect_sequence<'list, S, F>(
        &'list mut self, 
        sequence: Vec<(IsOptional, Box<dyn FnMut(&mut Self) -> Option<Token<'static>>> )>
    ) -> Option<Vec<Token<'static>>> {
        let mut tokens = vec![];
        for (optional, mut fun) in sequence {
            if optional == Optional { 
                if let Some(token) = fun(self) { tokens.push(token) }
            } else {
                tokens.push(fun(self)?);
            }
        }
        Some(tokens)
    }
}

#[derive(Default)]
pub struct Parser {
    pub index: usize,
    pub source: Vec<Vec<Token<'static>>>,
    pub document: usize,
}

fn parse<T: std::str::FromStr>(string: &str) -> T {
    string.parse::<T>().unwrap_or_else(|_| panic!("Problem parsing `{string}`"))
}

impl Parser {
    fn parse_value<T: std::str::FromStr>(&mut self, token: Token<'static>) -> (T, Attributes) {
        (parse(&token.source.to_string()), attr([token], self))
    }
}

pub enum TryAlternate<T> {
    Break(T),
    Continue
}

impl<T> FromResidual for TryAlternate<T> {
    fn from_residual(residual: T) -> Self {
        Self::Break(residual)
    }
}

impl<T> std::ops::Try for TryAlternate<T> {
    type Output = ();
    type Residual = T;
    fn from_output(_: Self::Output) -> Self {
        Self::Continue
    }
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Break(val) => ControlFlow::Break(val),
            _ => ControlFlow::Continue(())
        }
    }
}

pub trait TryParse where Self: Sized {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> { None }
    fn try_parse(parser: &mut Parser, tokens: &mut TokenList) -> TryAlternate<Self> { 
        Self::parse(parser, tokens).into() 
    }
}

impl TryParse for () {
    fn try_parse(_: &mut Parser, __: &mut TokenList) -> TryAlternate<Self> {
        Continue
    }
}

fn attr<T: Into<Vec<Token<'static>>>>(tokens: T, parser: &mut Parser) -> Attributes {
    parser.source.push(tokens.into());
    Attributes {
        scope: 0,
        address: parser.source.len() - 1
    }
}

impl TryParse for Expr {

}

impl TryParse for Decl {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {

        let tokens = tokens.expect_sequence(vec![
            (Required, |tokens: &mut TokenList| { tokens.expect_source("let") }.bx()),
            (Optional, |tokens: &mut TokenList| { tokens.expect_source("mut") }.bx()),
        ]);
        // let mut decl_tokens = vec![];
        // let mut decl = Decl::build();

        // decl_tokens.push(tokens.expect_source("let")?);
        // if let Some(token) = tokens.expect_source("mut") {
        //     decl_tokens.push(token);
        //     decl = decl.mutable(Mut::Mutable);
        // }
        // let base_ident = BaseIdent::parse(parser, tokens)?;
        // decl = decl.ident(Ident { name: base_ident.0, attr: base_ident.1, ..Default::default() });
        // decl_tokens.push(tokens.expect_source("=")?);

        // let expr: Option<Expr> = Expr::try_parse(parser, tokens).into();
        // decl = decl.value(expr?.bx());

        // Some(decl.build())
        todo!()
    }
}

impl<T> From<Option<T>> for TryAlternate<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(t) => Break(t),
            None => Continue
        }
    }
}

impl<T> From<TryAlternate<T>> for Option<T> {
    fn from(option: TryAlternate<T>) -> Self {
        match option {
            Break(t) => Some(t),
            Continue => None
        }
    }
}

impl TryParse for BaseIdent {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        if let Some(ident_token) = tokens.expect_kind(TokenKind::Ident) {
            match &*ident_token.source {
                "let" | "mut" | "type" | "trait" | "is" | "not" /* TODO */ => (), // don't match keywords
                _ => return Some(BaseIdent(ident_token.source.to_owned(), attr([ident_token], parser)))
            }
        }
        None
    }
}

impl TryParse for Value {
    fn try_parse(parser: &mut Parser, tokens: &mut TokenList) -> TryAlternate<Self> {
        if let Some((string, attr)) = VString::parse(parser, tokens) {
            Break(Value::VString(string, attr))?;
        }
        if let Some((int, attr)) = VInt::parse(parser, tokens) {
            Break(Value::VInteger(int, attr))?;
        }
        if let Some((whole, fraction, attr)) = VDec::parse(parser, tokens) {
            Break(Value::VDecimal(whole, fraction, attr))?;
        }
        if let Some((boolean, attr)) = VBool::parse(parser, tokens) {
            Break(Value::VBoolean(boolean, attr))?;
        }
        if let Some(none_token) = tokens.expect_source("none") {
            Break(Value::VNone(attr([none_token], parser)))?
        }
        Continue
    }
}

type VString = (Cow<'static, str>, Attributes);
impl TryParse for VString {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        tokens.expect_kind(TokenKind::String).map(|token| {
            let (string, attr) = parser.parse_value::<String>(token);
            return (Cow::Owned(string), attr);
        })
    }
}

type VInt = (i32, Attributes);
impl TryParse for VInt {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        tokens.expect_kind(TokenKind::Number).map(|token| parser.parse_value(token))
    }
}

type VDec = (i32, u32, Attributes);
impl TryParse for VDec {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        tokens.expect_kind(TokenKind::Decimal).map(|token| {
            let decimal: Vec<&str>  = token.source.split('.').collect();
            (parse(decimal[0]), parse(decimal[1]), attr([token], parser))
        })
    }
}

type VBool = (bool, Attributes);
impl TryParse for VBool {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        tokens.expect_source("true").or(tokens.expect_source("false")).map(|token| {
            parser.parse_value(token)
        })
    }
}