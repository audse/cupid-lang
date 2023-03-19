use self::{iter::Iter, parser::Parser};
use crate::{
    arena::{EntryId, UseArena},
    ast::expr::GetSource as GetExprSource,
    compiler::FunctionType,
    cst::{
        array::ArraySource,
        block::{ArrowBlockSource, BraceBlockSource},
        class::ClassSource,
        constant::ConstantSource,
        define::DefineSource,
        expr::ExprSource,
        fun::FunSource,
        get::GetSource,
        get_super::GetSuperSource,
        invoke_super::InvokeSuperSource,
        r#break::BreakSource,
        r#if::IfSource,
        r#loop::LoopSource,
        r#return::ReturnSource,
        set::SetSource,
        unop::UnOpSource,
        SourceId,
    },
    error::CupidError,
    gc::Gc,
    scope::ScopeContext,
    token::{Token, TokenType},
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
        let open_bracket = match parser.matches(TokenType::LeftBracket) {
            Some(token) => token,
            None => return Ok(None),
        };
        let mut items: Vec<EntryId> = vec![];
        let mut items_src: Vec<SourceId> = vec![];
        let mut commas: Vec<Token<'src>> = vec![];
        while !parser.check(TokenType::RightBracket) {
            match parse_expr(parser, gc)? {
                Some(expr) => {
                    items_src.push(expr.header().source);
                    items.push(parser.arena.insert(expr))
                }
                None => break,
            }
            match parser.matches(TokenType::Comma) {
                Some(comma) => commas.push(comma),
                None => break,
            }
        }
        let close_bracket =
            parser.expect(TokenType::RightBracket, "Expect ']' after array items.")?;
        let array_source = ArraySource {
            open_bracket,
            close_bracket,
            items_src,
            commas,
        }; // TODO sources
        let array_source_id = parser.arena.insert(ExprSource::from(array_source));
        Ok(Some(
            Array {
                header: parser.header(array_source_id),
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
        let break_kw = match parser.matches(TokenType::Break) {
            Some(token) => token,
            None => return Ok(None),
        };
        let value: Option<Expr<'src>> =
            match parser.matches_any(&[TokenType::Semicolon, TokenType::NewLine, TokenType::Eof]) {
                Some(_) => None,
                None => parse_expr(parser, gc)?,
            };
        let value: Option<EntryId> = match value {
            Some(value) => Some(parser.arena.insert(value)),
            None => None,
        };
        let break_source_id = parser.insert_source(BreakSource {
            break_kw,
            value_src: None,
        });
        Ok(Some(
            Break {
                header: parser.header(break_source_id),
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
        let class_kw = match parser.matches(TokenType::Class) {
            Some(token) => token,
            None => return Ok(None),
        };
        let name = parser.expect(TokenType::Identifier, "Expect class name.")?;
        let (super_class, super_class_name) = match parser.matches(TokenType::Less) {
            Some(token) => (
                Some(token),
                Some(parser.expect(TokenType::Identifier, "Expect superclass name.")?),
            ),
            None => (None, None),
        };
        let open_brace = parser.expect(TokenType::LeftBrace, "Expect '{' before body.")?;
        parser.begin_scope(ScopeContext::Class);
        let fields = parse_fields(parser, gc)?;
        let methods = parse_methods(parser, gc)?;
        let class_scope = parser.end_scope();
        let close_brace = parser.expect(TokenType::RightBrace, "Expect '}' after body.")?;
        let source_id = parser.insert_source(ClassSource {
            open_brace,
            close_brace,
            class_kw,
            name,
            super_class_name,
            super_class,
            fields: fields.iter().map(|f| f.source_id(&parser.arena)).collect(),
            methods: methods.iter().map(|f| f.source_id(&parser.arena)).collect(),
        });
        Ok(Some(
            Class {
                header: parser.header(source_id),
                name: name.lexeme,
                super_class: super_class.map(|s| s.lexeme),
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
        let kw = match parser.matches(TokenType::Fun) {
            Some(token) => token,
            None => return Ok(None),
        };
        parser.matches(TokenType::Identifier);
        Ok(Some(parse_fun(kw, parser, FunctionType::Function, gc)?.into()))
    }
}

fn parse_fun<'src>(
    kw: Token<'src>,
    parser: &mut Parser<'src>,
    function_type: FunctionType,
    gc: &mut Gc,
) -> Result<Fun<'src>, CupidError> {
    let name = match parser.prev.kind {
        TokenType::Identifier => Some(parser.prev),
        _ => None,
    };
    parser.begin_scope(ScopeContext::Fun);
    let open_paren = parser.expect(TokenType::LeftParen, "Expect '(' before parameters.")?;
    let mut params = vec![];
    let mut params_src: Vec<SourceId> = vec![];
    let mut commas = vec![];
    while !parser.check_any(&[TokenType::RightParen, TokenType::Eof]) {
        let name = parser.expect(TokenType::Identifier, "Expect parameter name.")?;
        let define_source = parser.insert_source(DefineSource {
            name,
            let_kw: None,
            equal: None,
            value_src: None,
        });
        params_src.push(define_source);
        params.push(Define {
            header: parser.header(define_source),
            name: name.lexeme,
            value: None,
        });
        match parser.matches(TokenType::Comma) {
            Some(token) => commas.push(token),
            None => break,
        }
    }
    let close_paren = parser.expect(TokenType::RightParen, "Expect ')' after parameters.")?;
    let body = Block::parse(parser, gc)?;
    let body_src = body.header.source;
    let body = parser.arena.insert(Expr::from(body));
    parser.end_scope();

    let source_id = parser.insert_source(FunSource {
        fun_kw: kw,
        name,
        params_src,
        body_src,
        open_paren,
        close_paren,
        commas,
    });
    Ok(Fun {
        header: parser.header(source_id),
        kind: function_type,
        name: name.map(|n| n.lexeme),
        params,
        body,
    })
}

impl<'src> Parse<'src> for Block<'src> {
    fn parse(parser: &mut Parser<'src>, gc: &mut Gc) -> Result<Block<'src>, CupidError> {
        if let Some(arrow) = parser.matches(TokenType::ThickArrow) {
            parser.begin_scope(ScopeContext::Block);
            let inner: Expr<'src> = parse_expect_expr(parser, gc, "Expected block.")?.into();
            let inner: EntryId = parser.arena.insert(inner);
            parser.end_scope();
            let body_src = inner.source_id(&parser.arena);
            let source_id = parser.insert_source(ArrowBlockSource { arrow, body_src });
            Ok(Block {
                header: parser.header(source_id),
                body: vec![inner],
            })
        } else {
            parser.begin_scope(ScopeContext::Block);
            let open_brace = parser.expect(TokenType::LeftBrace, "Expect '{' before block.")?;
            let mut body = vec![];
            let mut body_src = vec![]; // TODO
            while !parser.check_any(&[TokenType::RightBrace, TokenType::Eof]) {
                match parse_expr(parser, gc)? {
                    Some(expr) => {
                        body_src.push(expr.header().source);
                        let id = parser.arena.insert(Expr::from(expr));
                        body.push(id);
                    }
                    None => break,
                }
            }
            let close_brace = parser.expect(TokenType::RightBrace, "Expect '}' after block.")?;
            terminate(parser);
            parser.end_scope();
            let source_id = parser.insert_source(BraceBlockSource {
                open_brace,
                close_brace,
                body_src,
            });
            Ok(Block {
                header: parser.header(source_id),
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
            Some(name) => {
                let source_id = parser.insert_source(GetSource { name });
                Ok(Some(
                    Get {
                        header: parser.header(source_id),
                        symbol: None,
                        name: name.lexeme,
                    }
                    .into(),
                ))
            }
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
            Ok(Some(args)) => {
                let source_id = parser.insert_source(InvokeSuperSource {
                    name,
                    args: vec![],
                    open_paren: Token::synthetic("("),
                    close_paren: Token::synthetic(")"),
                    commas: vec![],
                });
                Ok(Some(
                    InvokeSuper {
                        header: parser.header(source_id),
                        symbol: None,
                        name: name.lexeme,
                        args,
                    }
                    .into(),
                ))
            }
            Ok(None) => {
                let source_id = parser.insert_source(GetSuperSource { name });
                Ok(Some(
                    GetSuper {
                        header: parser.header(source_id),
                        symbol: None,
                        name: name.lexeme,
                    }
                    .into(),
                ))
            }
            Err(e) => Err(e),
        }
    }
}

impl<'src> ParseExpr<'src> for If<'src> {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        let if_kw = match parser.matches(TokenType::If) {
            Some(token) => token,
            None => return Ok(None),
        };
        let condition = BinOp::parse_expect_expr(parser, gc, "Expected if condition.")?;
        let condition_src = condition.header().source;
        let condition = parser.arena.insert(condition);
        let body = parse_expect_expr(parser, gc, "Expected if body.")?;
        let body_src = body.header().source;
        let body = parser.arena.insert(body);
        let (else_kw, else_body, else_body_src) = match parser.matches(TokenType::Else) {
            Some(token) => {
                let else_body = parse_expect_expr(parser, gc, "Expected else body.")?;
                let else_body_src = else_body.header().source;
                (Some(token), Some(parser.arena.insert(else_body)), Some(else_body_src))
            }
            None => (None, None, None),
        };
        let source_id = parser.insert_source(IfSource {
            if_kw,
            condition_src,
            body_src,
            else_body_src,
            else_kw,
        });
        Ok(Some(
            If {
                header: parser.header(source_id),
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
        let loop_kw = match parser.matches(TokenType::Loop) {
            Some(token) => token,
            None => return Ok(None),
        };
        parser.begin_scope(ScopeContext::Loop);
        let body = Block::parse(parser, gc)?;
        let body_src = body.header.source;
        let body = parser.arena.insert(Expr::from(body));
        parser.end_scope();
        let source_id = parser.insert_source(LoopSource { loop_kw, body_src });
        Ok(Some(
            Loop {
                header: parser.header(source_id),
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
        let mut fun = parse_fun(Token::synthetic("fun"), parser, function_type, gc)?;
        let source_id = fun.source_id(&parser.arena);
        fun.name = None;
        Ok(Method {
            header: parser.header(source_id),
            name: name.lexeme,
            fun: fun.into(),
        })
    }
}

impl<'src> ParseExpr<'src> for Return<'src> {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        let return_kw = match parser.matches(TokenType::Return) {
            Some(token) => token,
            None => return Ok(None),
        };
        let value = match parser.matches_any(&[TokenType::NewLine, TokenType::Semicolon]) {
            Some(_) => None,
            None => {
                let expr = parse_expr(parser, gc)?;
                expr.map(|expr| parser.arena.insert(expr))
            }
        };
        let value_src = value.map(|val| val.source_id(&parser.arena));
        let source_id = parser.insert_source(ReturnSource {
            return_kw,
            value_src,
        });
        Ok(Some(
            Return {
                header: parser.header(source_id),
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
            Some(equal) => {
                let value = parse_expect_expr(parser, gc, "Expect value.")?;
                let value_src = value.source_id(&parser.arena);
                let value = parser.arena.insert(value);
                let source_id = parser.insert_source(SetSource {
                    name,
                    equal,
                    value_src,
                });
                Ok(Some(
                    Set {
                        header: parser.header(source_id),
                        symbol: None,
                        name: name.lexeme,
                        value,
                    }
                    .into(),
                ))
            }
            None => {
                let source_id = parser.insert_source(GetSource { name });
                Ok(Some(
                    Get {
                        header: parser.header(source_id),
                        symbol: None,
                        name: name.lexeme,
                    }
                    .into(),
                ))
            }
        }
    }
}

impl<'src> ParseExpr<'src> for Value {
    fn parse_expr(
        parser: &mut Parser<'src>,
        gc: &mut Gc,
    ) -> Result<Option<Expr<'src>>, CupidError> {
        let source = ConstantSource { value: parser.curr };
        match parser.curr.kind {
            TokenType::Float => {
                parser.advance();
                let value: f64 = parser.prev.lexeme.parse().expect("Parsed value is not a double");
                let source_id = parser.insert_source(source);
                Ok(Some(
                    Constant {
                        header: parser.header(source_id),
                        value: Value::Float(value),
                    }
                    .into(),
                ))
            }
            TokenType::Int => {
                parser.advance();
                let value: i32 =
                    parser.prev.lexeme.parse().expect("Parsed value is not an integer");
                let source_id = parser.insert_source(source);
                Ok(Some(
                    Constant {
                        header: parser.header(source_id),
                        value: Value::Int(value),
                    }
                    .into(),
                ))
            }
            TokenType::String => {
                parser.advance();
                let lexeme = parser.prev.lexeme;
                let value = &lexeme[1..(lexeme.len() - 1)];
                let source_id = parser.insert_source(source);
                Ok(Some(
                    Constant {
                        header: parser.header(source_id),
                        value: Value::String(gc.intern(value)),
                    }
                    .into(),
                ))
            }
            TokenType::False => {
                parser.advance();
                let source_id = parser.insert_source(source);
                Ok(Some(
                    Constant {
                        header: parser.header(source_id),
                        value: Value::Bool(false),
                    }
                    .into(),
                ))
            }
            TokenType::True => {
                parser.advance();
                let source_id = parser.insert_source(source);
                Ok(Some(
                    Constant {
                        header: parser.header(source_id),
                        value: Value::Bool(true),
                    }
                    .into(),
                ))
            }
            TokenType::Nil => {
                parser.advance();
                let source_id = parser.insert_source(source);
                Ok(Some(
                    Constant {
                        header: parser.header(source_id),
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
        let let_kw = match parser.matches(TokenType::Let) {
            Some(token) => token,
            None => return Ok(None),
        };
        let name = parser.expect(TokenType::Identifier, "Expect identifier.")?;
        match parser.matches(TokenType::Equal) {
            Some(equal) => {
                let value = parse_expect_expr(parser, gc, "Expect value after '='.")?;
                let value_src = value.header().source;
                let value = parser.arena.insert(value);
                let source_id = parser.insert_source(DefineSource {
                    name,
                    let_kw: Some(let_kw),
                    value_src: Some(value_src),
                    equal: Some(equal),
                });
                Ok(Some(
                    Define {
                        header: parser.header(source_id),
                        name: name.lexeme,
                        value: Some(value),
                    }
                    .into(),
                ))
            }
            None => {
                let source_id = parser.insert_source(DefineSource {
                    name,
                    let_kw: Some(let_kw),
                    value_src: None,
                    equal: None,
                });
                Ok(Some(
                    Define {
                        header: parser.header(source_id),
                        name: name.lexeme,
                        value: None,
                    }
                    .into(),
                ))
            }
        }
    }
}

fn parse_for_loop<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Option<Expr<'src>>, CupidError> {
    let kw = match parser.matches(TokenType::For) {
        Some(token) => token,
        None => return Ok(None),
    };
    parser.begin_scope(ScopeContext::Loop);

    parser.expect(TokenType::LeftParen, "Expect '(' after 'for'.")?;

    // Variable declaration
    let def = Define::parse_expect_expr(parser, gc, "Expect for loop definition.")?;
    let def = parser.arena.insert(def);

    // Break condition
    let cond = BinOp::parse_expect_expr(parser, gc, "Expect for loop condition.")?;
    let cond_src = cond.header().source;
    let cond = parser.arena.insert(cond);
    let cond_source_id = parser.insert_source(UnOpSource {
        expr_src: cond_src,
        op: Token::synthetic("todo"),
    });
    let cond = UnOp {
        header: parser.header(cond_source_id),
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
    let body_source = body.header.source;
    body.body.push(increment);

    let loop_expr: Expr<'src> = make_condition_loop(kw, parser, cond, body).into();
    let loop_expr: EntryId = parser.arena.insert(loop_expr);

    let block = Block {
        header: parser.header(body_source),
        body: vec![def, loop_expr],
    };

    parser.end_scope();
    Ok(Some(block.into()))
}

fn parse_while_loop<'src>(
    parser: &mut Parser<'src>,
    gc: &mut Gc,
) -> Result<Option<Expr<'src>>, CupidError> {
    let kw = match parser.matches(TokenType::While) {
        Some(token) => token,
        None => return Ok(None),
    };
    parser.begin_scope(ScopeContext::Loop);
    let cond = BinOp::parse_expect_expr(parser, gc, "Expected condition after 'while'.")?;
    let cond_src = cond.header().source;
    let cond = parser.arena.insert(cond);
    let cond_source_id = parser.insert_source(UnOpSource {
        expr_src: cond_src,
        op: Token::synthetic("todo"),
    });
    let cond = UnOp {
        header: parser.header(cond_source_id),
        expr: cond,
        op: TokenType::Bang,
    };
    let body = Block::parse(parser, gc)?;
    parser.end_scope();
    Ok(Some(make_condition_loop(kw, parser, cond.into(), body).into()))
}

fn make_condition_loop<'src>(
    kw: Token<'src>,
    parser: &mut Parser<'src>,
    cond: Expr<'src>,
    mut loop_body: Block<'src>,
) -> Loop<'src> {
    let cond_src = cond.header().source;
    let cond = parser.arena.insert(cond);

    let if_body_source_id = parser.insert_source(BreakSource {
        break_kw: Token::synthetic("break"),
        value_src: None,
    });
    let if_body = Break {
        header: parser.header(if_body_source_id),
        value: None,
    };
    let if_body_src = if_body.header.source;
    let if_body = parser.arena.insert(Expr::from(if_body));

    let if_stmt_source_id = parser.insert_source(IfSource {
        if_kw: Token::synthetic("if"),
        condition_src: cond_src,
        body_src: if_body_src,
        else_kw: None,
        else_body_src: None,
    });
    let if_stmt = If {
        header: parser.header(if_stmt_source_id),
        condition: cond,
        body: if_body,
        else_body: None,
    };
    let if_stmt = parser.arena.insert(Expr::from(if_stmt));
    let mut loop_body_src: Vec<SourceId> =
        loop_body.body.iter().map(|b| b.source_id(&parser.arena)).collect();
    let mut block_source = BraceBlockSource {
        open_brace: Token::synthetic("{"),
        close_brace: Token::synthetic("}"),
        body_src: vec![if_stmt_source_id.into()],
    };
    block_source.body_src.append(&mut loop_body_src);
    let block_source_id = parser.insert_source(block_source);
    let mut block = Block {
        header: parser.header(if_stmt_source_id),
        body: vec![if_stmt],
    };
    block.body.append(&mut loop_body.body);
    let body = parser.arena.insert(Expr::from(block));
    let loop_source_id = parser.insert_source(LoopSource {
        loop_kw: kw,
        body_src: block_source_id.into(),
    });
    Loop {
        header: parser.header(loop_source_id),
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
