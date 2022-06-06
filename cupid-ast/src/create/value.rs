use crate::*;
use super::attributes;

// #[allow(unused_variables)]
// impl CreateAST for Val {
// 	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
// 		let mut tokens = node.tokens.to_owned();
// 		Ok(match &*node.name {
// 			"boolean" => boolean(tokens.remove(0)),
// 			"none" => Val::None,
// 			"char" => char(tokens),
// 			"string" => string(tokens.remove(0)),
// 			"decimal" => decimal(tokens),
// 			"number" => integer(tokens.remove(0)),
// 			_ => panic!("could not parse value: {node:?}")
// 		})
// 	}
// }

impl<T: Default> CreateAST for Value<T> {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		let attributes = attributes(node, scope)?;
		let mut tokens = node.tokens.to_owned();
		let value: T = match &*node.name {
			"boolean" => boolean(tokens.remove(0)),
			"none" => Nothing,
			"char" => char(tokens),
			"string" => string(tokens.remove(0)),
			"decimal" => decimal(tokens),
			"number" => integer(tokens.remove(0)),
			_ => panic!("could not parse value: {node:?}")
		};
		Ok(Value {
			value: Untyped(value), 
			attributes
		})
	}
}

fn boolean(token: Token) -> bool {
	match &*token.source {
		"true" => true,
		"false" => false,
		_ => panic!("expected boolean")
	}
}

fn char(tokens: Vec<Token>) -> char {
	let get_char = |token: &Token| token.source.chars().next().unwrap();	
	if tokens.len() == 2 {
		get_char(&tokens[1])
	} else {
		let chars = [get_char(&tokens[1]), get_char(&tokens[2])];
		let c = match format!("{}{}", chars[0], chars[1]).as_str() {
			r"\n" => '\n',
			r"\t" => '\t',
			r"\r" => '\r',
			r"\s" => ' ',
			c => panic!("not char: {c}")
		};
		c
	}
}

fn string(token: Token) -> String {
	let mut string = token.source.to_owned();
	if let Some(first) = string.chars().next() {
		if is_quote(first) {
			string = Cow::Owned(string[1..string.len()-1].to_string());
		}
	}
	string
}

fn decimal(tokens: Vec<Token>) -> Decimal {
	Decimal(
		tokens[0].source.parse::<i32>().unwrap(),
		tokens[1].source.parse::<u32>().unwrap(),
	)
}

pub fn integer(token: Token) -> i32 {
	token.source.parse::<i32>().unwrap()
}


fn is_quote(c: char) -> bool {
	c == '"' || c =='\''
}