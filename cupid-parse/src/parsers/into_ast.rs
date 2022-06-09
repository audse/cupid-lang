use crate::*;

#[allow(unused_variables)]
pub trait IntoAST {
	fn into_ast(parser: &mut impl Parser, scope: &mut Env) -> Option<Self> where Self: Sized {
		None
	}
}

macro_rules! parse_alt {
	($parser:tt $(, $tokens:ident)? => $b:block) => {{
		let pos = $parser.tokens().index();
		$( let $tokens = vec![]; )?
		$b
		$parser.tokens().goto(pos);
	}}
}

pub fn attr(parser: &mut impl Parser, scope: &mut Env, tokens: Vec<Token>) -> Attributes {
	scope.token_data.push(tokens);
	Attributes::build()
		.source(Some(scope.token_data.len() - 1))
		.generics(GenericList::into_ast(parser, scope).unwrap_or_default())
		.build()
}

impl IntoAST for GenericList {}

impl IntoAST for Ident {
	fn into_ast(parser: &mut impl Parser, scope: &mut Env) -> Option<Self> {
		parse_alt!(parser => {
			if let Some(ident_token) = parser.expect_parse_word() {
				return Some(Self::build()
					.name(ident_token.source.to_owned())
					.attributes(attr(parser, scope, vec![ident_token]))
					.build())
			}
		});
		None
	}
}

impl IntoAST for Value {
	fn into_ast(parser: &mut impl Parser, scope: &mut Env) -> Option<Self> {
		parse_alt!(parser => {
			if let Some(int_token) = parser.expect_parse_number() {
				let int = int_token.source().parse::<i32>().unwrap();
				if let Some(_) = parser.expect(".") {
					if let Some(dec_token) = parser.expect_parse_number() {
						let dec = dec_token.source().parse::<u32>().unwrap();
						return Some(VDecimal(int, dec, attr(parser, scope, vec![int_token, dec_token])));
					}
				}
				return Some(VInteger(int, attr(parser, scope, vec![int_token])))
			}
			// if let Some(char_token) = parser.expect_parse_letter() {
			// }
		});
		None
	}
}