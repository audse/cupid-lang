use crate::*;
use super::attributes;

impl CreateAST for Val {
	fn create_ast(node: &mut ParseNode, _scope: &mut Env) -> Result<Self, ErrCode> {
		let mut tokens = node.tokens.to_owned();
		Ok(match &*node.name {
			"boolean" => boolean(tokens.remove(0)),
			"none" => Val::None,
			"char" => char(tokens),
			"string" => string(tokens.remove(0)),
			"decimal" => decimal(tokens),
			"number" => integer(tokens.remove(0)),
			_ => panic!("could not parse value: {node:?}")
		})
	}
}

impl CreateAST for Value {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		let attributes = attributes(node, scope)?;
		Ok(Value {
			val: Untyped(Val::create_ast(node, scope)?), 
			attributes
		})
	}
}

fn boolean(token: Token) -> Val {
	match &*token.source {
		"true" => Val::Boolean(true),
		"false" => Val::Boolean(false),
		_ => panic!("expected boolean")
	}
}

fn char(tokens: Vec<Token>) -> Val {
	let get_char = |token: &Token| token.source.chars().next().unwrap();	
	if tokens.len() == 2 {
		Val::Char(get_char(&tokens[1]))
	} else {
		let chars = [get_char(&tokens[1]), get_char(&tokens[2])];
		let c = match format!("{}{}", chars[0], chars[1]).as_str() {
			r"\n" => '\n',
			r"\t" => '\t',
			r"\r" => '\r',
			r"\s" => ' ',
			c => panic!("not char: {c}")
		};
		Val::Char(c)
	}
}

fn string(token: Token) -> Val {
	let mut string = token.source.to_owned();
	if let Some(first) = string.chars().next() {
		if is_quote(first) {
			string = Cow::Owned(string[1..string.len()-1].to_string());
		}
	}
	Val::String(string)
}

fn decimal(tokens: Vec<Token>) -> Val {
	Val::Decimal(
		tokens[0].source.parse::<i32>().unwrap(),
		tokens[1].source.parse::<u32>().unwrap(),
	)
}

pub fn integer(token: Token) -> Val {
	Val::Integer(token.source.parse::<i32>().unwrap())
}


fn is_quote(c: char) -> bool {
	c == '"' || c =='\''
}