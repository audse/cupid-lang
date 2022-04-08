use std::any::Any;
use super::*;

#[allow(dead_code)]
pub fn test(input: &str, expected: Box<dyn Any>) -> bool {
	let mut scope = CupidScope::new(None);
	
	// let mut lexer = Lexer::new(String::from(input), false);
	// lexer.scan();
	// 
	// let mut parser = Parser::new(lexer);
	// 
	// let mut result = CupidValue::None;
	// let block = parser.parse();
	// 
	// for exp in block {
	//    result = exp.resolve(&mut scope);
	// }
	// println!("{}", scope);
	// return result.is_equal(expected);
	return false;
}

#[allow(dead_code)]
pub fn test_int(input: &str, expected: i32) -> bool {
	return test(input, Box::new(expected));
}

#[allow(dead_code)]
fn test_dec(input: &str, expected: f64) -> bool {
	return test(input, Box::new(expected));
}

#[allow(dead_code)]
fn test_str(input: &str, expected: &str) -> bool {
	return test(input, Box::new(String::from(expected)));
}

#[allow(dead_code)]
fn test_boo(input: &str, expected: bool) -> bool {
	return test(input, Box::new(expected));
}

#[allow(dead_code)]
pub fn test_none(input: &str) -> bool {
	let none: Option<bool> = None;
	return test(input, Box::new(none));
}

#[test]
fn test_assignment() {
	assert!(test_int("x = 1", 1));
	assert!(test_int("x = 10 * 10", 100));
	assert!(test_int("x = (10 + 1) * 2", 22));
	assert!(test_int("let x = 1", 1));
	assert!(test_int("const y = 2", 2));
	assert!(test_str("const mut z = 'abc'", "abc"));
}

#[test]
fn test_arrow_block() {
	assert!(test_int("x = => 10 / 10  x()", 1));
}

#[test]
fn test_brace_block() {
	assert!(test_int("{ x = 1 x = x + 1 x * 10 }", 20));
	assert!(test_str("{ x = \"abc\" y = \"xyz\" z = x + y }", "abcxyz"));
}

#[test]
fn test_if_block() {
	assert!(test_int("if true == true { 10 }", 10));
	assert!(test_none("if true != true { 10 }"));
	assert!(test_int("if true != true { 10 } else { 5 }", 5));
	assert!(test_int("if 2 > 1 => 2 else => 1", 2));
}

#[test]
fn test_function() {
	assert!(test_int("x = a => a + 1   x(100)", 101));
}

#[test]
fn test_expression() {
	assert!(test_none("x"));
	assert!(test_int("-1", -1));
}

#[test]
fn test_operation() {
	assert!(test_str("'abc' + 'xyz'", "abcxyz"));
	assert!(test_dec("1.5 + 2.5 * 2.0", 6.5));
	assert!(test_dec("(1.5 + 2.5) * 2.0", 8.0));
}