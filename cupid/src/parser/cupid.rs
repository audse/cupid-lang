#![allow(clippy::all)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
use crate::{
    is_identifier, is_number, is_string, is_uppercase, BiDirectionalIterator, Tokenizer, Token,
};

macro_rules! use_item {
    ($item:expr, $method:expr, $is_concealed:expr) => {{
        if let Some((mut val, pass_through)) = $method {
            if pass_through && !$is_concealed {
                // move returned node's children to current node
                $item.tokens.append(&mut val.tokens);
                $item.children.append(&mut val.children);
            } else if !$is_concealed {
                $item.children.push(val);
            }
        } else {
            break;
        }
    }};
}

macro_rules! use_optional {
    ($item:expr, $method:expr, $is_concealed:expr) => {{
        if let Some((mut val, pass_through)) = $method {
            if pass_through && !$is_concealed {
                $item.tokens.append(&mut val.tokens);
                $item.children.append(&mut val.children);
            } else if !$is_concealed {
                $item.children.push(val);
            }
        }
    }};
}

macro_rules! use_repeat {
    ($item:expr, $method:expr, $is_concealed:expr) => {{
        while let Some((mut val, pass_through)) = $method {
            if pass_through && !$is_concealed {
                $item.tokens.append(&mut val.tokens);
                $item.children.append(&mut val.children);
            } else if !$is_concealed {
                $item.children.push(val);
            }
        }
    }};
}

macro_rules! use_negative_lookahead {
    ($parser:expr, $index:expr, $method:expr) => {{
        let index = $index;
        if let Some((_val, _pass_through)) = $method {
            $parser.tokens.goto(index);
            break;
        }
    }};
}

macro_rules! use_positive_lookahead {
    ($parser:expr, $index:expr, $method:expr) => {{
        let index = $index;
        if let Some((_val, _pass_through)) = $method {
        } else {
            $parser.tokens.goto(index);
            break;
        }
    }};
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
    pub tokens: Vec<Token>,
}

#[derive(PartialEq, Eq)]
pub struct Parser {
    pub tokens: BiDirectionalIterator<Token>,
    pub index: usize,
}

impl Parser {
    pub fn new(source: String) -> Self {
        let mut tokenizer = Tokenizer::new(source.as_str());
        tokenizer.scan();
        Self {
            index: 0,
            tokens: BiDirectionalIterator::new(tokenizer.tokens),
        }
    }
    
    #[inline]
    fn expect(&mut self, rule_name: String) -> Option<(Node, bool)> {
        if let Some(token) = self.tokens.peek(0) {
            if token.source == rule_name {
                return Some((node_from_token(self.tokens.next().unwrap(), rule_name), true));
            }
        }
        None
    }
    
    #[inline]
    fn expect_one(&mut self, rule_names: Vec<String>) -> Option<(Node, bool)> {
        for rule_name in rule_names {
            if let Some(next) = self.expect(rule_name) {
                return Some(next);
            }
        }
        None
    }
    
    #[inline]
    fn expect_constant(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(next) = self.tokens.peek(0) {
            if is_uppercase(&next.source) {
                let token = self.tokens.next().unwrap();
                return Some((node_from_token(token, "constant".to_string()), true));
            }
        }
        None
    }
    
    #[inline]
    fn expect_word(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(next) = self.tokens.peek(0) {
            if is_identifier(&next.source) {
                let token = self.tokens.next().unwrap();
                return Some((node_from_token(token, "word".to_string()), true));
            }
        }
        None
    }
    
    #[inline]
    fn expect_string(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(next) = self.tokens.peek(0) {
            if is_string(&next.source) {
                let token = self.tokens.next().unwrap();
                return Some((node_from_token(token, "string".to_string()), true));
            }
        }
        None
    }
    
    #[inline]
    fn expect_number(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(next) = self.tokens.peek(0) {
            if is_number(&next.source) {
                let token = self.tokens.next().unwrap();
                return Some((node_from_token(token, "number".to_string()), true));
            }
        }
        None
    }
    
    #[inline]
    fn expect_tag(&mut self, arg: String) -> Option<(Node, bool)> {
        if !self.tokens.at_end() {
            let current_token = self.tokens.peek_back(1).unwrap();
            return Some((
                Node {
                    name: "error".to_string(),
                    tokens: vec![
                        Token {
                            source: arg,
                            index: current_token.index + 1,
                            line: current_token.line,
                        },
                        current_token.clone()
                    ],
                    children: vec![],
                },
                false,
            ));
        }
        None
    }
    
    #[inline]
    fn expect_any(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(next) = self.tokens.next() {
            return Some((node_from_token(next, "any".to_string()), false));
        }
        None
    }
    
    #[inline]
    fn reset_parse(&mut self, item: &mut Node, pos: usize) {
        item.tokens.clear();
        item.children.clear();
        self.tokens.goto(pos);
    }
    
    
		pub fn _file(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "file".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._expression(None), false);
use_repeat!(&mut node, self._expression(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _expression(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "expression".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._statement(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._expression_item(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._comment(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _statement(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "statement".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._type_definition(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._typed_declaration(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _expression_item(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "expression_item".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._loop(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._block(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._break(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._return(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._continue(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._function(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._internal_property_assignment(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._property_assignment(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._assignment(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._term(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _loop(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "loop".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._for_loop(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._while_loop(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._infinite_loop(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _for_loop(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "for_loop".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("for".to_string()), false);
use_item!(&mut node, self._for_loop_parameters(None), false);
use_item!(&mut node, self.expect("in".to_string()), true);
use_item!(&mut node, self._term(None), false);
use_item!(&mut node, self._block(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _for_loop_parameters(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "for_loop_parameters".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self.expect(",".to_string()), true);
}
			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _while_loop(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "while_loop".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("while".to_string()), false);
use_item!(&mut node, self._expression_item(None), false);
use_item!(&mut node, self._block(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _infinite_loop(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "infinite_loop".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("loop".to_string()), false);
use_item!(&mut node, self._block(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _block(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "block".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._if_block(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._box_block(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._brace_block(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._arrow_block(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _if_block(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "if_block".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("if".to_string()), true);
use_item!(&mut node, self._expression(None), false);
use_item!(&mut node, self._block(None), false);
use_repeat!(&mut node, self._else_if_block(None), false);
use_optional!(&mut node, self._else_block(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _else_if_block(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "else_if_block".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("else".to_string()), true);
use_item!(&mut node, self.expect("if".to_string()), true);
use_item!(&mut node, self._expression(None), false);
use_item!(&mut node, self._block(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _else_block(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "else_block".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("else".to_string()), true);
use_item!(&mut node, self._block(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _box_block(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "box_block".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("box".to_string()), true);
use_item!(&mut node, self._brace_block(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _brace_block(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "brace_block".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("{".to_string()), true);
use_repeat!(&mut node, self._expression(None), false);
use_item!(&mut node, self._closing_brace(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _arrow_block(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "arrow_block".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._arrow(None), true);
use_item!(&mut node, self._require_expression(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _assignment(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "assignment".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self._equal(None), false);
use_item!(&mut node, self._expression(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _internal_property_assignment(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "internal_property_assignment".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._internal_property_access(None), false);
use_item!(&mut node, self._equal(None), false);
use_item!(&mut node, self._expression(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _property_assignment(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "property_assignment".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._property_access(None), false);
use_item!(&mut node, self._equal(None), false);
use_item!(&mut node, self._expression(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _type_definition(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "type_definition".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("type".to_string()), false);
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self.expect("[".to_string()), true);
loop { 
use_item!(&mut node, self._type_field(None), false);
use_item!(&mut node, self.expect(",".to_string()), false);
}use_item!(&mut node, self._closing_bracket(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _type_field(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "type_field".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._type(None), false);
use_item!(&mut node, self._identifier(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self._identifier(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _typed_declaration(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "typed_declaration".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._builtin_typed_declaration(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._custom_typed_declaration(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _builtin_typed_declaration(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "builtin_typed_declaration".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._type(None), false);
use_optional!(&mut node, self.expect("mut".to_string()), false);
use_item!(&mut node, self._identifier(None), false);
loop { 
use_item!(&mut node, self._equal(None), true);
use_item!(&mut node, self._expression_item(None), false);
break}
			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _custom_typed_declaration(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "custom_typed_declaration".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._identifier(None), false);
use_optional!(&mut node, self.expect("mut".to_string()), false);
use_item!(&mut node, self._identifier(None), false);
loop { 
use_item!(&mut node, self._equal(None), true);
use_item!(&mut node, self._expression_item(None), false);
break}
			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _equal(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "equal".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("=".to_string()), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect(">".to_string()));

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _term(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "term".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._log(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._structure(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._operation(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _structure(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "structure".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._dictionary(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._list(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._range(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _value(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "value".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._function_call(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._internal_property_access(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._property_access(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._atom(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _atom(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "atom".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._group(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._boolean(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._none(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._string(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._decimal(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._number(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._identifier(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _group(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "group".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("(".to_string()), true);
use_item!(&mut node, self._expression_item(None), false);
use_item!(&mut node, self._closing_paren(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _function(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "function".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._parameters(None), false);
use_item!(&mut node, self._block(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _parameters(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "parameters".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("_".to_string()), true);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
loop { 
use_item!(&mut node, self._annotated_parameter(None), false);
use_item!(&mut node, self.expect(",".to_string()), true);
}
			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _annotated_parameter(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "annotated_parameter".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._type(None), false);
use_item!(&mut node, self._identifier(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self._identifier(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _log(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "log".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._log_keyword(None), false);
use_item!(&mut node, self.expect("(".to_string()), true);
use_item!(&mut node, self._arguments(None), false);
use_item!(&mut node, self._closing_paren(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _function_call(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "function_call".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self.expect("(".to_string()), true);
use_item!(&mut node, self._arguments(None), false);
use_item!(&mut node, self._closing_paren(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _arguments(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "arguments".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
loop { 
use_item!(&mut node, self._expression_item(None), false);
use_item!(&mut node, self.expect(",".to_string()), true);
}
			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _log_keyword(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "log_keyword".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("log".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("logs".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("log_line".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("logs_line".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _list(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "list".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("[".to_string()), false);
loop { 
use_item!(&mut node, self._expression_item(None), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect("]".to_string()));
use_item!(&mut node, self.expect(",".to_string()), false);
}use_item!(&mut node, self.expect("]".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _dictionary(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "dictionary".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("[".to_string()), false);
loop { 
use_item!(&mut node, self._dictionary_entry(None), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect("]".to_string()));
use_item!(&mut node, self.expect(",".to_string()), false);
}use_item!(&mut node, self.expect("]".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _dictionary_entry(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "dictionary_entry".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._term(None), false);
use_item!(&mut node, self.expect(":".to_string()), false);
use_item!(&mut node, self._expression_item(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _range(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "range".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._range_inclusive_inclusive(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._range_inclusive_exclusive(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._range_exclusive_inclusive(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._range_exclusive_exclusive(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _range_inclusive_inclusive(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "range_inclusive_inclusive".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("[".to_string()), false);
use_item!(&mut node, self._atom(None), false);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self._atom(None), false);
use_item!(&mut node, self.expect("]".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _range_inclusive_exclusive(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "range_inclusive_exclusive".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("[".to_string()), false);
use_item!(&mut node, self._atom(None), false);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self.expect("]".to_string()), false);
use_item!(&mut node, self._atom(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _range_exclusive_inclusive(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "range_exclusive_inclusive".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._atom(None), false);
use_item!(&mut node, self.expect("[".to_string()), false);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self._atom(None), false);
use_item!(&mut node, self.expect("]".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _range_exclusive_exclusive(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "range_exclusive_exclusive".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._atom(None), false);
use_item!(&mut node, self.expect("[".to_string()), false);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self.expect("]".to_string()), false);
use_item!(&mut node, self._atom(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _internal_property_access(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "internal_property_access".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("self".to_string()), false);
use_item!(&mut node, self.expect(".".to_string()), false);
use_item!(&mut node, self._require_property(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _property_access(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "property_access".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._property_accessor(None), false);
use_item!(&mut node, self.expect(".".to_string()), false);
use_item!(&mut node, self._require_property(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _property_accessor(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "property_accessor".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._structure(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._function_call(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._identifier(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("self".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _require_property(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "require_property".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._value(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect_tag("<e 'missing index or property name'>".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _operation(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "operation".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._binary_op(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._unary_op(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _binary_op(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "binary_op".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._compare_op(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _compare_op(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "compare_op".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._add(None), false);
use_optional!(&mut node, self._compare_suffix(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _compare_suffix(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "compare_suffix".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._keyword_operator(None), false);
use_item!(&mut node, self._compare_op(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _add(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "add".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._multiply(None), false);
use_optional!(&mut node, self._add_suffix(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _add_suffix(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "add_suffix".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("+".to_string()), false);
use_item!(&mut node, self._add(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("-".to_string()), false);
use_item!(&mut node, self._add(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _multiply(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "multiply".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._exponent(None), false);
use_optional!(&mut node, self._multiply_suffix(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _multiply_suffix(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "multiply_suffix".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("*".to_string()), false);
use_item!(&mut node, self._multiply(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("/".to_string()), false);
use_item!(&mut node, self._multiply(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _exponent(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "exponent".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._value(None), false);
use_optional!(&mut node, self._exponent_suffix(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _exponent_suffix(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "exponent_suffix".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("^".to_string()), false);
use_item!(&mut node, self._exponent(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _unary_op(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "unary_op".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._unary_operator(None), false);
use_item!(&mut node, self._require_expression(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _unary_operator(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "unary_operator".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("+".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("-".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _break(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "break".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("break".to_string()), false);
use_optional!(&mut node, self._expression_item(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _return(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "return".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("return".to_string()), false);
use_optional!(&mut node, self._expression_item(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _continue(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "continue".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("continue".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _boolean(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "boolean".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("true".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("false".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _none(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "none".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("none".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _identifier(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "identifier".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_negative_lookahead!(self, self.tokens.index(), &mut self._keyword(None));
use_item!(&mut node, self.expect_word(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _string(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "string".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect_string(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _decimal(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "decimal".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect_number(None), false);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self.expect_number(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _number(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "number".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect_number(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _require_expression(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "require_expression".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._expression_item(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect_tag("<e 'missing expression'>".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _closing_paren(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "closing_paren".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect(")".to_string()), true);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect_tag("<e 'missing closing parenthesis'>".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _closing_brace(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "closing_brace".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("}".to_string()), true);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect_tag("<e 'missing closing brace'>".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _closing_bracket(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "closing_bracket".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("]".to_string()), true);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect_tag("<e 'missing closing bracket'>".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _keyword(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "keyword".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._keyword_variable(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._keyword_operator(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._reserved_word(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._type(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._boolean(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._none(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _reserved_word(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "reserved_word".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("for".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("while".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("else".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("if".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("loop".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("box".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("break".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("return".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _keyword_variable(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "keyword_variable".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("let".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("const".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _keyword_operator(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "keyword_operator".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("in".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("is".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("and".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("not".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("or".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect(">".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect(">".to_string()), false);
use_item!(&mut node, self.expect("=".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("<".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("<".to_string()), false);
use_item!(&mut node, self.expect("=".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _arrow(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "arrow".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("=".to_string()), true);
use_item!(&mut node, self.expect(">".to_string()), true);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _type(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "type".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._none(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("bool".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("int".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("dec".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("string".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("fun".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("maybe".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("list".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("dict".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("tuple".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _comment_delimiter(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "comment_delimiter".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("*".to_string()), false);
use_item!(&mut node, self.expect("*".to_string()), false);
use_item!(&mut node, self.expect("*".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _comment_content(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "comment_content".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect_any(None), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self._comment_delimiter(None));

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _comment(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "comment".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._comment_delimiter(None), true);
use_repeat!(&mut node, self._comment_content(None), false);
use_item!(&mut node, self.expect_any(None), false);
use_item!(&mut node, self._comment_delimiter(None), true);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		

}


fn node_from_token(token: Token, name: String) -> Node {
    Node { 
        name,
        tokens: vec![token], 
        children: vec![] 
    }
}