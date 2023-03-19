use self::{iter::Iter, parser::Parser};
use crate::{
    arena::{EntryId, UseArena},
    compiler::FunctionType,
    error::CupidError,
    gc::Gc,
    scope::ScopeContext,
    token::TokenType,
    value::Value,
};
use ast::*;

pub mod bytecode;

pub mod iter;
pub mod parser;
pub mod precedence;
pub mod recompose;

pub trait Parse<'src>
where
    Self: Sized,
{
    fn parse(parser: &mut Parser<'src>, gc: &mut Gc) -> Result<Self, CupidError>;
}

pub trait ParseExpr<'src>
where
    Self: Sized,
{
    fn parse_expr(parser: &mut Parser<'src>, gc: &mut Gc)
        -> Result<Option<Expr<'src>>, CupidError>;
    fn parse_expect_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
        msg: impl ToString,
    ) -> Result<Expr<'src>, CupidError> {
        let expr = Self::parse_expr(parser, gc)?;
        parser.expected(expr, msg)
    }
}

fn terminate<'src>(parser: &mut Parser<'src>) {
    parser.matches_any(&[TokenType::Semicolon, TokenType::NewLine, TokenType::Error]);
}

macro_rules! try_parse {
    ( $rule:expr, $($rest:expr),* ) => {
        match $rule {
            Some(expr) => Ok(Some(expr)),
            None => try_parse!($($rest),*)
        }
    };
    ( $rule:expr ) => {
        match $rule {
            Some(expr) => Ok(Some(expr)),
            None => Ok(None)
        }
    };
    () => {};
}

fn parse_expr<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Option<Expr<'src>>, CupidError> {
    let expr = try_parse! {
        Class::parse_expr(parser, gc)?,
        parse_for_loop(parser, gc)?,
        parse_while_loop(parser, gc)?,
        If::parse_expr(parser, gc)?,
        Loop::parse_expr(parser, gc)?,
        Break::parse_expr(parser, gc)?,
        Return::parse_expr(parser, gc)?,
        Define::parse_expr(parser, gc)?,
        BinOp::parse_expr(parser, gc)?
    };
    terminate(parser);
    expr
}

fn parse_expect_expr<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
    msg: impl ToString,
) -> Result<Expr<'src>, CupidError> {
    let expr = parse_expr(parser, gc)?;
    parser.expected(expr, msg)
}

impl<'src> ParseExpr<'src> for Array<'src> {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        let _open_bracket = match parser.matches(TokenType::LeftBracket) {
            Some(token) => token,
            None => return Ok(None),
        };
        let mut items: Vec<EntryId> = vec![];
        while !parser.check(TokenType::RightBracket) {
            match parse_expr(parser, gc)? {
                Some(expr) => items.push(parser.arena.insert(expr)),
                None => break,
            }
            if parser.matches(TokenType::Comma).is_none() {
                break;
            }
        }
        let _close_bracket =
            parser.expect(TokenType::RightBracket, "Expect ']' after array items.")?;
        Ok(Some(
            Array {
                header: parser.header(),
                items,
            }
            .into(),
        ))
    }
}

impl<'src> ParseExpr<'src> for BinOp<'src> {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        precedence::parse_precedence(parser, 0, gc)
    }
}

impl<'src> ParseExpr<'src> for Break<'src> {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        if let None = parser.matches(TokenType::Break) {
            return Ok(None);
        }
        let value: Option<Expr<'src>> =
            match parser.matches_any(&[TokenType::Semicolon, TokenType::NewLine, TokenType::Eof]) {
                Some(_) => None,
                None => parse_expr(parser, gc)?,
            };
        let value: Option<EntryId> = match value {
            Some(value) => Some(parser.arena.insert(value)),
            None => None,
        };
        Ok(Some(
            Break {
                header: parser.header(),
                value,
            }
            .into(),
        ))
    }
}

impl<'src> ParseExpr<'src> for Class<'src> {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        if let None = parser.matches(TokenType::Class) {
            return Ok(None);
        }
        let name = parser.expect(TokenType::Identifier, "Expect class name.")?;
        let super_class = match parser.matches(TokenType::Less) {
            Some(_) => Some(parser.expect(TokenType::Identifier, "Expect superclass name.")?),
            None => None,
        };
        parser.expect(TokenType::LeftBrace, "Expect '{' before body.")?;
        parser.begin_scope(ScopeContext::Class);
        let fields = parse_fields(parser, gc)?;
        let methods = parse_methods(parser, gc)?;
        let class_scope = parser.end_scope();
        parser.expect(TokenType::RightBrace, "Expect '}' after body.")?;
        Ok(Some(
            Class {
                header: parser.header(),
                name,
                super_class,
                fields,
                methods,
                class_scope,
            }
            .into(),
        ))
    }
}

impl<'src> ParseExpr<'src> for Fun<'src> {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        if let None = parser.matches(TokenType::Fun) {
            return Ok(None);
        }
        parser.matches(TokenType::Identifier);
        Ok(Some(parse_fun(parser, FunctionType::Function, gc)?.into()))
    }
}

fn parse_fun<'src>(
    parser: &mut Parser<'src>,
    function_type: FunctionType,
    gc: &mut Gc,
) -> Result<Fun<'src>, CupidError> {
    let name = match parser.prev.kind {
        TokenType::Identifier => Some(parser.prev),
        _ => None,
    };
    parser.begin_scope(ScopeContext::Fun);
    parser.expect(TokenType::LeftParen, "Expect '(' before parameters.")?;
    let mut params = vec![];
    while !parser.check_any(&[TokenType::RightParen, TokenType::Eof]) {
        let name = parser.expect(TokenType::Identifier, "Expect parameter name.")?;
        params.push(Define {
            header: parser.header(),
            name,
            value: None,
        });
        if parser.matches(TokenType::Comma).is_none() {
            break;
        }
    }
    parser.expect(TokenType::RightParen, "Expect ')' after parameters.")?;
    let body = Block::parse(parser, gc)?;
    let body = parser.arena.insert(Expr::from(body));
    parser.end_scope();
    Ok(Fun {
        header: parser.header(),
        kind: function_type,
        name,
        params,
        body,
    })
}

impl<'src> Parse<'src> for Block<'src> {
    fn parse(parser: &mut Parser<'src>, gc: &mut Gc) -> Result<Block<'src>, CupidError> {
        if parser.matches(TokenType::ThickArrow).is_some() {
            parser.begin_scope(ScopeContext::Block);
            let inner: Expr<'src> = parse_expect_expr(parser, gc, "Expected block.")?.into();
            let inner: EntryId = parser.arena.insert(inner);
            parser.end_scope();
            Ok(Block {
                header: parser.header(),
                body: vec![inner],
            })
        } else {
            parser.begin_scope(ScopeContext::Block);
            parser.expect(TokenType::LeftBrace, "Expect '{' before block.")?;
            let mut body = vec![];
            while !parser.check_any(&[TokenType::RightBrace, TokenType::Eof]) {
                match parse_expr(parser, gc)? {
                    Some(expr) => {
                        let id = parser.arena.insert(Expr::from(expr));
                        body.push(id);
                    }
                    None => break,
                }
            }
            parser.expect(TokenType::RightBrace, "Expect '}' after block.")?;
            terminate(parser);
            parser.end_scope();
            Ok(Block {
                header: parser.header(),
                body,
            })
        }
    }
}

fn parse_block_expr<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Option<Expr<'src>>, CupidError> {
    match parser.check_any(&[TokenType::ThickArrow, TokenType::LeftBrace]) {
        true => Ok(Some(Block::parse(parser, gc)?.into())),
        false => Ok(None),
    }
}

impl<'src> ParseExpr<'src> for Get<'src> {
    fn parse_expr(
        parser: &mut Parser<'src>,
        _gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        match parser.matches_any(&[TokenType::Identifier, TokenType::This, TokenType::Log]) {
            Some(name) => Ok(Some(
                Get {
                    header: parser.header(),
                    symbol: None,
                    name,
                }
                .into(),
            )),
            None => Ok(None),
        }
    }
}

impl<'src> ParseExpr<'src> for GetSuper<'src> {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        let name = match parser.matches(TokenType::Super) {
            Some(name) => name,
            None => return Ok(None),
        };
        match parse_args(parser, gc) {
            Ok(Some(args)) => Ok(Some(
                InvokeSuper {
                    header: parser.header(),
                    symbol: None,
                    name,
                    args,
                }
                .into(),
            )),
            Ok(None) => Ok(Some(
                GetSuper {
                    header: parser.header(),
                    symbol: None,
                    name,
                }
                .into(),
            )),
            Err(e) => Err(e),
        }
    }
}

impl<'src> ParseExpr<'src> for If<'src> {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        if let None = parser.matches(TokenType::If) {
            return Ok(None);
        }
        let condition = BinOp::parse_expect_expr(parser, gc, "Expected if condition.")?;
        let condition = parser.arena.insert(condition);
        let body = parse_expect_expr(parser, gc, "Expected if body.")?;
        let body = parser.arena.insert(body);
        let else_body = match parser.matches(TokenType::Else) {
            Some(_) => {
                let else_body = parse_expect_expr(parser, gc, "Expected else body.")?;
                Some(parser.arena.insert(else_body))
            }
            None => None,
        };
        Ok(Some(
            If {
                header: parser.header(),
                condition,
                body,
                else_body,
            }
            .into(),
        ))
    }
}

impl<'src> ParseExpr<'src> for Loop<'src> {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        if let None = parser.matches(TokenType::Loop) {
            return Ok(None);
        }
        parser.begin_scope(ScopeContext::Loop);
        let body = Block::parse(parser, gc)?;
        let body = parser.arena.insert(Expr::from(body));
        parser.end_scope();
        Ok(Some(
            Loop {
                header: parser.header(),
                body,
            }
            .into(),
        ))
    }
}

fn parse_fields<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Vec<Define<'src>>, CupidError> {
    let mut fields = vec![];
    while parser.check(TokenType::Let) {
        let field = match Define::parse_expr(parser, gc) {
            Ok(Some(Expr::Define(inner))) => inner,
            Ok(None) => break,
            _ => panic!("Expected field definition."),
        };
        fields.push(field);
    }
    Ok(fields)
}

fn parse_methods<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Vec<Method<'src>>, CupidError> {
    let mut methods = vec![];
    while !parser.check_any(&[TokenType::RightBrace, TokenType::Eof]) {
        methods.push(Method::parse(parser, gc)?.into());
    }
    terminate(parser);
    Ok(methods)
}

impl<'src> Parse<'src> for Method<'src> {
    fn parse(parser: &mut Parser<'src>, gc: &mut Gc) -> Result<Self, CupidError> {
        let name = parser.expect(TokenType::Identifier, "Expect method name.")?;
        let function_type = match name.lexeme {
            "init" => FunctionType::Initializer,
            _ => FunctionType::Method,
        };
        let mut fun = parse_fun(parser, function_type, gc)?;
        fun.name = None;
        Ok(Method {
            header: parser.header(),
            name,
            fun: fun.into(),
        })
    }
}

impl<'src> ParseExpr<'src> for Return<'src> {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        if let None = parser.matches(TokenType::Return) {
            return Ok(None);
        }
        let value = match parser.matches_any(&[TokenType::NewLine, TokenType::Semicolon]) {
            Some(_) => None,
            None => {
                let expr = parse_expr(parser, gc)?;
                expr.map(|expr| parser.arena.insert(expr))
            }
        };
        Ok(Some(
            Return {
                header: parser.header(),
                value,
            }
            .into(),
        ))
    }
}

impl<'src> ParseExpr<'src> for Set<'src> {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        let name = match parser.matches(TokenType::Identifier) {
            Some(name) => name,
            None => return Ok(None),
        };
        match parser.matches(TokenType::Equal) {
            Some(_) => {
                let value = parse_expect_expr(parser, gc, "Expect value.")?;
                let value = parser.arena.insert(value);
                Ok(Some(
                    Set {
                        header: parser.header(),
                        symbol: None,
                        name,
                        value,
                    }
                    .into(),
                ))
            }
            None => Ok(Some(
                Get {
                    header: parser.header(),
                    symbol: None,
                    name,
                }
                .into(),
            )),
        }
    }
}

impl<'src> ParseExpr<'src> for Value {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        match parser.curr.kind {
            TokenType::Float => {
                parser.advance();
                let value: f64 = parser.prev.lexeme.parse().expect("Parsed value is not a double");
                Ok(Some(
                    Constant {
                        header: parser.header(),
                        value: Value::Float(value),
                    }
                    .into(),
                ))
            }
            TokenType::Int => {
                parser.advance();
                let value: i32 =
                    parser.prev.lexeme.parse().expect("Parsed value is not an integer");
                Ok(Some(
                    Constant {
                        header: parser.header(),
                        value: Value::Int(value),
                    }
                    .into(),
                ))
            }
            TokenType::String => {
                parser.advance();
                let lexeme = parser.prev.lexeme;
                let value = &lexeme[1..(lexeme.len() - 1)];
                Ok(Some(
                    Constant {
                        header: parser.header(),
                        value: Value::String(gc.intern(value)),
                    }
                    .into(),
                ))
            }
            TokenType::False => {
                parser.advance();
                Ok(Some(
                    Constant {
                        header: parser.header(),
                        value: Value::Bool(false),
                    }
                    .into(),
                ))
            }
            TokenType::True => {
                parser.advance();
                Ok(Some(
                    Constant {
                        header: parser.header(),
                        value: Value::Bool(true),
                    }
                    .into(),
                ))
            }
            TokenType::Nil => {
                parser.advance();
                Ok(Some(
                    Constant {
                        header: parser.header(),
                        value: Value::Nil,
                    }
                    .into(),
                ))
            }
            _ => Ok(None),
        }
    }
}

impl<'src> ParseExpr<'src> for Define<'src> {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        if let None = parser.matches(TokenType::Let) {
            return Ok(None);
        }
        let name = parser.expect(TokenType::Identifier, "Expect identifier.")?;
        match parser.matches(TokenType::Equal) {
            Some(_) => {
                let value = parse_expect_expr(parser, gc, "Expect value after '='.")?;
                let value = parser.arena.insert(value);
                Ok(Some(
                    Define {
                        header: parser.header(),
                        name,
                        value: Some(value),
                    }
                    .into(),
                ))
            }
            None => Ok(Some(
                Define {
                    header: parser.header(),
                    name,
                    value: None,
                }
                .into(),
            )),
        }
    }
}

fn parse_for_loop<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Option<Expr<'src>>, CupidError> {
    if let None = parser.matches(TokenType::For) {
        return Ok(None);
    }
    parser.begin_scope(ScopeContext::Loop);

    parser.expect(TokenType::LeftParen, "Expect '(' after 'for'.")?;

    // Variable declaration
    let def = Define::parse_expect_expr(parser, gc, "Expect for loop definition.")?;
    let def = parser.arena.insert(def);

    // Break condition
    let cond = BinOp::parse_expect_expr(parser, gc, "Expect for loop condition.")?;
    let cond = parser.arena.insert(cond);
    let cond = UnOp {
        header: parser.header(),
        expr: cond,
        op: TokenType::Bang,
    }
    .into();
    parser.expect(TokenType::Semicolon, "Expect ';'.")?;

    // Increment
    let increment = BinOp::parse_expect_expr(parser, gc, "Expect increment.")?;
    let increment = parser.arena.insert(increment);

    parser.expect(TokenType::RightParen, "Expect ')' after 'for'.")?;
    let mut body = Block::parse(parser, gc)?;
    body.body.push(increment);

    let loop_expr: Expr<'src> = make_condition_loop(parser, cond, body).into();
    let loop_expr: EntryId = parser.arena.insert(loop_expr);

    let block = Block {
        header: parser.header(),
        body: vec![def, loop_expr],
    };

    parser.end_scope();
    Ok(Some(block.into()))
}

fn parse_while_loop<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Option<Expr<'src>>, CupidError> {
    if let None = parser.matches(TokenType::While) {
        return Ok(None);
    }
    parser.begin_scope(ScopeContext::Loop);
    let cond = BinOp::parse_expect_expr(parser, gc, "Expected condition after 'while'.")?;
    let cond = parser.arena.insert(cond);
    let cond = UnOp {
        header: parser.header(),
        expr: cond,
        op: TokenType::Bang,
    };
    let body = Block::parse(parser, gc)?;
    parser.end_scope();
    Ok(Some(make_condition_loop(parser, cond.into(), body).into()))
}

fn make_condition_loop<'src>(
    parser: &mut Parser<'src>,
    cond: Expr<'src>,
    mut loop_body: Block<'src>,
) -> Loop<'src> {
    let cond = parser.arena.insert(cond);
    let if_body = parser.arena.insert(Expr::from(Break {
        header: parser.header(),
        value: None,
    }));
    let if_stmt = If {
        header: parser.header(),
        condition: cond,
        body: if_body,
        else_body: None,
    };
    let if_stmt = parser.arena.insert(Expr::from(if_stmt));
    let mut block = Block {
        header: parser.header(),
        body: vec![if_stmt],
    };
    block.body.append(&mut loop_body.body);
    let body = parser.arena.insert(Expr::from(block));
    Loop {
        header: parser.header(),
        body,
    }
}

fn parse_args<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Option<Vec<EntryId>>, CupidError> {
    if parser.matches(TokenType::LeftParen).is_some() {
        let mut args = vec![];
        while !parser.check(TokenType::RightParen) {
            match parse_expr(parser, gc)? {
                Some(arg) => args.push(parser.arena.insert(Expr::from(arg))),
                None => break,
            };
            if parser.matches(TokenType::Comma).is_none() {
                break;
            }
        }
        parser.expect(TokenType::RightParen, "Expect ')' after arguments.")?;
        return Ok(Some(args));
    }
    Ok(None)
}

fn parse_group<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Option<Expr<'src>>, CupidError> {
    if let None = parser.matches(TokenType::LeftParen) {
        return Ok(None);
    }
    let inner = parse_expr(parser, gc)?;
    parser.expect(TokenType::RightParen, "Expect ')' after group.")?;
    Ok(inner)
}

fn parse_unit<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Option<Expr<'src>>, CupidError> {
    Ok(try_parse! {
        parse_group(parser, gc)?,
        parse_block_expr(parser, gc)?,
        Value::parse_expr(parser, gc)?,
        GetSuper::parse_expr(parser, gc)?,
        Get::parse_expr(parser, gc)?,
        Array::parse_expr(parser, gc)?,
        Fun::parse_expr(parser, gc)?
    }?)
    .or_else(|_: CupidError| match parser.matches(TokenType::Error) {
        Some(_) => {
            parser.advance();
            parse_expr(parser, gc)
        }
        None => Ok(None),
    })
}
