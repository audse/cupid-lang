use crate::{
	Error,
	CupidExpression,
	CupidFunction,
	CupidFunctionCall,
	CupidSymbol,
	CupidValue,
	Token,
	TokenType,
	Assign,
	Operator,
	Keyword,
	Symbol,
	Literal,
	Lexer,
};

pub struct Parser {
	tokens: Vec<Token>,
	index: usize,
}

impl Parser {
	pub fn new(lexer: Lexer) -> Self {
		Self {
			tokens: lexer.tokens,
			index: 0,
		}
	}
	
	pub fn parse(&mut self) -> Vec<CupidExpression> {
		let mut result = vec![];
		while !self.is_at_end() {
			if let Some(exp) = self.expression() {
				result.push(exp);
			}
		}
		return result;
	}
	
	fn is_at_end(&self) -> bool { self.index == self.tokens.len() - 1 }
	
	fn mark(&self) -> usize { self.index }
	fn current(&self) -> &Token { &self.tokens[self.index] }
	
	fn reset(&mut self, pos: usize) { self.index = pos; }
	
	fn expect(&mut self, token_type: TokenType) -> Option<&Token> {
		let next_token = &self.tokens[self.index];
		if token_type == next_token.token_type {
			self.index += 1;
			return Some(next_token);
		}
		return None;
	}
	
	fn expect_one(&mut self, token_types: Vec<TokenType>) -> Option<&Token> {
		let next_token = &self.tokens[self.index];
		for token_type in token_types {
			if token_type == next_token.token_type {
				self.index += 1;
				return Some(next_token);
			}
		}
		return None;
	}
	
	fn expression(&mut self) -> Option<CupidExpression> {
		/* Expression
			= Loop
			| Function
			| xBlock 
			| PropertyAssignment
			| xAssignment
			| xOperation 
			| xTerm
		*/
		
		if let Some(function) = self.function() {
			return Some(function);
		}
		if let Some(block) = self.block() {
			return Some(block);
		}
		if let Some(declaration) = self.declaration() {
			return Some(declaration);
		}
		if let Some(assignment) = self.assignment() {
			return Some(assignment);
		}
		if let Some(operation) = self.operation() {
			return Some(operation);
		}
		if let Some(term) = self.term() {
			return Some(term);
		}
		
		return None;
	}
	
	fn function(&mut self) -> Option<CupidExpression> {
		let pos = self.mark();
		let mut params: Vec<CupidSymbol> = vec![];
		loop {
			let mut unexpected_char = true;
			
			// parameters
			if let Some(param) = self.identifier() {
				unexpected_char = false;
				params.push(param);
			}
			// commas
			if let Some(_comma) = self.expect_one(vec![TokenType::Symbol(Symbol::Comma)]) {
				unexpected_char = false;
			}
			
			// block arrow
			if let Some(_arrow) = self.expect_one(vec![TokenType::Keyword(Keyword::Arrow)]) {
				if let Some(body) = self.expression() {
					return Some(CupidExpression::CupidFunction(CupidFunction { params, body: Box::new(body) }));	
				}
			}
			
			if unexpected_char {
				break;
			}
		}
		if let Some(_arrow) = self.expect_one(vec![TokenType::Keyword(Keyword::Arrow)]) {
			if let Some(exp) = self.expression() {
				return Some(CupidExpression::new_block(vec![exp]));
			}
		}
		self.reset(pos);
		return None;
	}
	
	fn block(&mut self) -> Option<CupidExpression> {
		if let Some(if_block) = self.if_block() {
			return Some(if_block);
		}
		if let Some(brace_block) = self.brace_block() {
			return Some(brace_block);
		}
		if let Some(arrow_block) = self.arrow_block() {
			return Some(arrow_block);
		}
		return None;
	}
	
	fn brace_block(&mut self) -> Option<CupidExpression> {
		let pos = self.mark();
		if let Some(_left_brace) = self.expect_one(vec![TokenType::Symbol(Symbol::LeftBrace)]) {
			let mut expressions = vec![];
			loop {
				if let Some(_right_brace) = self.expect_one(vec![TokenType::Symbol(Symbol::RightBrace)]) {
					break;
				}
				if let Some(expression) = self.expression() {
					expressions.push(expression);
				}
			}
			return Some(CupidExpression::new_block(expressions));
		}
		self.reset(pos);
		return None;
	}
	
	fn arrow_block(&mut self) -> Option<CupidExpression> {
		let pos = self.mark();
		if let Some(_arrow) = self.expect_one(vec![TokenType::Keyword(Keyword::Arrow)]) {
			if let Some(exp) = self.expression() {
				return Some(CupidExpression::new_block(vec![exp]));
			}
		}
		self.reset(pos);
		return None;
	}
	
	fn if_block(&mut self) -> Option<CupidExpression> {
		let pos = self.mark();
		if let Some(_if) = self.expect_one(vec![TokenType::Keyword(Keyword::If)]) {
			let condition = self.expression()?;
			let body = match self.block()? {
				CupidExpression::CupidBlock(b) => b,
				_ => {
					let mut error = Error::from_token(self.current(), "Expected a block after \"if\" condition");
					panic!("{}", error.to_string());
				}
			};
			
			let next_pos = self.mark();
			if let Some(_else) = self.expect_one(vec![TokenType::Keyword(Keyword::Else)]) {
				let else_body = match self.block()? {
					CupidExpression::CupidBlock(b) => b,
					_ => {
						let mut error = Error::from_token(self.current(), "Expected a block after \"else\" condition");
						panic!("{}", error.to_string());
					}
				};
				return Some(CupidExpression::new_if_block(condition, body, Some(else_body)));
			}
			self.reset(next_pos);
			return Some(CupidExpression::new_if_block(condition, body, None));
		}
		self.reset(pos);
		return None;
	}
	
	fn declaration(&mut self) -> Option<CupidExpression> {
		// ( let | const )  mut? identifier ( = expression )?
		let keywords = vec![
			TokenType::Keyword(Keyword::Let),
			TokenType::Keyword(Keyword::Const),
		];
		
		let pos = self.mark();
		
		if let Some(word) = self.expect_one(keywords) {
			
			let mutable = match word.token_type {
				TokenType::Keyword(Keyword::Let) => true,
				TokenType::Keyword(Keyword::Const) => false,
				_ => false
			};
			
			let mut deep_mutable = false;
			if let Some(_modifier) = self.expect_one(vec![TokenType::Keyword(Keyword::Mut)]) {
				deep_mutable = true;
			}
			
			if let Some(mut symbol) = self.identifier() {
				symbol.mutable = mutable;
				symbol.deep_mutable = deep_mutable;
				
				let next_pos = self.mark();
				
				if let Some(_assigner) = self.expect_one(vec![TokenType::Assign(Assign::Equal)]) {
					let right = self.expression()?;
					return Some(CupidExpression::new_declare(symbol, right));
				}
				
				self.reset(next_pos);
				return Some(CupidExpression::CupidSymbol(symbol));
			}
		}
		
		self.reset(pos);
		return None;
	}
	
	fn assignment(&mut self) -> Option<CupidExpression> {
		let assigners: Vec<TokenType> = vec![
			TokenType::Assign(Assign::Equal),
			TokenType::Assign(Assign::AddEqual),
			TokenType::Assign(Assign::SubtractEqual),
			TokenType::Assign(Assign::MultiplyEqual),
			TokenType::Assign(Assign::DivideEqual),
		];
		
		let pos = self.mark();
		if let Some(symbol) = self.identifier() {
			if let Some(assigner) = self.expect_one(assigners) {
				let assign = TokenType::get_assign(assigner.token_type);
				let right = self.expression()?;
				return Some(CupidExpression::new_assign(assign, symbol, right))
			}
		}
		self.reset(pos);
		return None;
	}
	
	fn operation(&mut self) -> Option<CupidExpression> {
		let operators: Vec<TokenType> = vec![
			TokenType::Operator(Operator::Add),
			TokenType::Operator(Operator::Subtract),
			TokenType::Operator(Operator::Multiply),
			TokenType::Operator(Operator::Divide),
			TokenType::Operator(Operator::Equal),
			TokenType::Operator(Operator::NotEqual),
			TokenType::Operator(Operator::Greater),
			TokenType::Operator(Operator::GreaterEqual),
			TokenType::Operator(Operator::Less),
			TokenType::Operator(Operator::LessEqual),
			TokenType::Operator(Operator::And),
			TokenType::Operator(Operator::Or),
		];
		
		let pos = self.mark();
		if let Some(term) = self.term() {
			if let Some(operator) = self.expect_one(operators) {
				let op = TokenType::get_op(operator.token_type);
				let right = self.expression()?;
				return Some(CupidExpression::new_operator(op, term, right));
			}
		}
		self.reset(pos);
		return None;
	}
	
	
	pub fn term(&mut self) -> Option<CupidExpression> {
		/* Term 
			= Group
			| AnonymousFunction
			| x FunctionCall
			| PropertyAccess
			| Dictionary
			| List
			| Tuple
			| Range
			| x atom
		*/
		
		if let Some(function_call) = self.function_call() {
			return Some(function_call);
		}
		
		if let Some(_eof) = self.expect_one(vec![TokenType::Eof]) {
			return None;
		}
		
		if let Some(identifier) = self.identifier() {
			return Some(CupidExpression::CupidSymbol(identifier));
		}
		
		return Some(self.atom());
	}
	
	fn function_call(&mut self) -> Option<CupidExpression> {
		let pos = self.mark();
		if let Some(identifier) = self.identifier() {
			if let Some(_lparen) = self.expect_one(vec![TokenType::Symbol(Symbol::LeftParen)]) {
				let mut args = vec![];
				loop {
					let mut unexpected_char = true;
					
					if let Some(_rparen) = self.expect_one(vec![TokenType::Symbol(Symbol::RightParen)]) {
						return Some(CupidExpression::CupidFunctionCall(CupidFunctionCall { fun: identifier, args }));
					}
					
					if let Some(arg) = self.term() {
						unexpected_char = false;
						args.push(arg);
					}
					
					if let Some(_comma) = self.expect_one(vec![TokenType::Symbol(Symbol::Comma)]) {
						unexpected_char = false;
					}
					
					if unexpected_char {
						break;
					}
				}
			}
		}
		self.reset(pos);
		return None;
	}
	
	fn atom(&mut self) -> CupidExpression {
		
		let atoms: Vec<TokenType> = vec![
			TokenType::Keyword(Keyword::True),
			TokenType::Keyword(Keyword::False),
			TokenType::Literal(Literal::String),
			TokenType::Literal(Literal::Decimal),
			TokenType::Literal(Literal::Number),
			TokenType::Eof,
		];
		
		let pos = self.mark();
		
		// parentheses
		if let Some(_token) = self.expect_one(vec![TokenType::Symbol(Symbol::LeftParen)]) {
			if let Some(exp) = self.expression() {
				if let Some(_paren) = self.expect_one(vec![TokenType::Symbol(Symbol::RightParen)]) {
					return exp;
				}
			}
		}
		
		// negative numbers
		if let Some(_token) = self.expect_one(vec![TokenType::Operator(Operator::Subtract)]) {
			if let Some(exp) = self.expression() {
				return CupidExpression::new_operator(
					Operator::Subtract, 
					CupidExpression::CupidEmpty, 
					exp
				);
			}
		}
		
		self.reset(pos);
		
		if let Some(token) = self.expect_one(atoms) {
			match token.token_type {
				TokenType::Keyword(Keyword::True) 
				| TokenType::Keyword(Keyword::False)
				| TokenType::Literal(Literal::String)
				| TokenType::Literal(Literal::Decimal)
				| TokenType::Literal(Literal::Number) => return CupidExpression::from_token(&token),
				_ => panic!("No atom found at token {}", token)
			};
		}
		
		panic!("No atom found at token {}", &self.tokens[self.index]);
	}
	
	fn identifier(&mut self) -> Option<CupidSymbol> {
		if let Some(identifier) = self.expect_one(vec![TokenType::Identifier]) {
			return Some(CupidSymbol { 
				identifier: CupidValue::String(identifier.source.clone()), 
				mutable: true, 
				deep_mutable: true
			});
		}
		return None;
	}
}
