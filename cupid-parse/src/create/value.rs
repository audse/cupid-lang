use crate::*;
use super::attributes;

impl CreateAST for Value {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		let attributes = attributes(node, scope)?;
		Ok(match &*node.name {
			"char" => VChar(char(node.tokens.to_owned()), attributes),
			"string" => VString(string(node.token(0)), attributes),
			"decimal" => {
				let dec = decimal(node.tokens.to_owned());
				VDecimal(dec.0, dec.1, attributes)
			},
			"number" => VInteger(integer(node.token(0)), attributes),
			_ => panic!("could not parse value: {node:?}")
		})
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

fn string(token: Token) -> Str {
	let mut string = token.source.to_owned();
	if let Some(first) = string.chars().next() {
		if is_quote(first) {
			string = Cow::Owned(string[1..string.len()-1].to_string());
		}
	}
	string
}

fn decimal(tokens: Vec<Token>) -> (i32, u32) {
	(
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