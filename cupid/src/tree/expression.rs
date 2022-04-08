use std::fmt::{
	Display,
	Formatter,
	Result,
};
use crate::{
	CupidScope,
	CupidValue,
	CupidSymbol,
	TokenType,
	Literal,
	Keyword,
	Operator,
	Token,
	Assign,
	CupidBlock,
	CupidIfBlock,
	CupidOperator,
	CupidAssign,
	CupidDeclare,
	CupidFunction,
	CupidFunctionCall,
	CupidNode,
	Tree,
};

#[derive(Debug, Hash, Clone)]
pub enum CupidExpression {
	CupidBlock(CupidBlock),
	CupidIfBlock(CupidIfBlock),
	CupidOperator(CupidOperator),
	CupidAssign(CupidAssign),
	CupidDeclare(CupidDeclare),
	CupidNode(CupidNode),
	CupidSymbol(CupidSymbol),
	CupidFunction(CupidFunction),
	CupidFunctionCall(CupidFunctionCall),
	CupidEmpty,
}

impl Display for CupidExpression {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Expression: {:?}", self)
	}
}

impl CupidExpression {
	pub fn new_node(value: CupidValue) -> Self { Self::CupidNode(CupidNode { value }) }
	pub fn new_string_node(string: String) -> Self {  Self::new_node(CupidValue::String(string)) }
	pub fn new_int_node(int: i32) -> Self { Self::new_node(CupidValue::Integer(int)) }
	pub fn new_dec_node(front: i32, back: u32) -> Self {  Self::new_node(CupidValue::Decimal { front, back }) }
	pub fn new_bool_node(boo: bool) -> Self { Self::new_node(CupidValue::Boolean(boo)) }
	pub fn new_none_node() -> Self { Self::new_node(CupidValue::None) }
	pub fn new_operator(operator: Operator, left: CupidExpression, right: CupidExpression) -> Self {
		Self::CupidOperator(CupidOperator::new(operator, left, right))
	}
	pub fn new_symbol(identifier: CupidValue, mutable: bool, deep_mutable: bool) -> Self {
		Self::CupidSymbol(CupidSymbol { identifier, mutable, deep_mutable })
	}
	pub fn new_assign(operator: Assign, symbol: CupidSymbol, value: CupidExpression) -> Self {
		Self::CupidAssign(CupidAssign { operator, symbol, value: Box::new(value) })
	}
	pub fn new_declare(symbol: CupidSymbol, value: CupidExpression) -> Self {
		Self::CupidDeclare(CupidDeclare { symbol, value: Box::new(value) })
	}
	pub fn new_block(expressions: Vec<CupidExpression>) -> Self {
		Self::CupidBlock(CupidBlock { expressions })
	}
	pub fn new_if_block(condition: CupidExpression, body: CupidBlock, else_body: Option<CupidBlock>) -> Self {
		Self::CupidIfBlock(CupidIfBlock { condition: Box::new(condition), body, else_body })
	}
	
	pub fn from_token(token: &Token) -> Self {
		match token.token_type {
			TokenType::Literal(Literal::String) => Self::new_string_node(token.source.clone()),
			TokenType::Literal(Literal::Number) => Self::new_int_node(token.source.clone().parse::<i32>().unwrap()),
			TokenType::Literal(Literal::Decimal) => {
				let string: String = token.source.clone();
				let parts: Vec<&str> = string.split(".").collect();
				let front = parts[0].parse::<i32>().unwrap();
				let back = parts[1].parse::<u32>().unwrap();
				Self::new_dec_node(front, back)
			},
			TokenType::Keyword(Keyword::True) => Self::new_bool_node(true),
			TokenType::Keyword(Keyword::False) => Self::new_bool_node(false),
			TokenType::Keyword(Keyword::None) => Self::new_none_node(),
			TokenType::Identifier => Self::new_symbol(CupidValue::String(token.source.clone()), true, true),
			token => panic!("Unknown token type, unable to create node from: {}", token)
		}
	}
}

impl Tree for CupidExpression {
	fn resolve(&self, scope: &mut CupidScope) -> CupidValue {
		match self {
			Self::CupidNode(n) => n.resolve(scope),
			Self::CupidOperator(o) => o.resolve(scope),
			Self::CupidSymbol(s) => s.resolve(scope),
			Self::CupidAssign(a) => a.resolve(scope),
			Self::CupidDeclare(a) => a.resolve(scope),
			Self::CupidBlock(b) => b.resolve(scope),
			Self::CupidIfBlock(b) => b.resolve(scope),
			Self::CupidFunction(b) => b.resolve(scope),
			Self::CupidFunctionCall(b) => b.resolve(scope),
			_ => CupidValue::None
		}
	}
}