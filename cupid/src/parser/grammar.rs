// #![allow(clippy::all)]
// use crate::{is_identifier, is_number, is_string, is_uppercase, is_tag, BiDirectionalIterator, Tokenizer};
// use std::collections::HashMap;
// use std::hash::Hash;
// 
// macro_rules! use_item {
//     ($item:expr, $method:expr, $is_concealed:expr) => {{
//         if let Some((mut val, pass_through)) = $method {
//             if pass_through && !$is_concealed {
//                 // move returned node's children to current node
//                 $item.tokens.append(&mut val.tokens);
//                 $item.children.append(&mut val.children);
//             } else if !$is_concealed {
//                 $item.children.push(val);
//             }
//         } else {
//             break;
//         }
//     }};
// }
// 
// macro_rules! use_optional {
//     ($item:expr, $method:expr, $is_concealed:expr) => {{
//         if let Some((mut val, pass_through)) = $method {
//             if pass_through && !$is_concealed {
//                 $item.tokens.append(&mut val.tokens);
//                 $item.children.append(&mut val.children);
//             } else if !$is_concealed {
//                 $item.children.push(val);
//             }
//         }
//     }};
// }
// 
// macro_rules! use_repeat {
//     ($item:expr, $method:expr, $is_concealed:expr) => {{
//         while let Some((mut val, pass_through)) = $method {
//             if pass_through && !$is_concealed {
//                 $item.tokens.append(&mut val.tokens);
//                 $item.children.append(&mut val.children);
//             } else if !$is_concealed {
//                 $item.children.push(val);
//             }
//         }
//     }};
// }
// 
// macro_rules! use_negative_lookahead {
//     ($parser:expr, $item:ident, $method:expr) => {{
//         let pos = $parser.tokens.index();
//         if let Some((mut val, pass_through)) = $method {
//             $parser.tokens.goto(pos);
//             break;
//         }
//     }};
// }
// 
// macro_rules! use_positive_lookahead {
//     ($parser:expr, $item:ident, $method:expr) => {{
//         let pos = $parser.tokens.index();
//         if let Some((mut val, pass_through)) = $method {
//         } else {
//             $parser.tokens.goto(pos);
//             break;
//         }
//     }};
// }
// 
// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
// pub struct Node {
//     pub name: String,
//     pub children: Vec<Node>,
//     pub tokens: Vec<String>,
// }
// 
// #[derive(PartialEq, Eq)]
// pub struct Parser {
//     pub tokens: BiDirectionalIterator<String>,
//     pub index: usize,
//     pub memos: Memos,
// }
// 
// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
// pub struct ParseResult(Option<(Node, bool)>, usize);
// 
// #[derive(PartialEq, Eq)]
// pub struct Memos {
//     memos: HashMap<(usize, String), ParseResult>
// }
// 
// impl Memos {
//     fn has_memo(&self, pos: usize, func: String) -> bool {
//         self.memos.contains_key(&(pos, func))
//     }
//     fn get_memo(&mut self, pos: usize, func: String) -> Option<&ParseResult> {
//         self.memos.get(&(pos, func))
//     }
//     fn add_memo(&mut self, pos: usize, func: String, result: ParseResult) -> ParseResult {
//         self.memos.insert((pos, func.clone()), result);
//         self.memos.get(&(pos, func)).unwrap().clone()
//     }
// }
// 
// 
// impl Parser {
//     pub fn new(source: String) -> Self {
//         let mut tokenizer = Tokenizer::new(source.as_str());
//         tokenizer.scan();
//         println!("{:?}", tokenizer.tokens);
//         Self {
//             index: 0,
//             tokens: BiDirectionalIterator::new(tokenizer.tokens),
//             memos: Memos {
//                 memos: HashMap::new(),
//             },
//         }
//     }
//     fn expect(&mut self, rule_name: String) -> Option<(Node, bool)> {
//         if let Some(memo) = self.memoize("expect".to_string(), Some(rule_name.clone())) {
//             return Some(memo);
//         }
//         if let Some(token) = self.tokens.expect(rule_name.clone()) {
//             return Some((node_from_token(token, rule_name), true));
//         }
//         None
//     }
//     fn expect_one(&mut self, rule_names: Vec<String>) -> Option<(Node, bool)> {
//         if let Some(memo) =
//             self.memoize("expect_one".to_string(), Some(format!("{:?}", rule_names)))
//         {
//             return Some(memo);
//         }
//         for rule_name in rule_names {
//             if let Some(next) = self.expect(rule_name) {
//                 return Some(next);
//             }
//         }
//         None
//     }
//     fn expect_constant(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
//         if let Some(memo) = self.memoize("expect_constant".to_string(), None) {
//             return Some(memo);
//         }
//         if let Some(next) = self.tokens.peek(0) {
//             if is_uppercase(next) {
//                 let token = self.tokens.next().unwrap();
//                 return Some((node_from_token(token, "constant".to_string()), true));
//             }
//         }
//         None
//     }
//     fn expect_word(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
//         if let Some(memo) = self.memoize("expect_word".to_string(), None) {
//             return Some(memo);
//         }
//         if let Some(next) = self.tokens.peek(0) {
//             if is_identifier(next) {
//                 let token = self.tokens.next().unwrap();
//                 return Some((node_from_token(token, "word".to_string()), true));
//             }
//         }
//         None
//     }
//     fn expect_string(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
//         if let Some(memo) = self.memoize("expect_string".to_string(), None) {
//             return Some(memo);
//         }
//         if let Some(next) = self.tokens.peek(0) {
//             if is_string(next) {
//                 let token = self.tokens.next().unwrap();
//                 return Some((node_from_token(token, "string".to_string()), true));
//             }
//         }
//         None
//     }
//     fn expect_number(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
//         if let Some(memo) = self.memoize("expect_number".to_string(), None) {
//             return Some(memo);
//         }
//         if let Some(next) = self.tokens.peek(0) {
//             if is_number(next) {
//                 let token = self.tokens.next().unwrap();
//                 return Some((node_from_token(token, "number".to_string()), true));
//             }
//         }
//         None
//     }
//     fn expect_tag(&mut self, arg: String) -> Option<(Node, bool)> {
//         if let Some(memo) = self.memoize("expect_tag".to_string(), Some(arg)) {
//             return Some(memo);
//         }
//         if let Some(next) = self.tokens.peek(0) {
//             if is_tag(next) {
//                 let token = self.tokens.next().unwrap();
//                 return Some((node_from_token(token, "tag".to_string()), true));
//             }
//         }
//         None
//     }
//     fn reset_parse(&mut self, item: &mut Node, pos: usize) {
//         item.tokens.clear();
//         item.children.clear();
//         self.tokens.goto(pos);
//     }
//     fn memoize(&mut self, func: String, arg: Option<String>) -> Option<(Node, bool)> {
//         let pos = self.tokens.index();
//         let key = format!("{}({:?})", func, arg);
//         if self.memos.has_memo(pos, key.clone()) {
//             let memo = self.memos.get_memo(pos, key).unwrap();
//             self.tokens.goto(memo.1);
//             return memo.0.clone();
//         } else {
//             return None;
//         }
//     }
//     fn make_memo(&mut self, pos: usize, func: String, result: Option<(Node, bool)>) -> Option<(Node, bool)> {
//         let end_pos = self.tokens.index();
//         let next_result = ParseResult(result, end_pos);
//         self.memos.add_memo(pos, func, next_result).0
//     }
//     
//     
// 		pub fn _grammar(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
// 			if let Some(memo) = self.memoize("grammar".to_string(), None) {
// 				return Some(memo);
// 			}
// 			let start_pos = self.tokens.index();
// 			let pos = start_pos;
// 			let mut node = Node {
// 				name: "grammar".to_string(),
// 				tokens: vec![],
// 				children: vec![],
// 			};
// 			loop { 
// use_repeat!(&mut node, self._rule(None), false);
// 
// 			let result = Some((node, false));
// 			return self.make_memo(start_pos, "grammar".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// 			self.make_memo(start_pos, "grammar".to_string(), None);
// 			None
// 		}
// 		
// 		pub fn _rule(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
// 			if let Some(memo) = self.memoize("rule".to_string(), None) {
// 				return Some(memo);
// 			}
// 			let start_pos = self.tokens.index();
// 			let pos = start_pos;
// 			let mut node = Node {
// 				name: "rule".to_string(),
// 				tokens: vec![],
// 				children: vec![],
// 			};
// 			loop { 
// use_item!(&mut node, self.expect("-".to_string()), true);
// use_item!(&mut node, self.expect_word(None), false);
// use_item!(&mut node, self.expect(":".to_string()), true);
// use_item!(&mut node, self._alt(None), false);
// use_repeat!(&mut node, self._alt(None), false);
// use_repeat!(&mut node, self._alt_alt(None), false);
// 
// 			let result = Some((node, false));
// 			return self.make_memo(start_pos, "rule".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// 			self.make_memo(start_pos, "rule".to_string(), None);
// 			None
// 		}
// 		
// 		pub fn _alt(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
// 			if let Some(memo) = self.memoize("alt".to_string(), None) {
// 				return Some(memo);
// 			}
// 			let start_pos = self.tokens.index();
// 			let pos = start_pos;
// 			let mut node = Node {
// 				name: "alt".to_string(),
// 				tokens: vec![],
// 				children: vec![],
// 			};
// 			loop { 
// use_item!(&mut node, self._item(None), false);
// use_repeat!(&mut node, self._item(None), false);
// 
// 			let result = Some((node, false));
// 			return self.make_memo(start_pos, "alt".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// 			self.make_memo(start_pos, "alt".to_string(), None);
// 			None
// 		}
// 		
// 		pub fn _alt_alt(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
// 			if let Some(memo) = self.memoize("alt_alt".to_string(), None) {
// 				return Some(memo);
// 			}
// 			let start_pos = self.tokens.index();
// 			let pos = start_pos;
// 			let mut node = Node {
// 				name: "alt_alt".to_string(),
// 				tokens: vec![],
// 				children: vec![],
// 			};
// 			loop { 
// use_item!(&mut node, self.expect("|".to_string()), true);
// use_item!(&mut node, self._item(None), false);
// use_repeat!(&mut node, self._item(None), false);
// 
// 			let result = Some((node, true));
// 			return self.make_memo(start_pos, "alt_alt".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// 			self.make_memo(start_pos, "alt_alt".to_string(), None);
// 			None
// 		}
// 		
// 		pub fn _item(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
// 			if let Some(memo) = self.memoize("item".to_string(), None) {
// 				return Some(memo);
// 			}
// 			let start_pos = self.tokens.index();
// 			let pos = start_pos;
// 			let mut node = Node {
// 				name: "item".to_string(),
// 				tokens: vec![],
// 				children: vec![],
// 			};
// 			loop { 
// use_optional!(&mut node, self._prefix(None), false);
// use_item!(&mut node, self._alt_item(None), false);
// use_optional!(&mut node, self._suffix(None), false);
// 
// 			let result = Some((node, false));
// 			return self.make_memo(start_pos, "item".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// 			self.make_memo(start_pos, "item".to_string(), None);
// 			None
// 		}
// 		
// 		pub fn _alt_item(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
// 			if let Some(memo) = self.memoize("alt_item".to_string(), None) {
// 				return Some(memo);
// 			}
// 			let start_pos = self.tokens.index();
// 			let pos = start_pos;
// 			let mut node = Node {
// 				name: "alt_item".to_string(),
// 				tokens: vec![],
// 				children: vec![],
// 			};
// 			loop { 
// use_item!(&mut node, self._group(None), false);
// 
// 			let result = Some((node, true));
// 			return self.make_memo(start_pos, "alt_item".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// loop { 
// use_item!(&mut node, self._constant(None), false);
// 
// 			let result = Some((node, true));
// 			return self.make_memo(start_pos, "alt_item".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// loop { 
// use_item!(&mut node, self._error(None), false);
// 
// 			let result = Some((node, true));
// 			return self.make_memo(start_pos, "alt_item".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// 			self.make_memo(start_pos, "alt_item".to_string(), None);
// 			None
// 		}
// 		
// 		pub fn _constant(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
// 			if let Some(memo) = self.memoize("constant".to_string(), None) {
// 				return Some(memo);
// 			}
// 			let start_pos = self.tokens.index();
// 			let pos = start_pos;
// 			let mut node = Node {
// 				name: "constant".to_string(),
// 				tokens: vec![],
// 				children: vec![],
// 			};
// 			loop { 
// use_item!(&mut node, self.expect_string(None), false);
// 
// 			let result = Some((node, false));
// 			return self.make_memo(start_pos, "constant".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// loop { 
// use_item!(&mut node, self.expect_word(None), false);
// 
// 			let result = Some((node, false));
// 			return self.make_memo(start_pos, "constant".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// loop { 
// use_item!(&mut node, self.expect_number(None), false);
// 
// 			let result = Some((node, false));
// 			return self.make_memo(start_pos, "constant".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// // loop { 
// // use_item!(&mut node, self.expect_tag(None), false);
// // 
// // 			let result = Some((node, false));
// // 			return self.make_memo(start_pos, "constant".to_string(), result);
// // 		
// // }
// 		self.reset_parse(&mut node, pos);
// 			self.make_memo(start_pos, "constant".to_string(), None);
// 			None
// 		}
// 		
// 		pub fn _group(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
// 			if let Some(memo) = self.memoize("group".to_string(), None) {
// 				return Some(memo);
// 			}
// 			let start_pos = self.tokens.index();
// 			let pos = start_pos;
// 			let mut node = Node {
// 				name: "group".to_string(),
// 				tokens: vec![],
// 				children: vec![],
// 			};
// 			loop { 
// use_optional!(&mut node, self._prefix(None), false);
// use_item!(&mut node, self.expect("(".to_string()), true);
// use_item!(&mut node, self._alt(None), false);
// use_item!(&mut node, self.expect(")".to_string()), true);
// use_optional!(&mut node, self._suffix(None), false);
// 
// 			let result = Some((node, false));
// 			return self.make_memo(start_pos, "group".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// 			self.make_memo(start_pos, "group".to_string(), None);
// 			None
// 		}
// 		
// 		pub fn _prefix(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
// 			if let Some(memo) = self.memoize("prefix".to_string(), None) {
// 				return Some(memo);
// 			}
// 			let start_pos = self.tokens.index();
// 			let pos = start_pos;
// 			let mut node = Node {
// 				name: "prefix".to_string(),
// 				tokens: vec![],
// 				children: vec![],
// 			};
// 			loop { 
// use_item!(&mut node, self.expect("~".to_string()), false);
// 
// 			let result = Some((node, true));
// 			return self.make_memo(start_pos, "prefix".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// loop { 
// use_item!(&mut node, self.expect("!".to_string()), false);
// 
// 			let result = Some((node, true));
// 			return self.make_memo(start_pos, "prefix".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// loop { 
// use_item!(&mut node, self.expect("&".to_string()), false);
// 
// 			let result = Some((node, true));
// 			return self.make_memo(start_pos, "prefix".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// 			self.make_memo(start_pos, "prefix".to_string(), None);
// 			None
// 		}
// 		
// 		pub fn _suffix(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
// 			if let Some(memo) = self.memoize("suffix".to_string(), None) {
// 				return Some(memo);
// 			}
// 			let start_pos = self.tokens.index();
// 			let pos = start_pos;
// 			let mut node = Node {
// 				name: "suffix".to_string(),
// 				tokens: vec![],
// 				children: vec![],
// 			};
// 			loop { 
// use_item!(&mut node, self.expect("*".to_string()), false);
// 
// 			let result = Some((node, true));
// 			return self.make_memo(start_pos, "suffix".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// loop { 
// use_item!(&mut node, self.expect("+".to_string()), false);
// 
// 			let result = Some((node, true));
// 			return self.make_memo(start_pos, "suffix".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// loop { 
// use_item!(&mut node, self.expect("?".to_string()), false);
// 
// 			let result = Some((node, true));
// 			return self.make_memo(start_pos, "suffix".to_string(), result);
// 		
// }
// 		self.reset_parse(&mut node, pos);
// 			self.make_memo(start_pos, "suffix".to_string(), None);
// 			None
// 		}
// 		
// 
// }
// 
// fn node_from_token(token: String, name: String) -> Node {
//     Node { 
//         name,
//         tokens: vec![token], 
//         children: vec![] 
//     }
// }