use crate::*;

pub trait IntoAST {
	fn into_ast(parser: &mut impl Parser) -> Option<Self> where Self: Sized;
}

build_struct! {
	#[derive(Debug, Clone, Default)]
	pub ValueBuilder => pub Value <T: Default + Clone> {
		pub val: T,
		pub tokens: Vec<Token>,
	}
}

impl<T: Default + Clone> ValueBuilder<T> {
	fn token(self, t: Token) -> Self {
		self.tokens(vec![t])
	}
}

macro_rules! parse_alt {
	($parser:tt => $b:block) => {{
		let pos = $parser.tokens().index();
		$b
		$parser.tokens().goto(pos);
	}}
}

impl IntoAST for Value<i32> {
	fn into_ast(parser: &mut impl Parser) -> Option<Self> {
		let value = Value::build();
		parse_alt!(parser => {
			if let Some(val_token) = parser.expect_parse_number() {
				return Some(value
					.val(val_token.source().parse::<i32>().unwrap())
					.token(val_token)
					.build());
			}
		});
		None
	}
}

impl IntoAST for Value<String> {
	fn into_ast(parser: &mut impl Parser) -> Option<Self> {
		todo!()
	}
}