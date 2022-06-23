use std::borrow::Cow;

use cupid_lex::{token::{Token, TokenKind}, token_iter::{IsOptional::*, TokenIterator, TokenListBuilder, EndTokenBuilder}};
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
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> { None }
}

impl Parse for Expr {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        TypeDef::parse(parser, tokens).map(|type_def| Expr::TypeDef(type_def))
            .or(Decl::parse(parser, tokens).map(|decl| Expr::Decl(decl)))
            .or(Value::parse(parser, tokens).map(|val| Expr::Value(val)))
            .or(Ident::parse(parser, tokens).map(|ident| Expr::Ident(ident)))
    }
}

impl Parse for Decl {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        let (mut token_let, mut token_mut, mut token_ident, mut type_annotation, mut token_equal, mut expr);

        TokenListBuilder::new(tokens)
            .source("let", Required)?.assign(&mut token_let)
            .source("mut", Optional)?.assign_option(&mut token_mut)
            .kind(TokenKind::Ident, Required)?.assign(&mut token_ident)
            .then_assign(&mut type_annotation, TypeAnnotation::parse(parser, tokens).map(|t| t.0))
            .source("=", Required)?.assign(&mut token_equal)
            .then_assign(&mut expr, Expr::parse(parser, tokens)?);

        Some(Decl::build()
            .mutable(if token_mut.is_some() { Mut::Mutable } else { Mut::Immutable })
            .ident(Ident { 
                name: token_ident.source.to_owned(), 
                attr: parser.attr([token_ident]), 
                ..Default::default() 
            })
            .type_annotation(type_annotation)
            .value(expr.bx())
            .attr(parser.attr(
                [Some(token_let), token_mut, Some(token_equal)]
                    .into_iter()
                    .filter_map(|token| token)
                    .collect::<Vec<Token>>()
            ))
            .build())
    }
}

// fn bracket_list<T>(brackets: [&str; 2], parser: &mut Parser, tokens: &mut TokenIterator, mut closure: impl FnMut(&mut Parser, &mut TokenIterator) -> Option<T>) -> Option<(Vec<T>, Vec<Token<'static>>)> {
//     TokenListBuilder::new(tokens)
//         .bracket_list(|builder| items.push(closure(parser, )), Some(","))?

//     tokens.mark(|tokens| {
//         let mut token_list = vec![tokens.expect(brackets[0])?];
//         let mut items = vec![];
//         while tokens.next_is_not(brackets[1]) {
//             items.push(closure(parser, tokens)?);
//             if let Some(comma_token) = tokens.expect(",") {
//                 token_list.push(comma_token);
//             }
//         }
//         token_list.push(tokens.expect(brackets[1])?);
//         Some((items, token_list))
//     })
// }


impl Parse for TypeDef {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        let (mut token_type, mut ident, mut token_equal);
        let mut fields = vec![];

        let mut bracket_tokens = TokenListBuilder::start(tokens, |builder: TokenListBuilder| {
                Some(builder
                    .one_of(["type", "sum"], Required)?.assign(&mut token_type)
                    .then_assign(&mut ident, TypeIdent::parse(parser, tokens)?)
                    .source("=", Required)?.assign(&mut token_equal)
                    .bracket_list(|mut builder: TokenListBuilder| {
                        let (mut field_ident, mut field_type);
                        builder
                            .then_assign(&mut field_ident, TypeIdent::parse(parser, tokens)?)
                            .then_assign(&mut field_type, TypeAnnotation::parse(parser, tokens).map(|t| t.0))
                            .then(|| Some(fields.push(Field(field_ident.0, field_type))), Required)
                    }, Some(","))?
                    .done())
            })
            .end(tokens)?;
            
        
        let mut type_def_tokens: Vec<Token> = vec![token_type, token_equal];
        type_def_tokens.append(&mut bracket_tokens);
        
        Some(TypeDef::build()
            .ident(ident.0)
            .fields(fields)
            .attr(parser.attr(type_def_tokens))
            .build())
    }
}

/// E.g. `let x : int = 1`
pub struct TypeAnnotation(pub Ident);
impl Parse for TypeAnnotation {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.mark(|tokens| {
            tokens.expect(":")?;
            Some(TypeAnnotation(TypeIdent::parse(parser, tokens)?.0))
        })
    }
}

/// E.g. `array (int)`
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

/// E.g. `(int, array (int))`
pub struct Generics(pub Vec<Ident>);
impl Parse for Generics {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
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
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.expect_kind(TokenKind::Ident).map(|token| {
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
            .or(VInt::parse(parser, tokens).map(|(int, attr)| Value::VInteger(int, attr)))
            .or(VDec::parse(parser, tokens).map(|(whole, fraction, attr)| Value::VDecimal(whole, fraction, attr)))
            .or(VBool::parse(parser, tokens).map(|(boolean, attr)| Value::VBoolean(boolean, attr)))
            .or(tokens.expect("none").map(|token| Value::VNone(parser.attr([token]))))
    }
}

type VString = (Cow<'static, str>, Attributes);
impl Parse for VString {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.expect_kind(TokenKind::String).map(|token| {
            let (string, attr) = parser.parse_value::<String>(token);
            return (Cow::Owned(string), attr);
        })
    }
}

type VInt = (i32, Attributes);
impl Parse for VInt {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.expect_kind(TokenKind::Number).map(|token| parser.parse_value(token))
    }
}

type VDec = (i32, u32, Attributes);
impl Parse for VDec {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.expect_kind(TokenKind::Decimal).map(|token| {
            let decimal: Vec<&str>  = token.source.split('.').collect();
            (parse(decimal[0]), parse(decimal[1]), parser.attr([token]))
        })
    }
}

type VBool = (bool, Attributes);
impl Parse for VBool {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.expect("true").or(tokens.expect("false")).map(|token| {
            parser.parse_value(token)
        })
    }
}