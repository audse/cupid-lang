#![allow(clippy::all)]
use super::*;

#[allow(dead_code)]
pub fn test(input: &str, expected: Value) -> bool {
	let mut parser = CupidParser::new(input.to_string());
	let parse_tree = parser._file(None);
	let semantics = to_tree(&parse_tree.unwrap().0);	
	let mut scope = LexicalScope::new();
	use_builtin_types(&mut scope);
	let result = semantics.resolve(&mut scope);
	println!("Result: {}", result);
	
	result.is_equal(&expected)
}

#[allow(dead_code)]
pub fn test_int(input: &str, expected: i32) -> bool {
	test(input, Value::Integer(expected))
}

#[allow(dead_code)]
fn test_dec(input: &str, expected: f64) -> bool {
	test(input, float_to_dec(expected))
}

#[allow(dead_code)]
fn test_str(input: &str, expected: &str) -> bool {
	test(input, Value::String(String::from(expected)))
}

#[allow(dead_code)]
fn test_boo(input: &str, expected: bool) -> bool {
	test(input, Value::Boolean(expected))
}

#[allow(dead_code)]
pub fn test_none(input: &str) -> bool {
	test(input, Value::None)
}

#[test]
fn test_assignment() {
	assert!(test_int("int x = 1", 1));
	assert!(test_int("int x = 10 * 10", 100));
	assert!(test_int("int x = (10 + 1) * 2", 22));
	assert!(test_int("int x = 1", 1));
	assert!(test_int("int y = 2", 2));
	assert!(test_str("string mut z = 'abc'", "abc"));
	assert!(test_boo("bool x = true", true));
	assert!(test_int("int x = 1", 1));
	assert!(test_str("string x = 'abc'", "abc"));
	assert!(test_dec("dec x = -1.5", -1.5));
}

/*
Blocks
*/
 
#[test]
fn test_arrow_block() {
	assert!(test_int("{
		fun div = a, b => a / b
		int c = div(10, 10)
	}", 1));
}

#[test]
fn test_brace_block() {
	assert!(test_int("{ 
		int mut x = 1 
		x = x + 1 
		x * 10 
	}", 20));
	assert!(test_str("{ 
		string x = 'abc'
		string y = 'xyz'
		string z = x + y 
	}", "abcxyz"));
}

#[test]
fn test_if_block() {
	assert!(test_int("if true is true { 10 }", 10));
	assert!(test_none("if true not true { 10 }"));
	assert!(test_int("if true not true { 10 } else { 5 }", 5));
	assert!(test_int("if 2 > 1 => 2 else => 1", 2));
}

#[test]
fn test_expression() {
	assert!(test_none("let x"));
	assert!(test_int("-1", -1));
}

#[test]
fn test_function() {
	assert!(test_int("{
		fun x = a => a + 1 
		x(100)
	}", 101));
}

#[test]
fn test_loop() {
	assert!(test_int("{
		int i = 10
		while i > 0 => i = i - 1
		i
	}", 0))
}

#[test]
fn test_operation() {
	assert!(test_str("'abc' + 'xyz'", "abcxyz"));
	assert!(test_dec("1.5 + 2.5 * 2.0", 6.5));
	assert!(test_dec("(1.5 + 2.5) * 2.0", 8.0));
}

#[test]
fn test_structure() {}

#[test]
fn test_typing() {}