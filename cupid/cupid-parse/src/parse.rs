use std::{borrow::Cow, cell::RefCell, rc::Rc};

use cupid_lex::{
    token::{Token, TokenKind},
    token_iter::{IsOptional::*, TokenIterator, TokenListBuilder},
};

use cupid_ast::{
    attr::Attr,
    expr::{
        block::Block,
        function::Function,
        function_call::FunctionCall,
        ident::Ident,
        value::{Val, Value},
        Expr,
    },
    stmt::{
        allocate::{Allocate, Allocation, AllocationStage},
        assign::Assign,
        decl::{Decl, Mut},
        implement::Impl,
        trait_def::TraitDef,
        type_def::TypeDef,
        Stmt,
    },
    types::{
        traits::Trait,
        typ::{BaseType, Type},
    },
};
use cupid_debug::source::*;
use cupid_env::{
    database::{source_table::query::Query, table::QueryTable},
    environment::Env,
};
use cupid_util::{BiDirectionalIterator, WrapRc, WrapRefCell};

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

    pub fn new_with(source: Rc<String>, env: Env) -> Self {
        let mut parser = Self { env, document: 0 };
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
    pub fn attr<T: Into<ExprSource>>(&mut self, source: T) -> (Attr, Rc<ExprSource>) {
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

// macro_rules! rule {
//
//     // "one of"
//     (one($tokens:ident => $($rest:tt),*)) => {
//         || -> Option<Token<'static>> {
//             $(if let Some(token) = rule!($tokens => $rest) {
//                 return Some(token)
//             })*
//             None
//         }()
//     };
//
//     // tuple
//     (tuple( $tokens:ident => $($rest:tt),* )) => {
//         Some((
//             $( rule!($tokens => $rest)? ),*
//         ))
//     };
//
//     ($tokens:ident => TokenKind::$kind:ident) => {
//         $tokens.kind(TokenKind::$kind)
//     };
//     ($tokens:ident => |tokens| $custom:block) => {
//         $custom($tokens)
//     };
//     ($tokens:ident => $string:literal) => {
//         $tokens.string($string)
//     };
//     ($tokens:ident => $name:ident) => {
//         $name($tokens)
//     };
//
//     // ??
//     // ($tokens:ident => $rest:tt) => {
//     //     None
//     // };
//     () => {};
// }
//
// macro_rules! parser {
//     // Returns the first matched token
//     ($name:ident = one $rule:tt; $($rest:tt)*) => {
//         pub fn $name(tokens: &mut TokenIterator) -> Option<Token<'static>> {
//             tokens.mark(|tokens| rule!(one(tokens => $rule)))
//         }
//         parser! { $($rest)* }
//     };
//     // Returns a tuple of tokens
//     ($name:ident = tuple( $($rule:tt),* ); $($rest:tt)*) => {
//         pub fn $name(tokens: &mut TokenIterator) -> Option<($( parser!{ @accum $rule } ),*)> {
//             tokens.mark(|tokens| rule!(tuple(tokens => $($rule),*)))
//         }
//         parser! { $($rest)* }
//     };
//     // Returns a single token
//     ($name:ident = $rule:tt; $($rest:tt)*) => {
//         pub fn $name(tokens: &mut TokenIterator) -> Option<Token<'static>> {
//             tokens.mark(|tokens| rule!(tokens => $rule))
//         }
//         parser! { $($rest)* }
//     };
//     // Creates a tuple return type
//     (@accum $_:expr) => { Token<'static> };
//     () => {};
// }
//
// parser! {
//     // Keywords
//     kw_let = "let";
//     kw_mut = "mut";
//     kw_type = "type";
//     kw_trait = "trait";
//     kw_is_not = tuple("is", "not");
//     kw_is_type = tuple("is", "type");
//
//     // Expressions
//     expr = one(ident, number, decimal, string);
//
//     ident = TokenKind::Ident;
//     number = TokenKind::Number;
//     decimal = TokenKind::Decimal;
//     string = TokenKind::String;
//
//     // Statements
//     assign = tuple(ident, "=", expr);
// }

fn map_stmt_parse(args: (impl Into<Stmt>, Rc<ExprSource>)) -> (Expr, Rc<ExprSource>) {
    (Expr::Stmt(args.0.into()), args.1)
}

fn map_expr_parse(args: (impl Into<Expr>, Rc<ExprSource>)) -> (Expr, Rc<ExprSource>) {
    (args.0.into(), args.1)
}

impl Parse for Expr {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        if let Some((val, val_src)) = TraitDef::parse(parser, tokens)
            .map(map_stmt_parse)
            .or_else(|| Impl::parse(parser, tokens).map(map_stmt_parse))
            .or_else(|| TypeDef::parse(parser, tokens).map(map_stmt_parse))
            .or_else(|| Decl::parse(parser, tokens).map(map_stmt_parse))
            .or_else(|| Assign::parse(parser, tokens).map(map_stmt_parse))
            .or_else(|| Function::parse(parser, tokens).map(map_expr_parse))
            .or_else(|| Block::parse(parser, tokens).map(map_expr_parse))
            .or_else(|| FunctionCall::parse(parser, tokens).map(map_expr_parse))
            .or_else(|| Value::parse(parser, tokens).map(map_expr_parse))
            .or_else(|| Ident::parse(parser, tokens).map(map_expr_parse))
        {
            super::bin_op::parse_bin_op(parser, tokens, val, val_src)
        } else {
            None
        }
    }
}

// macro_rules! parse {
//     /// Example
//     /// ```
//     /// // Decl
//     /// parse!(parser, tokens => {
//     ///     token_let = "let" (Required);
//     ///     token_mut = "mut" (Optional);
//     ///     ident, ident_src = Ident::parse (Required);
//     ///     token_equal = "=" (Required);
//     /// });
//     /// // Type
//     /// parse!(parser, tokens => {
//     ///     token_type = "type" (Required);
//     ///     ident, ident_src = Ident::parse (Required);
//     ///     token_equal = "=" (Required);
//     ///     fields = [
//     ///         |parser, tokens| parse!(parser, tokens => {
//     ///             field, src = Field::parse (Required)
//     ///         });
//     ///     ] ("," Required);
//     /// })
//     /// ```
//     $(
//         $parser:ident, $tokens:ident => { $($rest:tt)* }
//     ) => {{
//         $tokens.mark(|tokens| {
//             parse!($parser, tokens ... $($rest)*);
//         })
//     }};
//     $($parser, $tokens, ... $field = $string:tt ($is_required:ident); $($rest:tt)*) => {{
//         let $field = $tokens.string($string, $is_required)?;
//         parse!($parser, $tokens, ... $($rest)*);
//     }};
//     $($parser:ident, $tokens:ident ... $field:ident, $src:ident = $node:ident ::parse (Required); $($rest:tt)*) => {{
//         let ($field, $src) = $node::parse($parser, $tokens)?;
//         parse!($parser, $tokens, ... $($rest)*);
//     }};
//     $($parser:ident, $tokens:ident ... $field:ident, $src:ident = $node:ident ::parse (Optional); $($rest:tt)*) => {{
//         let ($field, $src) = if let Some(($field, $src)) = $node::parse($parser, $tokens) {
//             ($field, $src)
//         } else {
//             (None, None)
//         };
//         parse!($parser, $tokens, ... $($rest)*);
//     }};
//     $($parser:ident, $tokens:ident ... $field:ident = [ $parse:expr ]; $($rest:tt)*) => {{
//         let $field: Vec<Token<'static>> = vec![];
//         TokenListBuilder::start($tokens)
//             .bracket_list_collect(
//                 |builder| {
//                     $parse($parser, $builder.tokens)
//                 },
//                 |tokens| $field = tokens;
//             )
//     }};
//     $() => {};
// }

/// Just an ident/type pair as a decl
/// E.g. `x : int` or `square : fun (int)`
fn parse_typed_ident_decl(
    parser: &mut Parser,
    tokens: &mut TokenIterator,
) -> Option<(Decl, Rc<ExprSource>)> {
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
            Decl::build()
                .allocate(Allocate::build().ident(ident).attr(attr).build())
                .type_annotation(type_annotation.map(|t| t))
                .build(),
            source,
        ))
    })
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
                    let (ident_val, ident_src) = Ident::parse(parser, &mut builder.tokens)?;
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
            let allocate = Allocate::build()
                .ident(ident)
                .value(Allocation::Expr(expr.ref_cell().rc()))
                .stage(AllocationStage::NameResolution)
                .attr(attr)
                .build();
            Some((
                Decl::build()
                    .mutable(mutable)
                    .allocate(allocate)
                    .type_annotation(type_annotation)
                    .build(),
                source,
            ))
        })
    }
}

impl Parse for Assign {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.mark(|tokens| {
            let mut source = AssignSource::default();

            let (ident, ident_src) = Ident::parse(parser, tokens)?;
            source.ident = ident_src;
            source.token_eq = tokens.string("=")?;
            let (expr, expr_src) = Expr::parse(parser, tokens)?;
            source.value = Some(expr_src);

            let (attr, source) = parser.attr(source);

            Some((
                Assign(
                    Allocate::build()
                        .ident(ident)
                        .value(Allocation::Expr(expr.ref_cell().rc()))
                        .stage(AllocationStage::Runtime)
                        .attr(attr)
                        .build(),
                ),
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
            let _builder = TokenListBuilder::start(tokens).repeat_collect(
                |token| token.source != "=" && token.source != r"{",
                |mut builder| {
                    Some((
                        parse_typed_ident_decl(parser, &mut builder.tokens)?,
                        builder,
                    ))
                },
                |decls| {
                    let (param_list, src_list): (Vec<Decl>, Vec<Rc<ExprSource>>) =
                        decls.into_iter().unzip();
                    source.params = src_list;
                    params = param_list.into_iter().collect();
                },
                Some(","),
            )?;
            source.token_empty = if params.len() == 0 {
                Some(tokens.string("_")?)
            } else {
                None
            };
            let (ret_type, ret_type_src) = parse_return_type_annotation(parser, tokens)
                .map(|(i, src)| (Some(i), Some(src)))
                .unwrap_or_else(|| (None, None));
            let (block, block_src) = Block::parse(parser, tokens)?;
            source.body = block_src;
            source.return_type_annotation = ret_type_src;
            let (attr, source) = parser.attr(source);
            Some((
                Function::build()
                    .params(params)
                    .body(block)
                    .return_type_annotation(ret_type)
                    .attr(attr)
                    .build(),
                source,
            ))
        })
    }
}

impl Parse for FunctionCall {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.mark(|tokens| {
            let mut source = FunctionCallSource::default();
            let ident = tokens.kind(TokenKind::Ident).map(|token| {
                let name = token.source.to_owned();
                let ident_source = IdentSource::build().token_name(token).build();
                let (attr, ident_source) = parser.attr(ident_source);
                source.function = ident_source;
                Ident::build().name(name).attr(attr).build()
            })?;

            let mut args = vec![];
            TokenListBuilder::start(tokens).paren_list_collect(
                |mut builder| Some((Expr::parse(parser, &mut builder.tokens)?, builder)),
                |parsed_args| {
                    let (arg_list, src_list): (Vec<Expr>, Vec<Rc<ExprSource>>) =
                        parsed_args.into_iter().unzip();
                    source.args = src_list;
                    args = arg_list.into_iter().collect();
                },
                Some(","),
            )?;
            let (attr, source) = parser.attr(source);
            Some((
                FunctionCall::build()
                    .function(ident)
                    .args(args)
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
                    let (ident_val, ident_src) = Ident::parse(parser, &mut builder.tokens)?;
                    source.ident = ident_src.clone();
                    trait_source.ident = ident_src;
                    ident = ident_val;
                    Some(builder)
                })?
                .string("=", Required)?
                .assign(&mut token_equal)
                .bracket_list_collect(
                    |builder| parse_method_decl(parser, builder),
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
                .build()
                .ref_cell()
                .rc();

            let (attr, source) = parser.attr(source);

            Some((
                TraitDef(
                    Allocate::build()
                        .ident(ident)
                        .value(Allocation::Trait(trait_val))
                        .stage(AllocationStage::TypeNameResolution)
                        .attr(attr)
                        .build(),
                ),
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
                    let (t_ident, t_ident_src) = Ident::parse(parser, &mut builder.tokens)?;
                    source.ident = t_ident_src.clone();
                    type_source.ident = t_ident_src;
                    ident = t_ident;
                    Some(builder)
                })?
                .string("=", Required)?
                .assign(&mut token_equal)
                .bracket_list(
                    |mut builder| {
                        let (mut decl, decl_src) =
                            parse_typed_ident_decl(parser, &mut builder.tokens)?;
                        decl.allocate.value = Allocation::Expr(
                            decl.type_annotation
                                .take()
                                .map(|i| Expr::Ident(i))
                                .unwrap_or_default()
                                .ref_cell()
                                .rc(),
                        );
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
                .build()
                .ref_cell()
                .rc();

            let (attr, source) = parser.attr(source);

            Some((
                TypeDef(
                    Allocate::build()
                        .ident(ident)
                        .value(Allocation::Type(typ))
                        .stage(AllocationStage::TypeNameResolution)
                        .attr(attr)
                        .build(),
                ),
                source,
            ))
        })
    }
}

fn parse_method_decl<'t>(
    parser: &mut Parser,
    mut builder: TokenListBuilder<'t>,
) -> Option<((Decl, Rc<ExprSource>), TokenListBuilder<'t>)> {
    let mut decl_src = DeclSource::default();
    let (method_ident, method_ident_src) = Ident::parse(parser, &mut builder.tokens)?;
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
                .allocate(
                    Allocate::build()
                        .ident(method_ident)
                        .value(Allocation::Expr(Rc::new(RefCell::new(fun.into()))))
                        .attr(attr)
                        .build(),
                )
                .build(),
            decl_src,
        ),
        builder,
    ))
}

impl Parse for Impl {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.mark(|tokens| {
            let mut source = ImplSource::default();
            let mut implement = Impl::default();
            let mut tokens = TokenListBuilder::start(tokens)
                .string("implement", Required)?
                .and(|mut builder| {
                    let (trait_ident, trait_src) = Ident::parse(parser, &mut builder.tokens)?;
                    implement.trait_ident = trait_ident;
                    source.trait_ident = trait_src;
                    Some(builder)
                })?
                .string("for", Required)?
                .and(|mut builder| {
                    let (type_ident, type_src) = Ident::parse(parser, &mut builder.tokens)?;
                    implement.type_ident = type_ident;
                    source.type_ident = type_src;
                    Some(builder)
                })?
                .string("=", Required)?
                .bracket_list_collect(
                    |builder| parse_method_decl(parser, builder),
                    |m| {
                        let (method_list, method_srcs): (Vec<Decl>, Vec<Rc<ExprSource>>) =
                            m.into_iter().unzip();
                        source.methods = method_srcs;
                        implement.methods = method_list;
                    },
                    Some(","),
                )?
                .done();
            source.token_impl = tokens.remove(0);
            source.token_for = tokens.remove(0);
            source.token_equal = tokens.remove(0);
            source.token_delimiters = (tokens.remove(0), tokens.remove(0));
            let (attr, src) = parser.attr(source);
            implement.attr = attr;
            Some((implement, src))
        })
    }
}

impl Parse for Block {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        parse_brace_block(parser, tokens).or_else(|| parse_arrow_block(parser, tokens))
    }
}

/// E.g. `{ ... }`
fn parse_brace_block(
    parser: &mut Parser,
    tokens: &mut TokenIterator,
) -> Option<(Block, Rc<ExprSource>)> {
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
        let (close_delim, open_delim) = (block_tokens.pop().unwrap(), block_tokens.pop().unwrap());
        let source = BlockSource::build()
            .token_delimiters((open_delim, close_delim))
            .expressions(expr_srcs)
            .build();
        let (attr, source) = parser.attr(source);
        Some((Block::build().expressions(exprs).attr(attr).build(), source))
    })
}

/// E.g. `=> ...`
fn parse_arrow_block(
    parser: &mut Parser,
    tokens: &mut TokenIterator,
) -> Option<(Block, Rc<ExprSource>)> {
    tokens.mark(|tokens| {
        let mut block_tokens = TokenListBuilder::start(tokens)
            .string("=", Required)?
            .string(">", Required)?
            .done();
        let (expr, expr_source) = Expr::parse(parser, tokens)?;
        let (close_delim, open_delim) = (block_tokens.pop().unwrap(), block_tokens.pop().unwrap());
        let source = BlockSource::build()
            .token_delimiters((open_delim, close_delim))
            .expressions(vec![expr_source])
            .build();
        let (attr, source) = parser.attr(source);
        Some((
            Block::build().expressions(vec![expr]).attr(attr).build(),
            source,
        ))
    })
}

/// E.g. `x : int`
fn parse_type_annotation(
    parser: &mut Parser,
    tokens: &mut TokenIterator,
) -> Option<((Ident, Rc<ExprSource>), Token<'static>)> {
    tokens.mark(|tokens| {
        let mut ident = (Ident::default(), Rc::new(ExprSource::Empty));
        let mut token_colon = TokenListBuilder::start(tokens)
            .string(":", Required)?
            .and(|mut builder| {
                ident = Ident::parse(parser, &mut builder.tokens)?;
                Some(builder)
            })?
            .done();
        Some((ident, token_colon.pop().unwrap()))
    })
}

/// E.g. `-> int`
fn parse_return_type_annotation(
    parser: &mut Parser,
    tokens: &mut TokenIterator,
) -> Option<(Ident, Rc<ExprSource>)> {
    tokens.mark(|tokens| {
        let mut _arrow_tokens = TokenListBuilder::start(tokens)
            .string("-", Required)?
            .string(">", Required)?
            .done();
        Ident::parse(parser, tokens)
    })
}

impl Parse for Ident {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        tokens.mark(|tokens| {
            let (mut ident, mut source) = tokens.kind(TokenKind::Ident).map(|token| {
                let name = token.source.to_owned();
                let source = IdentSource::build().token_name(token).build();
                (Ident::build().name(name), source)
            })?;
            if let Some((generics, generics_source)) = parse_generics(parser, tokens) {
                ident.generics = generics;
                source.generics = generics_source;
            }
            let (attr, source) = parser.attr(source);
            Some((ident.attr(attr).build(), source))
        })
    }
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
                let (generic, src) = Ident::parse(parser, builder.tokens)?;
                generics.push(generic);
                src_list.push(src);
                Some(builder)
            },
            ",",
        )?;
        Some((generics, src_list))
    })
}

impl Parse for Value {
    fn parse(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Self, Rc<ExprSource>)> {
        parse_value_string(parser, tokens)
            .or_else(|| parse_value_int(parser, tokens))
            .or_else(|| parse_value_dec(parser, tokens))
            .or_else(|| parse_value_bool(parser, tokens))
            .or_else(|| parse_value_none(parser, tokens))
    }
}

fn into_value(val: impl Into<Val>, attr: Attr, src: Rc<ExprSource>) -> (Value, Rc<ExprSource>) {
    (
        Value {
            inner: val.into(),
            attr,
        },
        src,
    )
}

fn parse_value<T: std::str::FromStr>(
    parser: &mut Parser,
    tokens: impl Into<Vec<Token<'static>>>,
) -> (T, (Attr, Rc<ExprSource>))
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let tokens = tokens.into();
    (
        tokens[0].source.to_string().parse::<T>().unwrap(),
        parser.attr(tokens),
    )
}

fn parse_value_string(
    parser: &mut Parser,
    tokens: &mut TokenIterator,
) -> Option<(Value, Rc<ExprSource>)> {
    tokens.kind(TokenKind::String).map(|token| {
        let (mut string, (attr, src)) = parse_value::<String>(parser, vec![token]);
        string.pop(); // remove quotes
        string.remove(0);
        into_value(Cow::Owned(string), attr, src)
    })
}

fn parse_value_int(
    parser: &mut Parser,
    tokens: &mut TokenIterator,
) -> Option<(Value, Rc<ExprSource>)> {
    tokens.kind(TokenKind::Number).map(|token| {
        let (val, (attr, src)) = parse_value::<i32>(parser, vec![token]);
        into_value(val, attr, src)
    })
}

fn parse_value_dec(
    parser: &mut Parser,
    tokens: &mut TokenIterator,
) -> Option<(Value, Rc<ExprSource>)> {
    tokens.kind(TokenKind::Decimal).map(|token| {
        let token_src = token.source.to_owned();
        let decimal: Vec<&str> = token_src.split('.').collect();
        let (attr, src) = parser.attr(vec![token]);
        into_value(
            (
                decimal[0].parse::<i32>().unwrap(),
                decimal[1].parse::<u32>().unwrap(),
            ),
            attr,
            src,
        )
    })
}

fn parse_value_bool(
    parser: &mut Parser,
    tokens: &mut TokenIterator,
) -> Option<(Value, Rc<ExprSource>)> {
    tokens
        .string("true")
        .or(tokens.string("false"))
        .map(|token| {
            let (val, (attr, src)) = parse_value::<bool>(parser, vec![token]);
            into_value(val, attr, src)
        })
}

fn parse_value_none(
    parser: &mut Parser,
    tokens: &mut TokenIterator,
) -> Option<(Value, Rc<ExprSource>)> {
    tokens.string("none").map(|token| {
        let (attr, src) = parser.attr(vec![token]);
        into_value(Val::VNone, attr, src)
    })
}
