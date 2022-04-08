use crate::{
	Lexer,
	Parser,
	CupidValue,
	CupidScope,
};

fn test(input: &str, expected: Box<dyn Any>) -> bool {
	let mut scope = CupidScope::new(None);
	
	let mut lexer = Lexer::new(contents, false);
	lexer.scan();
	
	let mut parser = Parser::new(lexer);
	
	let mut result = CupidValue::None;
	let block = parser.parse();
	
	for exp in block {
	   result = exp.resolve(&mut scope);
	}
	
	return result.is_equal(expected);
}