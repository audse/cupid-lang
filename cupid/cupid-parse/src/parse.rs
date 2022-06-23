use std::borrow::Cow;

use cupid_lex::lexer::{Token, TokenKind};
use cupid_util::{BiDirectionalIterator, Bx};

use IsOptional::*;

use cupid_passes::{
    env::environment::Mut,
    pre_analysis::{Decl, Expr, TypeDef},
    util::{
        static_nodes::{value::Value, ident::Ident, field::Field}, 
        attributes::Attributes
    },
};

pub struct TokenList(BiDirectionalIterator<Token<'static>>);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum IsOptional {
    Optional,
    #[default]
    Required,
}

impl TokenList {

    fn index(&mut self) -> usize { self.0.index() }
    fn goto(&mut self, index: usize) { self.0.goto(index) }

    fn mark<T>(&mut self, mut closure: impl FnMut(&mut TokenList) -> Option<T>) -> Option<T> {
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

    pub fn next_is_not(&mut self, source: &str) -> bool {
        match self.0.peek(0) {
            Some(token) if token.source != source => true,
            _ => false
        }
    }

    pub fn expect(&mut self, source: &str) -> Option<Token<'static>> {
        match self.0.peek(0) {
            Some(token) if token.source == source => Some(self.advance()),
            _ => None
        }
    }

    pub fn expect_kind(&mut self, kind: TokenKind) -> Option<Token<'static>> {
        match self.0.peek(0) {
            Some(token) if token.kind == kind => Some(self.advance()),
            _ => None
        }
    }

    pub fn expect_any(&mut self) -> Option<Token<'static>> {
        self.0.next()
    }

    pub fn expect_sequence(
        &mut self, 
        sequence: Vec<(IsOptional, &dyn Fn(&mut TokenList) -> Option<Token<'static>>)>
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

#[derive(Default)]
pub struct Parser {
    pub source: Vec<Vec<Token<'static>>>,
    pub document: usize,
}

fn parse<T: std::str::FromStr>(string: &str) -> T {
    string.parse::<T>().unwrap_or_else(|_| panic!("Problem parsing `{string}`"))
}

impl Parser {

    pub fn new() -> Self { Self::default() }

    pub fn parse(&mut self, tokens: Vec<Token<'static>>) -> Option<Vec<Expr>> {
        let mut exprs = vec![];
        let mut tokens = TokenList(BiDirectionalIterator::new(tokens));
        while tokens.0.peek(0).is_some() {
            exprs.push(Expr::parse(self, &mut tokens)?)
        }
        Some(exprs)
    }

    fn parse_value<T: std::str::FromStr>(&mut self, token: Token<'static>) -> (T, Attributes) {
        (parse(&token.source.to_string()), self.attr([token]))
    }

    fn attr<T: Into<Vec<Token<'static>>>>(&mut self, tokens: T) -> Attributes {
        self.source.push(tokens.into());
        Attributes {
            scope: 0,
            address: self.source.len() - 1
        }
    }
}

#[allow(unused_variables)]
pub trait Parse where Self: Sized + 'static {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> { None }
}

impl Parse for Expr {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        TypeDef::parse(parser, tokens).map(|type_def| Expr::TypeDef(type_def))
            .or(Decl::parse(parser, tokens).map(|decl| Expr::Decl(decl)))
            .or(Value::parse(parser, tokens).map(|val| Expr::Value(val)))
            .or(Ident::parse(parser, tokens).map(|ident| Expr::Ident(ident)))
    }
}

impl Parse for Decl {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        tokens.mark(|tokens| {
            let mut decl_tokens = tokens.expect_sequence(vec![
                (Required, &|t| t.expect("let")),
                (Optional, &|t| t.expect("mut")),
                (Required, &|t| t.expect_kind(TokenKind::Ident)),
            ])?;
            let (ident_token, mutable) = if decl_tokens.len() == 3 {
                (decl_tokens.remove(2), Mut::Mutable)
            } else {
                (decl_tokens.remove(1), Mut::Immutable)
            };
            let type_annotation = TypeAnnotation::parse(parser, tokens).map(|t| t.0);
            decl_tokens.push(tokens.expect("=")?);
            Expr::parse(parser, tokens).map(|expr| {
                Decl::build()
                    .mutable(mutable)
                    .type_annotation(type_annotation)
                    .ident(Ident { 
                        name: ident_token.source.to_owned(), 
                        attr: parser.attr([ident_token]), 
                        ..Default::default() 
                    })
                    .value(expr.bx())
                    .attr(parser.attr(decl_tokens))
                    .build()
            })
        })
    }
}

fn bracket_list<T>(brackets: [&str; 2], parser: &mut Parser, tokens: &mut TokenList, mut closure: impl FnMut(&mut Parser, &mut TokenList) -> Option<T>) -> Option<(Vec<T>, Vec<Token<'static>>)> {
    tokens.mark(|tokens| {
        let mut token_list = vec![tokens.expect(brackets[0])?];
        let mut items = vec![];
        while tokens.next_is_not(brackets[1]) {
            items.push(closure(parser, tokens)?);
            if let Some(comma_token) = tokens.expect(",") {
                token_list.push(comma_token);
            }
        }
        token_list.push(tokens.expect(brackets[1])?);
        Some((items, token_list))
    })
}

impl Parse for TypeDef {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        tokens.mark(|tokens| {
            let mut def_tokens = vec![
                tokens.expect("type")
                    .or(tokens.expect("sum"))?
            ];
            let type_ident = TypeIdent::parse(parser, tokens)?.0;
            def_tokens.push(tokens.expect("=")?);

            let (fields, mut field_tokens) = bracket_list(["[", "]"], parser, tokens, |parser, tokens| {
                let field_ident = TypeIdent::parse(parser, tokens)?.0;
                let field_type = TypeAnnotation::parse(parser, tokens).map(|t| t.0);
                Some(Field(field_ident, field_type))
            })?;

            def_tokens.append(&mut field_tokens);

            Some(TypeDef::build()
                .ident(type_ident)
                .fields(fields)
                .attr(parser.attr(def_tokens))
                .build())
        })
    }
}

/// E.g. `let x : int = 1`
pub struct TypeAnnotation(pub Ident);
impl Parse for TypeAnnotation {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        tokens.mark(|tokens| {
            tokens.expect(":")?;
            Some(TypeAnnotation(TypeIdent::parse(parser, tokens)?.0))
        })
    }
}

/// E.g. `array (int)`
pub struct TypeIdent(pub Ident);
impl Parse for TypeIdent {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        tokens.mark(|tokens| {
            let mut ident = Ident::parse(parser, tokens)?;
            if let Some(generics) = Generics::parse(parser, tokens) {
                ident.generics = generics.0;
            }
            Some(TypeIdent(ident))
        })
    }
}

/// E.g. `(int, array (int))`
pub struct Generics(pub Vec<Ident>);
impl Parse for Generics {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        tokens.mark(|tokens| {
            tokens.expect("(")?;
            let mut idents = vec![Ident::parse(parser, tokens)?];
            while tokens.expect(",").is_some() {
                idents.push(TypeIdent::parse(parser, tokens)?.0);
            }
            tokens.expect(")")?;
            Some(Self(idents))
        })
    }
}


impl Parse for Ident {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        tokens.expect_kind(TokenKind::Ident).map(|token| {
            Ident::build()
                .name(token.source.to_owned())
                .attr(parser.attr([token]))
                .build()
        })
    }
}

impl Parse for Value {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        VString::parse(parser, tokens)
            .map(|(string, attr)| Value::VString(string, attr))
            .or(VInt::parse(parser, tokens).map(|(int, attr)| Value::VInteger(int, attr)))
            .or(VDec::parse(parser, tokens).map(|(whole, fraction, attr)| Value::VDecimal(whole, fraction, attr)))
            .or(VBool::parse(parser, tokens).map(|(boolean, attr)| Value::VBoolean(boolean, attr)))
            .or(tokens.expect("none").map(|token| Value::VNone(parser.attr([token]))))
    }
}

type VString = (Cow<'static, str>, Attributes);
impl Parse for VString {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        tokens.expect_kind(TokenKind::String).map(|token| {
            let (string, attr) = parser.parse_value::<String>(token);
            return (Cow::Owned(string), attr);
        })
    }
}

type VInt = (i32, Attributes);
impl Parse for VInt {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        tokens.expect_kind(TokenKind::Number).map(|token| parser.parse_value(token))
    }
}

type VDec = (i32, u32, Attributes);
impl Parse for VDec {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        tokens.expect_kind(TokenKind::Decimal).map(|token| {
            let decimal: Vec<&str>  = token.source.split('.').collect();
            (parse(decimal[0]), parse(decimal[1]), parser.attr([token]))
        })
    }
}

type VBool = (bool, Attributes);
impl Parse for VBool {
    fn parse(parser: &mut Parser, tokens: &mut TokenList) -> Option<Self> {
        tokens.expect("true").or(tokens.expect("false")).map(|token| {
            parser.parse_value(token)
        })
    }
}