use crate::{
    Token,
	TokenType,
	Operator,
	CupidExpression,
	Symbol,
	Assign,
	Lexer,
	CupidScope,
	CupidValue,
	Tree,
};
mod peg;
use std::any::Any;
pub use peg::Parser;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub index: usize,
}

impl Parser {
	
	pub fn new(tokens: Vec<Token>) -> Self {
		Self {
			tokens,
			index: 0,
		}
	}
	
	pub fn parse(&mut self) -> CupidExpression {
		return binary_op(self, 0);
	}
	
	pub fn parse_block(&mut self) -> Vec<CupidExpression> {
		let mut block: Vec<CupidExpression> = vec![];
		while !self.is_at_end() {
			block.push(binary_op(self, 0));
		}
		return block;
	}
	
	fn advance(&mut self) -> Option<&Token> {
		if !self.is_at_end() { self.index += 1; }
		return self.previous();
	}
	
	fn is_at_end(&mut self) -> bool {
		match self.peek().token_type {
			TokenType::Eof => return true,
			_ => return false
		}
	}
	
	fn peek(&self) -> &Token {
		return &self.tokens[self.index];
	}
	
	fn peek_mut(&mut self) -> Token {
		return self.tokens[self.index].clone();
	}
	
	fn previous(&mut self) -> Option<&Token> {
		if self.index > 0 {
			return Some(&self.tokens[self.index - 1]);
		}
		return None;
	}
}

fn binary_op(parser: &mut Parser, min_binding_pow: u8) -> CupidExpression {
	if let Some(token) = parser.advance() {
		let mut left = match &token.token_type {
			TokenType::Symbol(Symbol::LeftParen) => {
				let left = binary_op(parser, 0);
				if let Some(next) = parser.advance() {
					assert_eq!(next.token_type, TokenType::Symbol(Symbol::RightParen));
				}
				left
			},
			TokenType::Operator(op) => {
				let ((), right_binding_pow) = prefix_binding_pow(&TokenType::Operator(*op));
				CupidExpression::new_operator(*op, CupidExpression::new_none_node(), binary_op(parser, right_binding_pow))
			},
			_ => CupidExpression::from_token(token)
		};

		loop {
			let operator = parser.peek_mut();
			let op_token = match &operator.token_type {
				TokenType::Eof => break,
				TokenType::Operator(op) => TokenType::Operator(*op),
				TokenType::Symbol(symbol) => TokenType::Symbol(*symbol),
				TokenType::Assign(a) => TokenType::Assign(*a),
				_ => break,
				// token => panic!("Unexpected non-operator token: {}", token)
			};
			
			if let Some((left_binding_pow, right_binding_pow)) = infix_binding_pow(&op_token) {
				
				if left_binding_pow < min_binding_pow { break; }
				
				parser.advance();
				let right = binary_op(parser, right_binding_pow);
				match &op_token {
					TokenType::Assign(a) => {
						left = match &left {
							CupidExpression::CupidSymbol(s) => CupidExpression::new_assign(*a, s.clone(), right),
							_ => panic!("Attempted to assign a value to a non-symbol")
						};
						break;
					},
					TokenType::Operator(op) => {
						left = CupidExpression::new_operator(*op, left, right);
					},
					_ => ()
				}
			}
			
			break;
			
		};
		return left;
	};
	return CupidExpression::CupidEmpty;
}

fn prefix_binding_pow(operator: &TokenType) -> ((), u8) {
	match operator {
		TokenType::Operator(Operator::Add) | TokenType::Operator(Operator::Subtract) => ((), 8),
		op => panic!("Operator supplied is not a prefix: {}", op),
	}
}

fn infix_binding_pow(operator: &TokenType) -> Option<(u8, u8)> {
	let result = match operator {
		TokenType::Assign(Assign::Equal) => (1, 2),
		
		TokenType::Operator(Operator::Equal) 
		| TokenType::Operator(Operator::NotEqual)
		| TokenType::Operator(Operator::Greater)
		| TokenType::Operator(Operator::GreaterEqual)
		| TokenType::Operator(Operator::Less)
		| TokenType::Operator(Operator::LessEqual) => (3, 4),
		
		TokenType::Operator(Operator::Add) 
		| TokenType::Operator(Operator::Subtract) => (5, 6),
		
		TokenType::Operator(Operator::Multiply) 
		| TokenType::Operator(Operator::Divide) => (7, 8),
		
		TokenType::Symbol(Symbol::Dot) => (11, 12),
		
		_ => return None,
	};
	return Some(result);
}

#[test]
fn test_block() {
	
	fn test(input: &str, expected: Box<dyn Any>) {
		let mut scope = CupidScope::new(None);
		
		let mut lexer = Lexer::new(input.to_string(), true);
		lexer.scan();
		let mut parser = Parser { tokens: lexer.tokens, index: 0 };
		
		let mut result = CupidValue::None;
		let block = parser.parse_block();
		for exp in block {
			result = exp.resolve(&mut scope);
		}
		assert!(result.is_equal(expected));
	}
	
	test("x = 1 x + 1", Box::new(2));
	test("(5 + 5) * 2", Box::new(20));
	test("1 - 1 - 1", Box::new(-1));
	test("-10", Box::new(-10));
	test("1.5 - 0.5", Box::new(1.0));
}

