use std::borrow::Cow;

use cupid_lex::{token::{Token, TokenKind}, token_iter::{IsOptional::*, TokenIterator, TokenListBuilder}};
use cupid_util::{BiDirectionalIterator, Bx, FilterSome};

use cupid_ast::{attr::Attr, expr::Expr, expr::block::Block, expr::{function::Function, value::Val}, expr::ident::Ident, expr::value::Value, stmt::{Stmt, decl::Mut}, stmt::decl::Decl, stmt::trait_def::TraitDef, stmt::type_def::TypeDef, types::traits::Trait, types::typ::{Type, BaseType}};
use cupid_env::{environment::Env, database::{source_table::query::Query, table::QueryTable}};

#[derive(Default)]
pub struct Parser {
    pub source: Vec<Vec<Token<'static>>>,
    pub env: Env,
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

    fn parse_value<T: std::str::FromStr>(&mut self, token: Token<'static>) -> (T, Attr) {
        (parse(&token.source.to_string()), self.attr([token]))
    }

    fn attr<T: Into<Vec<Token<'static>>>>(&mut self, tokens: T) -> Attr {
        Attr {
            source: self.env.database.source_table.insert(Query::insert().write(tokens.into())),
            scope: 0,
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
        TraitDef::parse(parser, tokens).map(|trait_def| Expr::Stmt(Stmt::TraitDef(trait_def)))
            .or_else(|| TypeDef::parse(parser, tokens).map(|type_def| Expr::Stmt(Stmt::TypeDef(type_def))))
            .or_else(|| Decl::parse(parser, tokens).map(|decl| Expr::Stmt(Stmt::Decl(decl))))
            .or_else(|| Function::parse(parser, tokens).map(|function| Expr::Function(function)))
            .or_else(|| Block::parse(parser, tokens).map(|block| Expr::Block(block)))
            .or_else(|| Value::parse(parser, tokens).map(|val| Expr::Value(val)))
            .or_else(|| Ident::parse(parser, tokens).map(|ident| Expr::Ident(ident)))
    }
}

/// Just an ident/type pair as a decl
/// E.g. `x : int` or `square : fun (int)
#[derive(Debug, Default)]
struct TypedIdentDecl(Decl);
impl Parse for TypedIdentDecl {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.mark(|tokens| {
            let ident = Ident::parse(parser, tokens)?;
            let type_annotation = TypeAnnotation::parse(parser, tokens).map(|t| t.0);
            Some(TypedIdentDecl(Decl::build()
                .ident(ident)
                .type_annotation(type_annotation)
                .attr(parser.attr([]))
                .build()))
        })
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
                        .filter_some()
                        .collect::<Vec<Token>>()
                ))
                .build())
        })
    }
}

impl Parse for Function {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.mark(|tokens| {
            let mut params: Vec<Decl> = vec![];
            // TODO return type annotation
            TokenListBuilder::start(tokens)
                .repeat_collect(
                    |token| token.source != "=" && token.source != "{",
                    |mut builder| Some((TypedIdentDecl::parse(parser, &mut builder.tokens)?.0, builder)),
                    |decls| params = decls,
                    Some(",")
                )?;
            let fun_tokens = if params.len() == 0 {
                vec![tokens.string("_")?]
            } else {
                vec![]
            };
            let block = Block::parse(parser, tokens)?;
            Some(Function::build()
                .params(params)
                .body(block)
                .attr(parser.attr(fun_tokens))
                .build())
        })
    }
}

impl Parse for TraitDef {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.mark(|tokens| {
            let (mut token_trait, mut ident, mut token_equal) = (None, Ident::default(), None);
            let mut methods = vec![];
            let bracket_tokens = TokenListBuilder::start(tokens)
                .string("trait", Required)?.assign(&mut token_trait)
                .and(|mut builder| {
                    ident = TypeIdent::parse(parser, &mut builder.tokens)?.0;
                    Some(builder)
                })?
                .string("=", Required)?.assign(&mut token_equal)
                .bracket_list_collect(
                    |mut builder| {
                        let (mut token_colon, ident) = (None, TypeIdent::parse(parser, &mut builder.tokens)?.0);
                        builder = builder.string(":", Required)?.assign(&mut token_colon);
                        Some((Decl::build()
                            .ident(ident)
                            .value(Box::new(Function::parse(parser, &mut builder.tokens)?.into()))
                            .attr(parser.attr([token_colon.unwrap()]))
                            .build(), builder))
                    },
                    |m| methods = m,
                    Some(",")
                )?
                .done();
            
            let trait_val = Trait::build()
                .ident(ident.clone())
                .methods(methods)
                .attr(parser.attr(bracket_tokens))
                .build();
            
            Some(TraitDef::build()
                .ident(ident)
                .value(trait_val)
                .attr(parser.attr([]))
                .build())
        })
    }
}

impl Parse for TypeDef {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.mark(|tokens| {
            let (mut token_type, mut ident, mut token_equal) = (None, Ident::default(), None);
            let mut fields = vec![];

            let bracket_tokens = TokenListBuilder::start(tokens)
                .one_of(["type", "sum"], Required)?.assign(&mut token_type)
                .and(|mut builder| {
                    ident = TypeIdent::parse(parser, &mut builder.tokens)?.0;
                    Some(builder)
                })?
                .string("=", Required)?.assign(&mut token_equal)
                .bracket_list(
                    |mut builder| {
                        let mut decl = TypedIdentDecl::parse(parser, &mut builder.tokens)?.0;
                        decl.value = decl.type_annotation.take().map(|i| Expr::Ident(i)).unwrap_or_default().bx();
                        fields.push(decl);
                        Some(builder)
                    }, 
                    ","
                )?
                .done();
            
            let type_def_tokens: Vec<Token> = vec![token_type.unwrap(), token_equal.unwrap()];
            let base_type = match &*type_def_tokens[0].source {
                "type" => match &*ident.name {
                    "int" | "char" | "bool" | "dec" | "string" | "none" => BaseType::Struct,
                    "array" => BaseType::Array,
                    _ => BaseType::Variable
                },
                "sum" => BaseType::Sum,
                _ => unreachable!("expected either `type` or `sum` to define a type")
            };
            
            let typ = Type::build()
                .ident(ident.clone())
                .fields(fields)
                .base(base_type)
                .attr(parser.attr(bracket_tokens))
                .build();

            Some(TypeDef::build()
                .ident(ident)
                .value(typ)
                .attr(parser.attr(type_def_tokens))
                .build())
        })
    }
}

impl Parse for Block {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        BraceBlock::parse(parser, tokens)
            .map(|block| block.0)
            .or_else(|| ArrowBlock::parse(parser, tokens).map(|block| block.0))
    }
}

/// E.g. `{ ... }`
pub struct BraceBlock(pub Block);
impl Parse for BraceBlock {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.mark(|tokens| {
            let mut exprs: Vec<Expr> = vec![];
            let block_tokens = TokenListBuilder::start(tokens)
                .brace_list_collect(
                    |mut builder| Some((Expr::parse(parser, &mut builder.tokens)?, builder)),
                    |e| exprs = e,
                    None
                )?
                .done();
            Some(BraceBlock(Block::build()
                .expressions(exprs)
                .attr(parser.attr(block_tokens))
                .build()))
        })
    }
}

/// E.g. `=> ...`
#[derive(Debug, Default)]
struct ArrowBlock(Block);
impl Parse for ArrowBlock {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.mark(|tokens| {
            let block_tokens = TokenListBuilder::start(tokens)
                .string("=", Required)?
                .string(">", Required)?
                .done();
            let expr = Expr::parse(parser, tokens)?;
            Some(ArrowBlock(Block::build()
                .expressions(vec![expr])
                .attr(parser.attr(block_tokens))
                .build()))
        })
    }
}

/// E.g. `let x : int = 1`
#[derive(Debug, Default)]
struct TypeAnnotation(Ident);
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
struct TypeIdent(Ident);
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
struct Generics(Vec<Ident>);
impl Parse for Generics {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.mark(|tokens| {
            let mut generics = vec![];
            TokenListBuilder::start(tokens)
                .paren_list(
                    |builder: TokenListBuilder| {
                        generics.push(TypeIdent::parse(parser, builder.tokens)?.0);
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
            .map(|(string, attr)| (Val::VString(string), attr))
            .or_else(|| VInt::parse(parser, tokens).map(|(int, attr)| (Val::VInteger(int), attr)))
            .or_else(|| VDec::parse(parser, tokens).map(|(whole, fraction, attr)| (Val::VDecimal(whole, fraction), attr)))
            .or_else(|| VBool::parse(parser, tokens).map(|(boolean, attr)| (Val::VBoolean(boolean), attr)))
            .or_else(|| tokens.string("none").map(|token| (Val::VNone, parser.attr([token]))))
            .map(|(inner, attr)| Value { inner, attr })
    }
}

type VString = (Cow<'static, str>, Attr);
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

type VInt = (i32, Attr);
impl Parse for VInt {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.kind(TokenKind::Number).map(|token| parser.parse_value(token))
    }
}

type VDec = (i32, u32, Attr);
impl Parse for VDec {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.kind(TokenKind::Decimal).map(|token| {
            let decimal: Vec<&str>  = token.source.split('.').collect();
            (parse(decimal[0]), parse(decimal[1]), parser.attr([token]))
        })
    }
}

type VBool = (bool, Attr);
impl Parse for VBool {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Self> {
        tokens.string("true").or(tokens.string("false")).map(|token| {
            parser.parse_value(token)
        })
    }
}