use crate::{
    arena::UseArena,
    ast::BinOp,
    error::CupidError,
    gc::Gc,
    token::{TokenType, INFIX_OPS, POSTFIX_OPS, PREFIX_OPS},
};

use super::{iter::Iter, parse_args, parse_unit, parser::Parser, Call, Expr, Header, UnOp};

pub fn parse_precedence<'src>(
    parser: &mut Parser<'src>,
    min_bp: u8,
    gc: &mut Gc,
) -> Result<Option<Expr<'src>>, CupidError> {
    let mut lhs = match PREFIX_OPS.contains(&parser.curr.kind) {
        true => {
            parser.advance();
            let ((), r_bp) = prefix_binding_power(parser.curr.kind);
            let rhs = parse_precedence(parser, r_bp, gc)?.unwrap();
            let rhs = parser.arena.insert(rhs);
            UnOp {
                header: parser.header(),
                expr: rhs,
                op: parser.curr.kind,
            }
            .into()
        }
        false => match parse_unit(parser, gc)? {
            Some(lhs) => lhs,
            None => return Ok(None),
        },
    };
    loop {
        let op = match INFIX_OPS.contains(&parser.curr.kind)
            || POSTFIX_OPS.contains(&parser.curr.kind)
        {
            true => parser.curr,
            false => break,
        };

        if let Some((l_bp, ())) = postfix_binding_power(op.kind) {
            if l_bp < min_bp {
                break;
            }
            match op.kind {
                TokenType::LeftParen => {
                    let args = parse_args(parser, gc)?.unwrap();
                    let header = lhs.header().clone();
                    let callee = parser.arena.insert(lhs);
                    lhs = Call {
                        header,
                        callee,
                        args,
                    }
                    .into();
                }
                _ => (),
            }
            continue;
        }

        let (l_bp, r_bp) = infix_binding_power(op.kind);
        if l_bp < min_bp {
            break;
        }

        parser.advance();
        let rhs = match parse_precedence(parser, r_bp, gc)? {
            Some(rhs) => rhs,
            None => return Err(parser.err("Expected righthand side of operation.")),
        };
        let left = parser.arena.insert(lhs);
        let right = parser.arena.insert(rhs);
        lhs = BinOp {
            header: parser.header(),
            left,
            right,
            op: op.kind,
        }
        .into();
    }
    Ok(Some(lhs))
}

fn prefix_binding_power(op: TokenType) -> ((), u8) {
    match op {
        TokenType::Minus | TokenType::Bang => ((), 6),
        _ => panic!("bad op: {:?}", op),
    }
}

fn infix_binding_power(op: TokenType) -> (u8, u8) {
    use self::TokenType as T;
    match op {
        T::Equal => (1, 2),
        T::And | T::Or => (2, 3),
        T::EqualEqual
        | TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::Less
        | TokenType::LessEqual
        | TokenType::BangEqual => (3, 4),
        T::Plus | T::Minus => (4, 5),
        T::Star | T::Slash => (5, 6),
        T::Dot => (6, 7),
        _ => panic!("bad op: {:?}", op),
    }
}

fn postfix_binding_power(op: TokenType) -> Option<(u8, ())> {
    let res = match op {
        TokenType::LeftParen => (8, ()),
        _ => return None,
    };
    Some(res)
}
