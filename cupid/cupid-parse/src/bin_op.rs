use std::rc::Rc;

use cupid_ast::expr::{function::Function, Expr, ident::Ident, function_call::FunctionCall};
use cupid_debug::source::{ExprSource, FunctionCallSource, IdentSource};
use cupid_lex::{token_iter::{TokenIterator, TokenListBuilder, IsOptional::*}, token::{Token, ReduceTokens}};

use crate::parse::{Parser, Parse};

fn ident_from_logic_op(parser: &mut Parser, tokens: Vec<Token<'static>>) -> (Ident, Rc<ExprSource>) {
    let token = tokens.reduce_tokens().unwrap();
    let mut i: Ident = Ident { name: token.source.clone(), ..Default::default() };
    let src = IdentSource::build().token_name(token).build();
    let (attr, source) = parser.attr(src);
    i.attr = attr;
    (i, source)
}

fn parse_bin_op(parser: &mut Parser, tokens: &mut TokenIterator) -> Option<(FunctionCall, Rc<ExprSource>)> {
    tokens.mark(|tokens| {
        let (left, left_src) = Expr::parse(parser, tokens)?;
        let (op_ident, op_src) = parse_logic_bin_op(parser, tokens)?; // TODO `or_else`
        let (right, right_src) = Expr::parse(parser, tokens)?;
        let source = FunctionCallSource::build()
            .function(op_src)
            .args(vec![left_src, right_src])
            .build();

        let (attr, source) = parser.attr(source);
        let function_call = FunctionCall::build()
            .function(op_ident)
            .args(vec![left, right])
            .attr(attr)
            .build();
        Some((function_call, source))
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