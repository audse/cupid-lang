use crate::*;

pub trait IntoAST {
	fn into_ast(parser: &mut impl Parser, scope: &mut Env) -> Option<Self> where Self: Sized;
}

macro_rules! parse_alt {
	($parser:tt $(, $tokens:ident)? => $b:block) => {{
		let pos = $parser.tokens().index();
		$b
		$parser.tokens().goto(pos);
	}}
}

pub fn attr(tokens: Vec<Token>, scope: &mut Env) -> Attributes {
	scope.token_data.push(tokens);
	let generics = vec![];
	Attributes::build()
		.source(Some(scope.token_data.len() - 1))
		.generics(GenericList(generics))
		.build()
}

impl IntoAST for Value {
	fn into_ast(parser: &mut impl Parser, scope: &mut Env) -> Option<Self> {
		parse_alt!(parser => {
			if let Some(val_token) = parser.expect_parse_number() {
				let int = val_token.source().parse::<i32>().unwrap();
				let attr = attr(vec![val_token], scope);
				return Some(VInteger(int, attr))
			}
		});
		None
	}
}