use self::{iter::Iter, parser::Parser};
use crate::{
    compiler::FunctionType, error::CupidError, gc::Gc, scope::ScopeContext, token::TokenType,
    value::Value,
};
pub use ast::expr::*;

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

pub trait ParseInst<'src>
where
    Self: Sized,
{
    fn parse_inst(parser: &mut Parser<'src>, gc: &mut Gc)
        -> Result<Option<Expr<'src>>, CupidError>;
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

fn parse_inst<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Option<Expr<'src>>, CupidError> {
    let expr = try_parse! {
        Class::parse_inst(parser, gc)?,
        parse_for_loop(parser, gc)?,
        parse_while_loop(parser, gc)?,
        If::parse_inst(parser, gc)?,
        Loop::parse_inst(parser, gc)?,
        Break::parse_inst(parser, gc)?,
        Return::parse_inst(parser, gc)?,
        Define::parse_inst(parser, gc)?,
        BinOp::parse_inst(parser, gc)?
    };
    terminate(parser);
    expr
}

impl<'src> ParseInst<'src> for Array<'src> {
    fn parse_inst(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        if let None = parser.matches(TokenType::LeftBracket) {
            return Ok(None);
        }
        let mut items: Vec<Expr<'_>> = vec![];
        while !parser.check(TokenType::RightBracket) {
            match parse_inst(parser, gc)? {
                Some(expr) => items.push(expr),
                None => break,
            }
            if parser.matches(TokenType::Comma).is_none() {
                break;
            }
        }
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

impl<'src> ParseInst<'src> for BinOp<'src> {
    fn parse_inst(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        precedence::parse_precedence(parser, 0, gc)
    }
}

impl<'src> ParseInst<'src> for Break<'src> {
    fn parse_inst(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        if let None = parser.matches(TokenType::Break) {
            return Ok(None);
        }
        let value =
            match parser.matches_any(&[TokenType::Semicolon, TokenType::NewLine, TokenType::Eof]) {
                Some(_) => None,
                None => parse_inst(parser, gc)?.map(Box::new),
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

impl<'src> ParseInst<'src> for Class<'src> {
    fn parse_inst(
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
        parser.begin_scope(ScopeContext::Class);
        let methods = parse_methods(parser, gc)?;
        parser.end_scope();
        Ok(Some(
            Class {
                header: parser.header(),
                name,
                super_class,
                methods,
            }
            .into(),
        ))
    }
}

impl<'src> ParseInst<'src> for Fun<'src> {
    fn parse_inst(
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
            let inner = parse_inst(parser, gc)?;
            let inner = parser.expected(inner, "Expected block.")?.into();
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
                match parse_inst(parser, gc)? {
                    Some(expr) => body.push(expr.into()),
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

fn parse_block_inst<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Option<Expr<'src>>, CupidError> {
    match parser.check_any(&[TokenType::ThickArrow, TokenType::LeftBrace]) {
        true => Ok(Some(Block::parse(parser, gc)?.into())),
        false => Ok(None),
    }
}

impl<'src> ParseInst<'src> for Get<'src> {
    fn parse_inst(
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

impl<'src> ParseInst<'src> for GetSuper<'src> {
    fn parse_inst(
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

impl<'src> ParseInst<'src> for If<'src> {
    fn parse_inst(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        if let None = parser.matches(TokenType::If) {
            return Ok(None);
        }
        let condition = BinOp::parse_inst(parser, gc)?;
        let condition = parser.expected(condition, "Expected if condition.")?;
        let body = parse_inst(parser, gc)?;
        let body = parser.expected(body, "Expected if body.")?;
        let else_body = match parser.matches(TokenType::Else) {
            Some(_) => {
                let else_body = parse_inst(parser, gc)?;
                Some(Box::new(parser.expected(else_body, "Expected else body.")?))
            }
            None => None,
        };
        Ok(Some(
            If {
                header: parser.header(),
                condition: Box::new(condition),
                body: Box::new(body),
                else_body,
            }
            .into(),
        ))
    }
}

impl<'src> ParseInst<'src> for Loop<'src> {
    fn parse_inst(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        if let None = parser.matches(TokenType::Loop) {
            return Ok(None);
        }
        parser.begin_scope(ScopeContext::Loop);
        let body = Block::parse(parser, gc)?.into();
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

fn parse_methods<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Vec<Method<'src>>, CupidError> {
    parser.expect(TokenType::LeftBrace, "Expect '{' before body.")?;
    let mut methods = vec![];
    while !parser.check_any(&[TokenType::RightBrace, TokenType::Eof]) {
        methods.push(Method::parse(parser, gc)?.into());
    }
    parser.expect(TokenType::RightBrace, "Expect '}' after body.")?;
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

impl<'src> ParseInst<'src> for Return<'src> {
    fn parse_inst(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        if let None = parser.matches(TokenType::Return) {
            return Ok(None);
        }
        let value = match parser.matches_any(&[TokenType::NewLine, TokenType::Semicolon]) {
            Some(_) => None,
            None => parse_inst(parser, gc)?.map(Box::new),
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

impl<'src> ParseInst<'src> for Set<'src> {
    fn parse_inst(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        let name = match parser.matches(TokenType::Identifier) {
            Some(name) => name,
            None => return Ok(None),
        };
        match parser.matches(TokenType::Equal) {
            Some(_) => {
                let value = parse_inst(parser, gc)?;
                Ok(Some(
                    Set {
                        header: parser.header(),
                        symbol: None,
                        name,
                        value: Box::new(parser.expected(value, "Expect value.")?),
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

impl<'src> ParseInst<'src> for Value {
    fn parse_inst(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        match parser.curr.kind {
            TokenType::Float => {
                parser.advance();
                let value: f64 = parser.prev.lexeme.parse().expect("Parsed value is not a double");
                Ok(Some(Value::Float(value).into()))
            }
            TokenType::Int => {
                parser.advance();
                let value: i32 =
                    parser.prev.lexeme.parse().expect("Parsed value is not an integer");
                Ok(Some(Value::Int(value).into()))
            }
            TokenType::String => {
                parser.advance();
                let lexeme = parser.prev.lexeme;
                let value = &lexeme[1..(lexeme.len() - 1)];
                Ok(Some(Value::String(gc.intern(value)).into()))
            }
            TokenType::False => {
                parser.advance();
                Ok(Some(Value::Bool(false).into()))
            }
            TokenType::True => {
                parser.advance();
                Ok(Some(Value::Bool(true).into()))
            }
            TokenType::Nil => {
                parser.advance();
                Ok(Some(Value::Nil.into()))
            }
            _ => Ok(None),
        }
    }
}

impl<'src> ParseInst<'src> for Define<'src> {
    fn parse_inst(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        if let None = parser.matches(TokenType::Let) {
            return Ok(None);
        }
        let name = parser.expect(TokenType::Identifier, "Expect identifier.")?;
        match parser.matches(TokenType::Equal) {
            Some(_) => {
                let value = parse_inst(parser, gc)?;
                Ok(Some(
                    Define {
                        header: parser.header(),
                        name,
                        value: Some(Box::new(parser.expected(value, "Expect value after '='.")?)),
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
    let def = Define::parse_inst(parser, gc)?;
    let def = parser.expected(def, "Expect for loop definition.")?;
    // Break condition

    let cond = BinOp::parse_inst(parser, gc)?;
    let cond = parser.expected(cond, "Expect for loop condition.")?;
    let cond = UnOp {
        header: parser.header(),
        expr: Box::new(cond),
        op: TokenType::Bang,
    }
    .into();
    parser.expect(TokenType::Semicolon, "Expect ';'.")?;

    // Increment
    let increment = BinOp::parse_inst(parser, gc)?;
    let increment = parser.expected(increment, "Expect increment.")?;

    parser.expect(TokenType::RightParen, "Expect ')' after 'for'.")?;
    let mut body = Block::parse(parser, gc)?;
    body.body.push(increment);
    let block = Block {
        header: parser.header(),
        body: vec![def, make_condition_loop(parser, cond, body).into()],
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
    let cond = BinOp::parse_inst(parser, gc)?;
    let cond = parser.expected(cond, "Expected condition after 'while'.")?;
    let cond = UnOp {
        header: parser.header(),
        expr: Box::new(cond),
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
    let if_stmt = If {
        header: parser.header(),
        condition: Box::new(cond),
        body: Box::new(
            Break {
                header: parser.header(),
                value: None,
            }
            .into(),
        ),
        else_body: None,
    };
    let mut body: Vec<Expr<'src>> = vec![if_stmt.into()];
    body.append(&mut loop_body.body);
    Loop {
        header: parser.header(),
        body: Block {
            header: parser.header(),
            body,
        }
        .into(),
    }
}

fn parse_args<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Option<Vec<Expr<'src>>>, CupidError> {
    if parser.matches(TokenType::LeftParen).is_some() {
        let mut args = vec![];
        while !parser.check(TokenType::RightParen) {
            match parse_inst(parser, gc)? {
                Some(arg) => args.push(arg.into()),
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
    let inner = parse_inst(parser, gc)?;
    parser.expect(TokenType::RightParen, "Expect ')' after group.")?;
    Ok(inner)
}

fn parse_unit<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Option<Expr<'src>>, CupidError> {
    Ok(try_parse! {
        parse_group(parser, gc)?,
        parse_block_inst(parser, gc)?,
        Value::parse_inst(parser, gc)?,
        GetSuper::parse_inst(parser, gc)?,
        Get::parse_inst(parser, gc)?,
        Array::parse_inst(parser, gc)?,
        Fun::parse_inst(parser, gc)?
    }?)
    .or_else(|_: CupidError| match parser.matches(TokenType::Error) {
        Some(_) => {
            parser.advance();
            parse_inst(parser, gc)
        }
        None => Ok(None),
    })
}
