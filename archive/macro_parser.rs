/*

Rule:
- alts: Vec<Vec<Alt>>
- name: String

Alt:
- kind: String
- source: String
- suffix_modifier: Option<String>
- prefix_modifier: Option<String>

*/

macro_rules! parse_grammar {
	($parser:ident, $rules:ident) => {
		
	}
}

macro_rules! parse_rule {
	($parser:ident, $rule:ident) => {
		fn _($rule).name() {
			
		}
	}
}

macro_rules! parse_alternate {
	($parser:ident, $items:ident) => {}
}

macro_rules! parse_item {
	($parser:ident, $item:ident, $get_next:ident) => {}
}

// macro_rules! use_rule {
// 	($rule:ident) => {{
// 		let mut alt_list = $rule.alts.iter();
// 		
// 		let mut pos = self.tokens.index()
// 		let mut tokens = vec![];
// 		let mut children = vec![];
// 		
// 		 
// 		use_alternate!($rule.name, $items.next(), tokens, children, $items.next);
// 		
// 	}}
// }
// 
// macro_rules! use_alternate {
// 	($rule_name:ident, $item:ident, $tokens:ident, $children:ident, $get_next:ident) => {{
// 		let method = get_method_name($item);
// 		let is_token = is_token($item);
// 		let is_required = is_required($item);
// 		
// 		if is_required {
// 			if let Some(val) = $method($($val)?) {
// 				if is_token {
// 					$tokens.push(val);
// 				} else { 
// 					$children.push(val); 
// 				}
// 				if let Some(next) = $get_next() {
// 					use_alternate!($rule_name, next, $tokens, $children, get_next);
// 				} else {
// 					return Some(Node { 
// 						name: String::from($rule_name), 
// 						tokens: $tokens, 
// 						children: $children 
// 					});
// 				}
// 			}
// 		} else {
// 			if let Some(val) = $method($($val)?) {
// 				if is_token {
// 					$tokens.push(val);
// 				} else { 
// 					$children.push(val); 
// 				}
// 			}
// 			if let Some(next) = $get_next() {
// 				use_alternate!($rule_name, next, $tokens, $children, get_next);
// 			} else {
// 				return Some(Node { 
// 					name: String::from($rule_name), 
// 					tokens: $tokens, 
// 					children: $children 
// 				});
// 			}
// 		}
// 		
// 		// if let Some(prefix) = $item.prefix_modifier {
// 		// 	use_prefix_modifier!(prefix, method, if is_token { tokens } else { children });
// 		// }
// 		// 
// 		// if let Some(suffix) = $item.suffix_modifier {
// 		// 	use_suffix_modifier!(suffix, method, if is_token { tokens } else { children });
// 		// }
// 		
// 	}};
// }
// 
// macro_rules! use_prefix_modifier {
// 	($modifier:ident, $method:ident, $list:ident, $(, $val:expr)?) => {{
// 		match $modifier.as_str() {
// 			"~" => use_conceal_modifier!(),
// 			"!" => use_negative_lookahead_modifier!(),
// 			"&" => use_positive_lookahead_modifier!(),
// 		}
// 	}};
// }
// 
// 
// macro_rules! use_suffix_modifier {
// 	($modifier:ident, $method:ident, $list:ident $(, $val:expr)?) => {{
// 		match $modifier.as_str() {
// 			"+" | "*" => use_repeat_modifier!($method, $list $(, $val)?),
// 			"?" => use_optional_modifier!($method, $list, $(, $val)?),
// 		}
// 	}};
// }
// 
// macro_rules! use_repeat_modifier {
// 	($method:ident, $list:ident $(, $val:expr)?) => {{
// 		while let Some(val) = $method($($val)?) {
// 			$list.push(val);
// 		}
// 	}}
// }
// 
// macro_rules! use_optional_modifier {
// 	($method:ident, $list:ident $(, $val:expr)?) => {{
// 		if let Some(val) = $method($($val)?) {
// 			$list.push(val);
// 		}
// 	}};
// }
