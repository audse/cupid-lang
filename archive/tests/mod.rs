// #![allow(clippy::all)]
// use super::*;
// 
// #[allow(dead_code)]
// pub fn test(input: &str, expected: Value) -> bool {
// 	let mut handler = FileHandler::from(input);
// 	let result = handler.run_and_return().pop().unwrap();
// 	println!("Result: {}", result);
// 	result == expected
// }
// 
// #[allow(dead_code)]
// pub fn test_error(input: &str) -> bool {
// 	let mut handler = FileHandler::from(input);
// 	let result = handler.run_and_return().pop().unwrap();
// 	if let Value::Error(_) = result {
// 		true
// 	} else {
// 		false
// 	}
// }
// 
// #[allow(dead_code)]
// pub fn test_int(input: &str, expected: i32) -> bool {
// 	test(input, Value::Integer(expected))
// }
// 
// #[allow(dead_code)]
// fn test_dec(input: &str, expected: f64) -> bool {
// 	test(input, float_to_dec(expected))
// }
// 
// #[allow(dead_code)]
// fn test_char(input: &str, expected: char) -> bool {
// 	test(input, Value::Char(expected))
// }
// 
// #[allow(dead_code)]
// fn test_str(input: &str, expected: &str) -> bool {
// 	test(input, Value::String(String::from(expected)))
// }
// 
// #[allow(dead_code)]
// fn test_boo(input: &str, expected: bool) -> bool {
// 	test(input, Value::Boolean(expected))
// }
// 
// #[allow(dead_code)]
// pub fn test_none(input: &str) -> bool {
// 	test(input, Value::None)
// }
// 
// #[test]
// fn test_array() {
// 	assert!(test_int("
// 		array [int] x = 10, 20, 30
// 		x.0
// 	", 10));
// 	assert!(test_int("
// 		array [int] mut x = 10, 20, 30
// 		x.0++
// 		x.0
// 	", 11));
// 	assert!(test_int("
// 		array [int] mut x = 10, 20, 30
// 		x.0 += 100
// 		x.0
// 	", 110));
// 	assert!(test_int("
// 		array [int] mut x = 10, 20, 30
// 		x.0 = 1000
// 		x.0
// 	", 1000));
// }
// 
// #[test]
// fn test_assignment() {
// 	assert!(test_int("int x = 1", 1));
// 	assert!(test_int("int x = 10 * 10", 100));
// 	assert!(test_int("int x = (10 + 1) * 2", 22));
// 	assert!(test_int("int x = 1", 1));
// 	assert!(test_int("int y = 2", 2));
// 	assert!(test_str("string mut z = 'abc'", "abc"));
// 	assert!(test_boo("bool x = true", true));
// 	assert!(test_int("int x = 1", 1));
// 	assert!(test_str("string x = 'abc'", "abc"));
// 	assert!(test_dec("dec x = -1.5", -1.5));
// 	assert!(test_error("
// 		# throws immutable error
// 		int x = 1
// 		x = 100
// 	"));
// 	assert!(test_error("
// 		# throws type mismatch error
// 		int mut x = 1
// 		x = 'abc'
// 	"));
// 	assert!(test_error("
// 		# throws undefined error
// 		x = 1
// 	"));
// }
// 
// #[test]
// fn test_operator_assignment() {
// 	assert!(test_int("
// 		int mut x = 0
// 		x += 1
// 	", 1));
// 	assert!(test_int("
// 		int mut x = 0
// 		x -= 1
// 	", -1));
// 	assert!(test_int("
// 		int mut x = 0
// 		x *= 1
// 	", 0));
// 	assert!(test_int("
// 		int mut x = 0
// 		x++
// 	", 1));
// 	assert!(test_int("
// 		int mut x = 0
// 		x --
// 	", -1));
// }
// 
// /*
// Blocks
// */
//  
// #[test]
// fn test_arrow_block() {
// 	assert!(test_int("{
// 		fun div = int a, int b => a / b
// 		int c = div(10, 10)
// 	}", 1));
// }
// 
// #[test]
// fn test_brace_block() {
// 	assert!(test_int("{ 
// 		int mut x = 1 
// 		x = x + 1 
// 		x * 10 
// 	}", 20));
// 	assert!(test_str("{ 
// 		string x = 'abc'
// 		string y = 'xyz'
// 		string z = x + y 
// 	}", "abcxyz"));
// }
// 
// #[test]
// fn test_box_block() {
// 	assert!(test_error("
// 		int x = 1
// 		box { x * 10 }
// 	"));
// 	// assert!(test_int("
// 	// 	int z = 1000
// 	// 	int y = box { 
// 	// 		int z = 10
// 	// 		int x = 1
// 	// 		x * z 
// 	// 	}
// 	// 	y
// 	// ", 10));
// }
// 
// #[test]
// fn test_if_block() {
// 	assert!(test_int("if true is true { 10 }", 10));
// 	assert!(test_none("if true not true { 10 }"));
// 	assert!(test_int("if true not true { 10 } else { 5 }", 5));
// 	assert!(test_int("if 2 > 1 => 2 else => 1", 2));
// 	assert!(test_str("
// 		int my_num = 8
// 		if my_num is 1 => '1'
// 		else if my_num < 10 => 'between 1 and 10'
// 		else => '10+'
// 	", "between 1 and 10"));
// 	assert!(test_str("
// 		int my_num = 8
// 		if my_num is 1 => '1'
// 		else if my_num is 2 => '2'
// 		else if my_num not 3 => 'not 3'
// 		else => 'something else'
// 	", "not 3"));
// }
// 
// #[test]
// fn test_function() {
// 	assert!(test_int("{
// 		fun x = int a => a + 1 
// 		x(100)
// 	}", 101));
// 	assert!(test_int("{
// 		fun pythagorean = int a, int b => a * a + b * b
// 		pythagorean(2, 3)
// 	}", 13));
// 	assert!(test_error("{
// 		# throws wrong num of arguments error
// 		fun x = int a => a + 1 
// 		x(100, 100)
// 	}"));
// 	assert!(test_error("{
// 		# throws wrong num of arguments error
// 		fun x = int a => a + 1 
// 		x()
// 	}"));
// 	assert!(test_int("
// 		fun square = int a {
// 			if a is 100 => return 0
// 			a * a
// 		}
// 		square (10)
// 	", 100));
// 	assert!(test_int("
// 		fun square = int a {
// 			if a is 100 => return 0
// 			a * a
// 		}
// 		square (100)
// 	", 0));
// }
// 
// #[test]
// fn test_loop() {
// 	assert!(test_int("{
// 		int mut i = 10
// 		while i > 0 => i = i - 1
// 		i
// 	}", 0));
// 	assert!(test_str("
// 		string mut abc = ''
// 		for letter in ['a', 'b', 'c'] => abc = abc + letter
// 		abc
// 	", "abc"));
// 	assert!(test_int("
// 		int mut i = 10
// 		while i > 0 {
// 			i = i - 1
// 			if i is 5 {
// 				i = 100
// 				break i
// 			}
// 		}
// 		i
// 	", 100));
// 	assert!(test_int("
// 		array [int] nums = 1, 2, 3
// 		int n = for num in nums {
// 			if num > 2 => break num
// 			num
// 		}
// 		n
// 	", 3));
// }
// 
// #[test]
// fn test_operation() {
// 	assert!(test_str("'abc' + 'xyz'", "abcxyz"));
// 	assert!(test_dec("1.5 + 2.5 * 2.0", 6.5));
// 	assert!(test_dec("(1.5 + 2.5) * 2.0", 8.0));
// 	assert!(test_int("1 + 10 * 10 / 10", 11));
// 	assert!(test_int("-1", -1));
// 	assert!(test_int("2 ^ 2", 4));
// 	assert!(test_int("2 % 2", 0));
// 	assert!(test_int("2 and 3", 3));
// 	assert!(test_int("-2 or 2", 2));
// }
// 
// #[test]
// fn test_typing() {
// 	assert!(test_int("
// 		type int_alias = int
// 		int_alias my_int = -10
// 	", -10));
// 	assert!(test_int("
// 		type int_list = array [int]
// 		int_list my_list = 0, 1, 2
// 		my_list.0
// 	", 0));
// 	assert!(test_char(r"
// 		type char_list = array [char]
// 		char_list my_list = \a, \b, \c
// 		my_list.0
// 	", 'a'));
// 	assert!(test_error(r"
// 		type char_list = array [char]
// 		char_list mut my_list = \a, \b, \c
// 		my_list = 1, 2, 3
// 	"));
// 	assert!(test_char(r"
// 		type char_list = array [array [char]]
// 		array [char] a = \a, \b
// 		array [char] b = \x, \y
// 		char_list z = a, b
// 		array [char] first = z.0
// 		first.0
// 	", 'a'))
// }