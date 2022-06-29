use std::{borrow::Cow, rc::Rc};

use cupid_lex::{
    token::{Token, TokenKind},
    token_iter::{IsOptional::*, TokenIterator, TokenListBuilder},
};

use cupid_ast::{
    attr::Attr,
    expr::block::Block,
    expr::ident::Ident,
    expr::value::Value,
    expr::Expr,
    expr::{function::Function, value::Val},
    stmt::decl::Decl,
    stmt::trait_def::TraitDef,
    stmt::type_def::TypeDef,
    stmt::{decl::Mut, Stmt},
    types::traits::Trait,
    types::typ::{BaseType, Type},
};
use cupid_debug::source::*;
use cupid_env::{
    database::{source_table::query::Query, table::QueryTable},
    environment::Env,
};
use cupid_util::{BiDirectionalIterator, Bx};

#[derive(Default)]
pub struct Parser {
    pub env: Env,
    pub document: usize,
}

impl Parser {
    pub fn new(source: Rc<String>) -> Self {
        let mut parser = Self::default();
        parser.env.source = source;
        parser
    }

    pub fn parse(&mut self, tokens: Vec<Token<'static>>) -> Option<Vec<Expr>> {
        let mut exprs = vec![];
        let mut tokens = TokenIterator(BiDirectionalIterator::new(tokens));
        while !tokens.0.at_end() {
            exprs.push(Expr::parse(self, &mut tokens)?.0)
        }
        Some(exprs)
    }

    fn parse_value<T: std::str::FromStr>(
        &mut self,
        tokens: impl Into<Vec<Token<'static>>>,
    ) -> (T, (Attr, Rc<ExprSource>)) where <T as std::str::FromStr>::Err: std::fmt::Debug {
        let tokens = tokens.into();
        (tokens[0].source.to_string().parse::<T>().unwrap(), self.attr(tokens))
    }

    fn attr<T: Into<ExprSource>>(&mut self, source: T) -> (Attr, Rc<ExprSource>) {
        let src = source.into();
        let src_ref = Rc::new(src);
        let source = self
            .env
            .database
            .source_table
            .insert(Query::insert().write(src_ref.clone()));
        (Attr { source, scope: 0 }, src_ref)
    }
}

#[allow(unused_variables)]
pub trait Parse
where
    Self: Sized + 'static,
{
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        None
    }
}

fn map_stmt_parse(args: (impl Into<Stmt>, Rc<ExprSource>)) -> (Expr, Rc<ExprSource>) {
    (Expr::Stmt(args.0.into()), args.1)
}

fn map_expr_parse(args: (impl Into<Expr>, Rc<ExprSource>)) -> (Expr, Rc<ExprSource>) {
    (args.0.into(), args.1)
}

impl Parse for Expr {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        TraitDef::parse(parser, tokens)
            .map(map_stmt_parse)
            .or_else(|| TypeDef::parse(parser, tokens).map(map_stmt_parse))
            .or_else(|| Decl::parse(parser, tokens).map(map_stmt_parse))
            .or_else(|| Function::parse(parser, tokens).map(map_expr_parse))
            .or_else(|| Block::parse(parser, tokens).map(map_expr_parse))
            .or_else(|| Value::parse(parser, tokens).map(map_expr_parse))
            .or_else(|| Ident::parse(parser, tokens).map(map_expr_parse))
    }
}

/// Just an ident/type pair as a decl
/// E.g. `x : int` or `square : fun (int)
#[derive(Debug, Default)]
struct TypedIdentDecl(Decl);
impl Parse for TypedIdentDecl {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.mark(|tokens| {
            let (ident, ident_source) = Ident::parse(parser, tokens)?;
            let ((type_annotation, type_annot_src), token_colon) =
                parse_type_annotation(parser, tokens)
                    .map(|((t, src), token_colon)| ((Some(t), Some(src)), Some(token_colon)))
                    .unwrap_or_else(|| ((None, None), None));
            
            let source = DeclSource::build()
                .ident(ident_source)
                .type_annotation(type_annot_src)
                .token_colon(token_colon)
                .build();

            let (attr, source) = parser.attr(source);
            Some((
                TypedIdentDecl(
                    Decl::build()
                        .ident(ident)
                        .type_annotation(type_annotation.map(|t| t))
                        .attr(attr)
                        .build(),
                ),
                source,
            ))
        })
    }
}

impl Parse for Decl {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.mark(|tokens| {
            let mut source = DeclSource::default();
            let (mut ident, mut type_annotation, mut expr) =
                (Ident::default(), None, Expr::default());

            TokenListBuilder::start(tokens)
                .string("let", Required)?
                .assign(&mut source.token_let)
                .string("mut", Optional)?
                .assign(&mut source.token_mut)
                .and(|mut builder| {
                    let (ident_val, ident_src) = parse_type_ident(parser, &mut builder.tokens)?;
                    ident = ident_val;
                    source.ident = ident_src;
                    Some(builder)
                })?
                .and(|mut builder| {
                    parse_type_annotation(parser, &mut builder.tokens).map(|((t, src), token)| {
                        type_annotation = Some(t);
                        source.type_annotation = Some(src);
                        source.token_colon = Some(token);
                    });
                    Some(builder)
                })?
                .string("=", Required)?
                .assign(&mut source.token_eq)
                .and(|mut builder| {
                    let (expr_val, expr_source) = Expr::parse(parser, &mut builder.tokens)?;
                    source.value = Some(expr_source);
                    expr = expr_val;
                    Some(builder)
                })?;

            let mutable = if source.token_mut.is_some() {
                Mut::Mutable
            } else {
                Mut::Immutable
            };
            let (attr, source) = parser.attr(source);

            Some((
                Decl::build()
                    .mutable(mutable)
                    .ident(ident)
                    .type_annotation(type_annotation)
                    .value(expr.bx())
                    .attr(attr)
                    .build(),
                source,
            ))
        })
    }
}

impl Parse for Function {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.mark(|tokens| {
            let mut source = FunctionSource::default();
            let mut params: Vec<Decl> = vec![];
            // TODO return type annotation
            TokenListBuilder::start(tokens).repeat_collect(
                |token| token.source != "=" && token.source != "{",
                |mut builder| Some((TypedIdentDecl::parse(parser, &mut builder.tokens)?, builder)),
                |decls| {
                    let (param_list, src_list): (Vec<TypedIdentDecl>, Vec<Rc<ExprSource>>) =
                        decls.into_iter().unzip();
                    source.params = src_list;
                    params = param_list.into_iter().map(|d| d.0).collect();
                },
                Some(","),
            )?;
            source.token_empty = if params.len() == 0 {
                Some(tokens.string("_")?)
            } else {
                None
            };
            let (block, block_src) = Block::parse(parser, tokens)?;
            source.body = block_src;
            let (attr, source) = parser.attr(source);
            Some((
                Function::build()
                    .params(params)
                    .body(block)
                    .attr(attr)
                    .build(),
                source,
            ))
        })
    }
}

impl Parse for TraitDef {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.mark(|tokens| {
            let mut source = TraitDefSource::default();
            let mut trait_source = TraitSource::default();

            let (mut token_trait, mut ident, mut token_equal) = (None, Ident::default(), None);
            let mut methods = vec![];
            let mut bracket_tokens = TokenListBuilder::start(tokens)
                .string("trait", Required)?
                .assign(&mut token_trait)
                .and(|mut builder| {
                    let (ident_val, ident_src) = parse_type_ident(parser, &mut builder.tokens)?;
                    source.ident = ident_src.clone();
                    trait_source.ident = ident_src;
                    ident = ident_val;
                    Some(builder)
                })?
                .string("=", Required)?
                .assign(&mut token_equal)
                .bracket_list_collect(
                    |mut builder| {
                        let mut decl_src = DeclSource::default();
                        let (method_ident, method_ident_src) =
                            parse_type_ident(parser, &mut builder.tokens)?;
                        decl_src.ident = method_ident_src;

                        builder = builder
                            .string(":", Required)?
                            .assign(&mut decl_src.token_colon);
                        let (fun, fun_src) = Function::parse(parser, &mut builder.tokens)?;
                        decl_src.value = Some(fun_src);
                        let (attr, decl_src) = parser.attr(decl_src);
                        Some((
                            (
                                Decl::build()
                                    .ident(method_ident)
                                    .value(Box::new(fun.into()))
                                    .attr(attr)
                                    .build(),
                                decl_src,
                            ),
                            builder,
                        ))
                    },
                    |m| {
                        let (method_list, method_srcs): (Vec<Decl>, Vec<Rc<ExprSource>>) =
                            m.into_iter().unzip();
                        trait_source.methods = method_srcs;
                        methods = method_list;
                    },
                    Some(","),
                )?
                .done();
            let (close_delim, open_delim) =
                (bracket_tokens.pop().unwrap(), bracket_tokens.pop().unwrap());
            trait_source.token_brackets = (open_delim, close_delim);

            let (trait_attr, trait_source) = parser.attr(trait_source);
            source.value = trait_source;

            let trait_val = Trait::build()
                .ident(ident.clone())
                .methods(methods)
                .attr(trait_attr)
                .build();

            let (attr, source) = parser.attr(source);

            Some((
                TraitDef::build()
                    .ident(ident)
                    .value(trait_val)
                    .attr(attr)
                    .build(),
                source,
            ))
        })
    }
}

impl Parse for TypeDef {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.mark(|tokens| {
            let mut source = TypeDefSource::default();
            let mut type_source = TypeSource::default();

            let (mut token_type, mut ident, mut token_equal) = (None, Ident::default(), None);
            let mut fields = vec![];

            let bracket_tokens = TokenListBuilder::start(tokens)
                .one_of(["type", "sum"], Required)?
                .assign(&mut token_type)
                .and(|mut builder| {
                    let (t_ident, t_ident_src) = parse_type_ident(parser, &mut builder.tokens)?;
                    source.ident = t_ident_src.clone();
                    type_source.ident = t_ident_src;
                    ident = t_ident;
                    Some(builder)
                })?
                .string("=", Required)?
                .assign(&mut token_equal)
                .bracket_list(
                    |mut builder| {
                        let (TypedIdentDecl(mut decl), decl_src) =
                            TypedIdentDecl::parse(parser, &mut builder.tokens)?;
                        decl.value = decl
                            .type_annotation
                            .take()
                            .map(|i| Expr::Ident(i))
                            .unwrap_or_default()
                            .bx();
                        fields.push(decl);
                        type_source.fields.push(decl_src);
                        Some(builder)
                    },
                    ",",
                )?
                .done();

            let type_def_tokens: Vec<Token> = vec![token_type.unwrap(), token_equal.unwrap()];
            let base_type = match &*type_def_tokens[0].source {
                "type" => match &*ident.name {
                    "int" | "char" | "bool" | "dec" | "string" | "none" => BaseType::Struct,
                    "array" => BaseType::Array,
                    _ => BaseType::Variable,
                },
                "sum" => BaseType::Sum,
                _ => unreachable!("expected either `type` or `sum` to define a type"),
            };

            let (type_attr, type_src) = parser.attr(bracket_tokens);
            source.value = type_src;

            let typ = Type::build()
                .ident(ident.clone())
                .fields(fields)
                .base(base_type)
                .attr(type_attr)
                .build();

            let (attr, source) = parser.attr(source);

            Some((
                TypeDef::build().ident(ident).value(typ).attr(attr).build(),
                source,
            ))
        })
    }
}

impl Parse for Block {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        BraceBlock::parse(parser, tokens)
            .map(|(block, src)| (block.0, src))
            .or_else(|| ArrowBlock::parse(parser, tokens).map(|(block, src)| (block.0, src)))
    }
}

/// E.g. `{ ... }`
pub struct BraceBlock(pub Block);
impl Parse for BraceBlock {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.mark(|tokens| {
            let mut exprs: Vec<(Expr, Rc<ExprSource>)> = vec![];
            let mut block_tokens = TokenListBuilder::start(tokens)
                .brace_list_collect(
                    |mut builder| Some((Expr::parse(parser, &mut builder.tokens)?, builder)),
                    |e| exprs = e,
                    None,
                )?
                .done();
            let (exprs, expr_srcs): (Vec<Expr>, Vec<Rc<ExprSource>>) = exprs.into_iter().unzip();
            let (close_delim, open_delim) =
                (block_tokens.pop().unwrap(), block_tokens.pop().unwrap());
            let source = BlockSource::build()
                .token_delimiters((open_delim, close_delim))
                .expressions(expr_srcs)
                .build();
            let (attr, source) = parser.attr(source);
            Some((
                BraceBlock(Block::build().expressions(exprs).attr(attr).build()),
                source,
            ))
        })
    }
}

/// E.g. `=> ...`
#[derive(Debug, Default)]
struct ArrowBlock(Block);
impl Parse for ArrowBlock {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.mark(|tokens| {
            let mut block_tokens = TokenListBuilder::start(tokens)
                .string("=", Required)?
                .string(">", Required)?
                .done();
            let (expr, expr_source) = Expr::parse(parser, tokens)?;
            let (close_delim, open_delim) =
                (block_tokens.pop().unwrap(), block_tokens.pop().unwrap());
            let source = BlockSource::build()
                .token_delimiters((open_delim, close_delim))
                .expressions(vec![expr_source])
                .build();
            let (attr, source) = parser.attr(source);
            Some((
                ArrowBlock(Block::build().expressions(vec![expr]).attr(attr).build()),
                source,
            ))
        })
    }
}

/// E.g. `let x : int = 1`
fn parse_type_annotation(
    parser: &mut Parser,
    tokens: &mut TokenIterator,
) -> Option<((Ident, Rc<ExprSource>), Token<'static>)> {
    tokens.mark(|tokens| {
        let mut ident = (Ident::default(), Rc::new(ExprSource::Empty));
        let mut token_colon = TokenListBuilder::start(tokens)
            .string(":", Required)?
            .and(|mut builder| {
                ident = parse_type_ident(parser, &mut builder.tokens)?;
                Some(builder)
            })?
            .done();
        Some((ident, token_colon.pop().unwrap()))
    })
}

/// E.g. `array (int)`
fn parse_type_ident(
    parser: &mut Parser,
    tokens: &mut TokenIterator,
) -> Option<(Ident, Rc<ExprSource>)> {
    tokens.mark(|tokens| {
        let (mut ident, mut src) = tokens.kind(TokenKind::Ident).map(|token| {
            let name = token.source.to_owned();
            let src = IdentSource::build().token_name(token).build();
            (Ident::build().name(name), src)
        })?;
        if let Some((generics, generics_src)) = parse_generics(parser, tokens) {
            ident.generics = generics;
            src.generics = generics_src;
        }
        let (attr, src) = parser.attr(src);
        Some((ident.attr(attr).build(), src))
    })
}

/// E.g. `(int, array (int))`
fn parse_generics(
    parser: &mut Parser,
    tokens: &mut TokenIterator,
) -> Option<(Vec<Ident>, Vec<Rc<ExprSource>>)> {
    tokens.mark(|tokens| {
        let mut src_list = vec![];
        let mut generics = vec![];
        TokenListBuilder::start(tokens).paren_list(
            |builder: TokenListBuilder| {
                let (generic, src) = parse_type_ident(parser, builder.tokens)?;
                generics.push(generic);
                src_list.push(src);
                Some(builder)
            },
            ",",
        )?;
        Some((generics, src_list))
    })
}

impl Parse for Ident {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.kind(TokenKind::Ident).map(|token| {
            let name = token.source.to_owned();
            let (attr, src) = parser.attr(IdentSource::build().token_name(token).build());
            (Ident::build().name(name).attr(attr).build(), src)
        })
    }
}

fn map_value_parse(args: ((impl Into<Val>, Attr), Rc<ExprSource>)) -> (Value, Rc<ExprSource>) {
    let ((val, attr), src) = args;
    (
        Value {
            inner: val.into(),
            attr,
        },
        src,
    )
}

impl Parse for Value {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        VString::parse(parser, tokens)
            .map(map_value_parse)
            .or_else(|| VInt::parse(parser, tokens).map(map_value_parse))
            .or_else(|| VDec::parse(parser, tokens).map(map_value_parse))
            .or_else(|| VBool::parse(parser, tokens).map(map_value_parse))
            .or_else(|| {
                tokens.string("none").map(|token| {
                    let (attr, src) = parser.attr(vec![token]);
                    map_value_parse(((Val::VNone, attr), src))
                })
            })
    }
}

type VString = (Cow<'static, str>, Attr);
impl Parse for VString {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.kind(TokenKind::String).map(|token| {
            let (mut string, (attr, src)) = parser.parse_value::<String>(vec![token]);
            string.pop(); // remove quotes
            string.remove(0);
            ((Cow::Owned(string), attr), src)
        })
    }
}

type VInt = (i32, Attr);
impl Parse for VInt {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.kind(TokenKind::Number).map(|token| {
            let (val, (attr, src)) = parser.parse_value(vec![token]);
            ((val, attr), src)
        })
    }
}

type VDec = ((i32, u32), Attr);
impl Parse for VDec {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.kind(TokenKind::Decimal).map(|token| {
            let token_src = token.source.to_owned();
            let decimal: Vec<&str> = token_src.split('.').collect();
            let (attr, src) = parser.attr(vec![token]);
            (((decimal[0].parse::<i32>().unwrap(), decimal[1].parse::<u32>().unwrap()), attr), src)
        })
    }
}

type VBool = (bool, Attr);
impl Parse for VBool {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens
            .string("true")
            .or(tokens.string("false"))
            .map(|token| {
                let (val, (attr, src)) = parser.parse_value(vec![token]);
                ((val, attr), src)
            })
    }
}
