use std::rc::Rc;

use cupid_ast::expr::{namespace::Namespace, Expr, ident::Ident, function_call::FunctionCall};
use cupid_debug::source::{ExprSource, FunctionCallSource, IdentSource, NamespaceSource};
use cupid_lex::{token_iter::{TokenIterator, TokenListBuilder, IsOptional::*}, token::{Token, ReduceTokens}};
use cupid_util::Bx;

use crate::parse::{Parser, Parse};

fn ident_from_logic_op(parser: &mut Parser, tokens: Vec<Token<'static>>) -> (Ident, Rc<ExprSource>) {
    let token = tokens.reduce_tokens().unwrap();
    let mut i: Ident = Ident { name: token.source.clone(), ..Default::default() };
    let src = IdentSource::build().token_name(token).build();
    let (attr, source) = parser.attr(src);
    i.attr = attr;
    (i, source)
}

/// Returns
/// ```no_run
/// Namespace { 
///     namespace: <left_value>, 
///     value: FunctionCall {
///         function: <operation_name>
///         args: [<right_value>]
///     }
/// }
/// ```
pub(super) fn parse_bin_op(parser: &mut Parser, tokens: &mut TokenIterator, left: Expr, left_src: Rc<ExprSource>) -> Option<(Expr, Rc<ExprSource>)> {
    tokens.mark(|tokens| {
        if let Some((op_ident, op_src)) = parse_logic_bin_op(parser, tokens){
            if let Some((right, right_src)) = Expr::parse(parser, tokens) {
                let func_src = FunctionCallSource::build()
                    .function(op_src)
                    .args(vec![right_src])
                    .build();
                let (func_attr, func_src) = parser.attr(func_src);
                let function_call = FunctionCall::build()
                    .function(op_ident)
                    .args(vec![right])
                    .attr(func_attr)
                    .build();

                let namespace_src = NamespaceSource::build()
                    .namespace(left_src)
                    .value(func_src)
                    .build();
                let (attr, source) = parser.attr(namespace_src);
                let namespace = Namespace::build()
                    .namespace(Box::new(left))
                    .value(Expr::FunctionCall(function_call).bx())
                    .attr(attr)
                    .build();

                Some((Expr::Namespace(namespace), source))
            } else {
                Some((left, left_src))
            }
        } else {
            Some((left, left_src))
        }
    })
}

fn parse_logic_bin_op(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(Ident, Rc<ExprSource>)> {
    tokens.mark(|tokens| {
        let ops = parse_logic_operator(parser, tokens)?;
        let (mut op_ident, op_src) = ident_from_logic_op(parser, ops);
        op_ident.name += "!";
        Some((op_ident, op_src))
    })
}

fn parse_logic_operator(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<Vec<Token<'static>>> {
    tokens.mark(|tokens| {
        Some(TokenListBuilder::start(tokens)
            .string("is", Required)?
            .one_of(["not", "type", "in"], Optional)?
            .done())
    })
    .or_else(|| tokens.mark(|tokens| {
        Some(TokenListBuilder::start(tokens)
            .string("not", Required)?
            .one_of(["in", "type"], Optional)?
            .done())
    }))
    .or_else(|| tokens.mark(|tokens| {
        Some(TokenListBuilder::start(tokens)
            .one_of(["and", "or"], Required)?
            .string("not", Optional)?
            .done())
    }))
}