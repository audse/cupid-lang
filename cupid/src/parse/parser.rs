use crate::{
    arena::{ExprArena, UseArena},
    cst::{expr::ExprSource, SourceId},
    error::CupidError,
    gc::Gc,
    pointer::Pointer,
    scanner::Scanner,
    scope::{Scope, ScopeContext},
    token::{Token, TokenType},
    ty::Type,
};

use super::{iter::Iter, parse_expr, recompose::Recompose, Expr, ExprHeader};

pub struct Parser<'src> {
    scanner: Scanner<'src>,
    pub arena: ExprArena<'src>,
    pub curr: Token<'src>,
    pub prev: Token<'src>,
    pub scope: Pointer<Scope<'src>>,
    pub depth: usize,
}

impl<'src> Parser<'src> {
    pub fn new(code: impl Into<&'src str>) -> Parser<'src> {
        let scope = Pointer::<Scope>::global();
        let mut arena = ExprArena::default();
        scope.borrow_mut().initialize(&mut arena);
        Self {
            scanner: Scanner::new(code.into()),
            curr: Token::synthetic(""),
            prev: Token::synthetic(""),
            depth: 0,
            scope,
            arena,
        }
    }

    pub fn parse(&mut self, gc: &mut Gc) -> Result<Vec<Expr<'src>>, CupidError> {
        let mut exprs = vec![];
        while self.matches(TokenType::Eof).is_none() {
            match parse_expr(self, gc) {
                Ok(Some(expr)) => exprs.push(expr),
                Ok(None) => (),
                Err(err) => return Err(err),
            }
        }
        exprs.recompose(&mut self.arena)
    }

    pub fn err(&self, msg: impl ToString) -> CupidError {
        CupidError::parse_error(msg, Some(self.curr.to_static()))
    }

    pub fn expected<T>(&self, item: Option<T>, msg: impl ToString) -> Result<T, CupidError> {
        match item {
            Some(item) => Ok(item),
            None => Err(self.err(msg)),
        }
    }

    pub fn header(&self, source: impl Into<SourceId>) -> ExprHeader<'src> {
        ExprHeader {
            ty: Type::Unknown,
            scope: self.scope.clone(),
            source: source.into(),
        }
    }

    pub fn begin_scope(&mut self, context: ScopeContext) {
        let scope = self.scope.subscope(context);
        self.scope = scope;
    }

    pub fn end_scope(&mut self) -> Pointer<Scope<'src>> {
        let parent = self.scope.parent().unwrap();
        std::mem::replace(&mut self.scope, parent)
    }

    pub fn insert_source(&mut self, source: impl Into<ExprSource<'src>>) -> SourceId {
        self.arena.insert(Into::<ExprSource>::into(source)).into()
    }
}

impl<'src> Iter<'src> for Parser<'src> {
    fn next(&mut self) -> Token<'src> {
        let mut curr = self.scanner.scan_token();
        while curr.kind == TokenType::NewLine {
            match self.curr.kind {
                TokenType::Return | TokenType::Break => break,
                _ => match self.scanner.peek_token().kind {
                    TokenType::RightBrace | TokenType::LeftParen => break,
                    _ => curr = self.scanner.scan_token(),
                },
            }
        }
        curr
    }

    fn advance(&mut self) -> Token<'src> {
        self.prev = self.curr;
        loop {
            self.curr = self.next();
            match self.curr.kind {
                TokenType::Error => todo!(),
                _ => break,
            }
        }
        self.prev
    }

    fn check(&mut self, kind: TokenType) -> bool {
        self.curr.kind == kind
    }

    fn check_any(&mut self, kinds: &[TokenType]) -> bool {
        kinds.contains(&self.curr.kind)
    }

    fn matches(&mut self, kind: TokenType) -> Option<Token<'src>> {
        match self.check(kind) {
            true => Some(self.advance()),
            false => None,
        }
    }

    fn matches_any(&mut self, kinds: &[TokenType]) -> Option<Token<'src>> {
        match self.check_any(kinds) {
            true => Some(self.advance()),
            false => None,
        }
    }

    fn expect(
        &mut self,
        kind: TokenType,
        msg: impl std::fmt::Display,
    ) -> Result<Token<'src>, CupidError> {
        match self.check(kind) {
            true => Ok(self.advance()),
            false => Err(self.err(msg)),
        }
    }
}
