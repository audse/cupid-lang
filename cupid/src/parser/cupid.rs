#![allow(clippy::all)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
use serde::{Serialize, Deserialize};
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
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
    fn expect_letter(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {
        if let Some(next) = self.tokens.peek(0) {
            if next.source.len() == 1 {
                let token = self.tokens.next().unwrap();
                return Some((node_from_token(token, "letter".to_string()), true));
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
use_item!(&mut node, self._comment(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._statement(None), false);

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
use_item!(&mut node, self._trait_definition(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._implement_type(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._implement_trait(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._typed_declaration(None), false);

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
use_item!(&mut node, self._op_assignment(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._op_increment_assignment(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._assignment(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._property_assignment(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._property_op_assignment(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._property_op_increment_assignment(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._log(None), false);

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
use_item!(&mut node, self._function(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._no_op(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._operation(None), false);

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
use_item!(&mut node, self._term(None), false);
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
use_item!(&mut node, self._term(None), false);
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
use_item!(&mut node, self._term(None), false);
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
use_item!(&mut node, self._expression(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _assignment_value(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "assignment_value".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._array(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._require_term(None), false);

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
use_item!(&mut node, self._assignment_value(None), false);

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
use_item!(&mut node, self._assignment_value(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _property_op_assignment(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "property_op_assignment".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._property_access(None), false);
use_item!(&mut node, self._operator(None), false);
use_item!(&mut node, self._equal(None), false);
use_item!(&mut node, self._assignment_value(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _property_op_increment_assignment(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "property_op_increment_assignment".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._property_access(None), false);
use_item!(&mut node, self._increment_operator(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _op_assignment(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "op_assignment".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self._operator(None), false);
use_item!(&mut node, self._equal(None), false);
use_item!(&mut node, self._assignment_value(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _operator(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "operator".to_string(),
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
loop { 
use_item!(&mut node, self.expect("*".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("/".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("^".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("%".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _op_increment_assignment(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "op_increment_assignment".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self._increment_operator(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _increment_operator(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "increment_operator".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("+".to_string()), false);
use_item!(&mut node, self.expect("+".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("-".to_string()), false);
use_item!(&mut node, self.expect("-".to_string()), false);

			return Some((node, true));
		
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
use_item!(&mut node, self._builtin_type_definition(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._struct_type_definition(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._sum_type_definition(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._alias_type_definition(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _builtin_type_definition(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "builtin_type_definition".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("type".to_string()), false);
use_item!(&mut node, self.expect_word(None), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect("=".to_string()));

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _struct_type_definition(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "struct_type_definition".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._type_symbol(None), false);
use_item!(&mut node, self._equal(None), true);
use_item!(&mut node, self.expect("[".to_string()), false);
loop { 
use_item!(&mut node, self._struct_member(None), false);
use_item!(&mut node, self.expect(",".to_string()), true);
}use_item!(&mut node, self.expect("]".to_string()), true);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _struct_member(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "struct_member".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._type_hint(None), false);
use_item!(&mut node, self._identifier(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _sum_type_definition(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "sum_type_definition".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._type_symbol(None), false);
use_item!(&mut node, self._equal(None), true);
use_item!(&mut node, self.expect("[".to_string()), false);
loop { 
use_item!(&mut node, self._sum_member(None), false);
use_item!(&mut node, self.expect(",".to_string()), true);
}use_item!(&mut node, self.expect("]".to_string()), true);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _sum_member(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "sum_member".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._type_hint(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _alias_type_definition(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "alias_type_definition".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._type_symbol(None), false);
use_item!(&mut node, self._equal(None), true);
use_item!(&mut node, self._type_hint(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _type_symbol(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "type_symbol".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("type".to_string()), false);
use_optional!(&mut node, self._generics(None), false);
use_item!(&mut node, self._identifier(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _generics(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "generics".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("[".to_string()), false);
loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self.expect(",".to_string()), false);
}use_item!(&mut node, self._closing_bracket(None), false);

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
use_item!(&mut node, self._type_hint(None), false);
use_optional!(&mut node, self.expect("mut".to_string()), false);
use_item!(&mut node, self._identifier(None), false);
loop { 
use_item!(&mut node, self._equal(None), true);
use_item!(&mut node, self._assignment_value(None), false);
break}
			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _type_hint(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "type_hint".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._array_type_hint(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._function_type_hint(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._map_type_hint(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._struct_type_hint(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._primitive_type_hint(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _array_type_hint(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "array_type_hint".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("array".to_string()), false);
use_item!(&mut node, self.expect("[".to_string()), false);
use_item!(&mut node, self._type_hint(None), false);
use_item!(&mut node, self._closing_bracket(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _map_type_hint(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "map_type_hint".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("map".to_string()), false);
use_item!(&mut node, self.expect("[".to_string()), false);
use_item!(&mut node, self._type_hint(None), false);
use_item!(&mut node, self.expect(",".to_string()), false);
use_item!(&mut node, self._type_hint(None), false);
use_item!(&mut node, self._closing_bracket(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _function_type_hint(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "function_type_hint".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("fun".to_string()), false);
use_item!(&mut node, self.expect("[".to_string()), false);
use_item!(&mut node, self._type_hint(None), false);
use_item!(&mut node, self._closing_bracket(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _primitive_type_hint(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "primitive_type_hint".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._identifier(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _struct_type_hint(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "struct_type_hint".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self.expect("[".to_string()), false);
loop { 
use_item!(&mut node, self._struct_member_type_hint(None), false);
use_item!(&mut node, self.expect(",".to_string()), false);
}use_item!(&mut node, self.expect("]".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _struct_member_type_hint(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "struct_member_type_hint".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self.expect(":".to_string()), false);
use_item!(&mut node, self._type_hint(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _implement_type(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "implement_type".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("use".to_string()), false);
use_optional!(&mut node, self._generics(None), false);
use_item!(&mut node, self.expect_word(None), false);
use_item!(&mut node, self.expect("{".to_string()), false);
use_repeat!(&mut node, self._typed_declaration(None), false);
use_item!(&mut node, self._closing_brace(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _implement_trait(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "implement_trait".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("use".to_string()), false);
use_optional!(&mut node, self._generics(None), false);
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self.expect("with".to_string()), false);
use_item!(&mut node, self._identifier(None), false);
use_optional!(&mut node, self._implement_trait_body(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _implement_trait_body(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "implement_trait_body".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("{".to_string()), false);
use_repeat!(&mut node, self._typed_declaration(None), false);
use_item!(&mut node, self._closing_brace(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _trait_definition(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "trait_definition".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("trait".to_string()), false);
use_item!(&mut node, self._generics(None), false);
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self._equal(None), false);
use_item!(&mut node, self.expect("[".to_string()), false);
loop { 
use_item!(&mut node, self._typed_declaration(None), false);
use_item!(&mut node, self.expect(",".to_string()), false);
}use_item!(&mut node, self.expect("]".to_string()), false);

			return Some((node, false));
		
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
		
		pub fn _atom(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "atom".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._function_call(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._range(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._map(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._bracket_array(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._group(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._unary_op(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._type_hint(None), false);

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
use_item!(&mut node, self._char(None), false);

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
use_item!(&mut node, self._self(None), false);

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
use_item!(&mut node, self._term(None), false);
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
loop { 
use_item!(&mut node, self._parameter(None), false);
use_item!(&mut node, self.expect(",".to_string()), true);
}
			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _parameter(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "parameter".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("_".to_string()), true);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_optional!(&mut node, self.expect("mut".to_string()), false);
use_item!(&mut node, self._self(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._annotated_parameter(None), false);

			return Some((node, true));
		
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
use_item!(&mut node, self._type_hint(None), false);
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
use_item!(&mut node, self._term(None), false);
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
		
		pub fn _bracket_array(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "bracket_array".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("[".to_string()), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect(".".to_string()));
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect(".".to_string()));
use_optional!(&mut node, self._array(None), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect(".".to_string()));
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect(".".to_string()));
use_item!(&mut node, self._closing_bracket(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _array(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "array".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._operation(None), false);
use_item!(&mut node, self.expect(",".to_string()), true);
loop { 
use_item!(&mut node, self._operation(None), false);
use_item!(&mut node, self.expect(",".to_string()), true);
}
			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _map(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "map".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("[".to_string()), false);
loop { 
use_item!(&mut node, self._map_entry(None), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect("]".to_string()));
use_item!(&mut node, self.expect(",".to_string()), false);
}use_item!(&mut node, self.expect("]".to_string()), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _map_entry(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "map_entry".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._atom(None), false);
use_item!(&mut node, self.expect(":".to_string()), false);
use_item!(&mut node, self._term(None), false);

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
use_item!(&mut node, self._range_term(None), false);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self._range_term(None), false);
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
use_item!(&mut node, self._range_term(None), false);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self.expect("]".to_string()), false);
use_item!(&mut node, self._range_term(None), false);

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
use_item!(&mut node, self._range_term(None), false);
use_item!(&mut node, self.expect("[".to_string()), false);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self._range_term(None), false);
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
use_item!(&mut node, self._range_term(None), false);
use_item!(&mut node, self.expect("[".to_string()), false);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self.expect(".".to_string()), true);
use_item!(&mut node, self.expect("]".to_string()), false);
use_item!(&mut node, self._range_term(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _range_term(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "range_term".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._function_call(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._group(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._unary_op(None), false);

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
		
		pub fn _no_op(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "no_op".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect("+".to_string()));
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect("-".to_string()));
use_item!(&mut node, self._atom(None), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self._operator(None));
use_negative_lookahead!(self, self.tokens.index(), &mut self._keyword_operator(None));
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect(".".to_string()));

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
loop { 
use_item!(&mut node, self.expect("%".to_string()), false);
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
use_item!(&mut node, self._property_access(None), false);
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
		
		pub fn _property_access(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "property_access".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._atom(None), false);
use_optional!(&mut node, self._property_access_suffix(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _property_access_suffix(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "property_access_suffix".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect(".".to_string()), false);
use_item!(&mut node, self._property_access(None), false);

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
use_item!(&mut node, self.expect("+".to_string()), false);
use_item!(&mut node, self._atom(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("-".to_string()), false);
use_item!(&mut node, self._atom(None), false);

			return Some((node, false));
		
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
use_optional!(&mut node, self._term(None), false);

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
use_optional!(&mut node, self._term(None), false);

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
		
		pub fn _char(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "char".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("\\".to_string()), false);
use_item!(&mut node, self.expect_letter(None), false);

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
		
		pub fn _require_term(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "require_term".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._term(None), false);

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
		
		pub fn _self(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: "self".to_string(),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("self".to_string()), false);

			return Some((node, false));
		
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
use_item!(&mut node, self.expect("mut".to_string()), false);

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
loop { 
use_item!(&mut node, self.expect("continue".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("type".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
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
loop { 
use_item!(&mut node, self.expect("use".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("trait".to_string()), false);

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
use_item!(&mut node, self.expect("as".to_string()), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("istype".to_string()), false);

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