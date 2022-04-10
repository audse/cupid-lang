use std::fmt;
use crate::{
    Token,
    TokenType,
};

#[derive(Debug)]
pub enum Expression {
    EmptyExpression,
    BinaryExpression(Box<BinaryExpression>),
    UnaryExpression(Box<UnaryExpression>),
    LiteralExpression(Box<LiteralExpression>),
}

impl Expression {

    pub fn new_binary(left: Self, operator: &Token, right: Self) -> Self {
        return Self::BinaryExpression(BinaryExpression::new_box(left, operator, right));
    }

    pub fn new_unary(operator: &Token, right: Self) -> Self {
        return Self::UnaryExpression(UnaryExpression::new_box(operator, right));
    }

    pub fn new_literal(token: &Token) -> Self {
        return Self::LiteralExpression(LiteralExpression::new_box(token));
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
    }
}


#[derive(Debug)]
pub struct BinaryExpression {
    pub left_exp: Expression,
    pub operator: Token,
    pub right_exp: Expression
}

impl BinaryExpression {
    pub fn new(left_exp: Expression, operator: &Token, right_exp: Expression) -> Self {
        let operator_token = Token::new_copy(operator);
        Self {
            left_exp: left_exp,
            operator: operator_token,
            right_exp: right_exp
        }
    }

    pub fn new_box(left_exp: Expression, operator: &Token, right_exp: Expression) -> Box<Self> {
        return Box::new(Self::new(left_exp, operator, right_exp));
    }
}

#[derive(Debug)]
pub struct UnaryExpression {
    operator: Token,
    right_exp: Expression
}

impl UnaryExpression {
    pub fn new(operator: &Token, right_exp: Expression) -> Self {
        let operator_token = Token::new_copy(operator);
        Self {
            operator: operator_token,
            right_exp: right_exp
        }
    }

    pub fn new_box(operator: &Token, right_exp: Expression) -> Box<Self> {
        return Box::new(Self::new(operator, right_exp));
    }
}

#[derive(Debug)]
pub struct LiteralExpression {
    pub token: Token
}

impl LiteralExpression {
    pub fn new(token: &Token) -> Self {
        let token_copy = Token::new_copy(token);
        Self {
            token: token_copy
        }
    }

    pub fn new_box(token: &Token) -> Box<Self> {
        return Box::new(Self::new(token));
    }
}