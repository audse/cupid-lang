use std::borrow::Cow;

use cupid_lex::{token::{Token, TokenKind}, token_iter::{IsOptional::*, TokenIterator, TokenListBuilder}};
use cupid_util::{BiDirectionalIterator, Bx};

use cupid_passes::{
    env::environment::Mut,
    pre_analysis::{Decl, Expr, TypeDef},
    util::{
        static_nodes::{value::Value, ident::Ident, field::Field}, 
        attributes::Attributes
    },
};

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
        let mut tokens = TokenIterator(BiDirectionalIterator::new(tokens));
        while !tokens.0.at_end() {
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
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> { None }
}

#[allow(unused_variables)]
pub trait ParseInto<T>: Parse {
    fn parse_into(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<T> { None }
}

impl Parse for Expr {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        TypeDef::parse(parser, tokens).map(|type_def| Expr::TypeDef(type_def))
            .or_else(|| Decl::parse(parser, tokens).map(|decl| Expr::Decl(decl)))
            .or_else(|| Value::parse(parser, tokens).map(|val| Expr::Value(val)))
            .or_else(|| Ident::parse(parser, tokens).map(|ident| Expr::Ident(ident)))
    }
}

impl Parse for Decl {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.mark(|tokens| {
            let (mut token_let, mut token_mut, mut token_ident, mut type_annotation, mut token_equal, mut expr) = (None, None, None, None, None, Expr::default());

            TokenListBuilder::start(tokens)
                .string("let", Required)?.assign(&mut token_let)
                .string("mut", Optional)?.assign(&mut token_mut)
                .kind(TokenKind::Ident, Required)?.assign(&mut token_ident)
                .and(|mut builder| {
                    type_annotation = TypeAnnotation::parse(parser, &mut builder.tokens).map(|t| t.0);
                    Some(builder)
                })?
                .string("=", Required)?.assign(&mut token_equal)
                .and(|mut builder| {
                    expr = Expr::parse(parser, &mut builder.tokens)?;
                    Some(builder)
                })?;

            Some(Decl::build()
                .mutable(if token_mut.is_some() { Mut::Mutable } else { Mut::Immutable })
                .ident(Ident { 
                    name: token_ident.as_ref().unwrap().source.to_owned(), 
                    attr: parser.attr([token_ident.unwrap()]), 
                    ..Default::default() 
                })
                .type_annotation(type_annotation)
                .value(expr.bx())
                .attr(parser.attr(
                    [token_let, token_mut, token_equal]
                        .into_iter()
                        .filter_map(|token| token)
                        .collect::<Vec<Token>>()
                ))
                .build())
        })
    }
}

impl Parse for TypeDef {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.mark(|tokens| {
            let (mut token_type, mut ident, mut token_equal) = (None, TypeIdent(Ident::default()), None);
            let mut fields = vec![];

            let mut bracket_tokens = TokenListBuilder::start(tokens)
                .one_of(["type", "sum"], Required)?.assign(&mut token_type)
                .and(|mut builder| {
                    ident = TypeIdent::parse(parser, &mut builder.tokens)?;
                    Some(builder)
                })?
                .string("=", Required)?.assign(&mut token_equal)
                .bracket_list(
                    |mut builder| {
                        let field_ident = TypeIdent::parse_into(parser, &mut builder.tokens)?;
                        let field_type = TypeAnnotation::parse_into(parser, &mut builder.tokens);
                        fields.push(Field(field_ident, field_type));
                        Some(builder)
                    }, 
                    ","
                )?
                .done();
                
            
            let mut type_def_tokens: Vec<Token> = vec![token_type.unwrap(), token_equal.unwrap()];
            type_def_tokens.append(&mut bracket_tokens);
            
            Some(TypeDef::build()
                .ident(ident.0)
                .fields(fields)
                .attr(parser.attr(type_def_tokens))
                .build())
        })
    }
}

/// E.g. `let x : int = 1`
#[derive(Debug, Default)]
pub struct TypeAnnotation(pub Ident);
impl Parse for TypeAnnotation {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.mark(|tokens| {
            let mut ident = TypeIdent::default();
            TokenListBuilder::start(tokens)
                .string(":", Required)?
                .and(|mut builder| {
                    ident = TypeIdent::parse(parser, &mut builder.tokens)?;
                    Some(builder)
                })?;
            Some(TypeAnnotation(ident.0))
        })
    }
}
impl ParseInto<Ident> for TypeAnnotation {
    fn parse_into(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Ident> {
        Some(Self::parse(parser, tokens)?.0)
    }
}

/// E.g. `array (int)`
#[derive(Debug, Default)]
pub struct TypeIdent(pub Ident);
impl Parse for TypeIdent {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.mark(|tokens| {
            let mut ident = Ident::parse(parser, tokens)?;
            if let Some(generics) = Generics::parse(parser, tokens) {
                ident.generics = generics.0;
            }
            Some(TypeIdent(ident))
        })
    }
}
impl ParseInto<Ident> for TypeIdent {
    fn parse_into(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Ident> {
        Some(Self::parse(parser, tokens)?.0)
    }
}

/// E.g. `(int, array (int))`
#[derive(Debug, Default)]
pub struct Generics(pub Vec<Ident>);
impl Parse for Generics {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.mark(|tokens| {
            let mut generics = vec![];
            TokenListBuilder::start(tokens)
                .paren_list(
                    |builder: TokenListBuilder| {
                        generics.push(Ident::parse(parser, builder.tokens)?);
                        Some(builder)
                    }, 
                    ","
                )?;
            Some(Self(generics))
        })
    }
}

impl ParseInto<Vec<Ident>> for Generics {
    fn parse_into(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Vec<Ident>> {
        Some(Self::parse(parser, tokens)?.0)
    }
}

impl Parse for Ident {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.kind(TokenKind::Ident).map(|token| {
                Ident::build()
                    .name(token.source.to_owned())
                    .attr(parser.attr([token]))
                    .build()
            })
    }
}

impl Parse for Value {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        VString::parse(parser, tokens)
            .map(|(string, attr)| Value::VString(string, attr))
            .or_else(|| VInt::parse(parser, tokens).map(|(int, attr)| Value::VInteger(int, attr)))
            .or_else(|| VDec::parse(parser, tokens).map(|(whole, fraction, attr)| Value::VDecimal(whole, fraction, attr)))
            .or_else(|| VBool::parse(parser, tokens).map(|(boolean, attr)| Value::VBoolean(boolean, attr)))
            .or_else(|| tokens.string("none").map(|token| Value::VNone(parser.attr([token]))))
    }
}

type VString = (Cow<'static, str>, Attributes);
impl Parse for VString {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.kind(TokenKind::String).map(|token| {
            let (mut string, attr) = parser.parse_value::<String>(token);
            string.pop(); // remove quotes
            string.remove(0);
            return (Cow::Owned(string), attr);
        })
    }
}

type VInt = (i32, Attributes);
impl Parse for VInt {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.kind(TokenKind::Number).map(|token| parser.parse_value(token))
    }
}

type VDec = (i32, u32, Attributes);
impl Parse for VDec {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.kind(TokenKind::Decimal).map(|token| {
            let decimal: Vec<&str>  = token.source.split('.').collect();
            (parse(decimal[0]), parse(decimal[1]), parser.attr([token]))
        })
    }
}

type VBool = (bool, Attributes);
impl Parse for VBool {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.string("true").or(tokens.string("false")).map(|token| {
            parser.parse_value(token)
        })
    }
}