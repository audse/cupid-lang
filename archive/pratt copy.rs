
fn binary_op(&mut self, min_binding_pow: u8) -> S {
	if let Some(next_token) = self.advance() {
		let mut left_node = to_literal(&next_token);
		let mut left = match &next_token.token_type {
			// literals
			TokenType::Literal(Literal::String) => Some(CupidNode { value: CupidValue::String(token.source.clone()) }),
			TokenType::Keyword(Keyword::True) => Some(CupidNode { value: CupidValue::Boolean(true) }),
			TokenType::Keyword(Keyword::False) => Some(CupidNode { value: CupidValue::Boolean(false) }),
			TokenType::Keyword(Keyword::None) => Some(CupidNode { value: CupidValue::None }),
			
			// operators
			TokenType::Operator(operator) => {
				let ((), right_binding_pow) = self.prefix_binding_pow(operator);
				let right = self.binary_op(right_binding_pow);
				// return ? S::Cons in original
			},
			token => panic!("Unexpected token: {?:}", token)
		};
	}

	loop {
		if let Some(operator) = self.peek() {
			let op_token = match operator.token_type {
				TokenType::Eof => break,
				TokenType::Operator(op) => op,
				token => panic!("Unexpected non-operator token: {?:}", token)
			};
			
			let (left_binding_pow, right_binding_pow) = self.infix_binding_pow(op_token);
			if left_binding_pow < min_binding_pow { break; }
			
			self.advance();
			let right = self.binary_op(right_binding_pow);
			left = CupidBinaryOp(operator, left, right);
			
			continue;
		}
		break;
	};
	return left;
}

fn prefix_binding_pow(operator: Token) -> ((), u8) {
	match operator.token_type {
		TokenType::Operator(Operator::Add) | TokenType::Operator(Operator::Subtract) => ((), 5),
		token => panic!("Operator supplied is not a prefix: {?:}", token)
	}
}

fn infix_binding_pow(operator: Token) -> (u8, u8) {
	match operator.token_type {
		TokenType::Operator(Operator::Add) | TokenType::Operator(Operator::Subtract) => (1, 2),
		TokenType::Operator(Operator::Multiply) | TokenType::Operator(Operator::Divide) => (3, 4),
		token => panic!("Operator supplied is invalid: {?:}", token)
	}
}