#![allow(clippy::all)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
use serde::{Serialize, Deserialize};
use crate::*;

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
pub struct Node<'src> {
    pub name: Cow<'src, str>,
    pub children: Vec<Node<'src>>,
    pub tokens: Vec<Token<'src>>,
}

impl<'src> Node<'src> {
    pub fn map<R>(&self, function: &dyn Fn(&Self) -> R) -> Vec<R> {
        self.children.iter().map(function).collect()
    }
    pub fn map_mut<R>(&mut self, function: &dyn Fn(&mut Self) -> R) -> Vec<R> {
        self.children.iter_mut().map(function).collect()
    }
	pub fn filter_map_mut<R>(&mut self, function: &dyn Fn(&mut Self) -> Option<R>) -> Vec<R> {
		self.children.iter_mut().filter_map(function).collect()
	}
    pub fn has(&self, name: &str) -> bool {
        self.children
            .iter()
            .find(|c| c.name == name)
            .is_some()
    }
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Self> {
        self.children
            .iter_mut()
            .find(|c| c.name == name)
    }
}

#[derive(PartialEq, Eq)]
pub struct Parser<'src> {
    pub tokens: BiDirectionalIterator<Token<'src>>,
    pub index: usize,
}

impl<'src> Parser<'src> {
    pub fn new(source: String) -> Self {
        let mut tokenizer = Tokenizer::new(source.into());
        tokenizer.scan();
        Self {
            index: 0,
            tokens: BiDirectionalIterator::new(tokenizer.tokens),
        }
    }
    
    #[inline]
    fn expect(&mut self, rule_name: &'src str) -> Option<(Node, bool)> {
        if let Some(token) = self.tokens.peek(0) {
            if token.source == rule_name {
                return Some((node_from_token(self.tokens.next().unwrap(), rule_name), true));
            }
        }
        None
    }
    
    #[inline]
    fn expect_one(&mut self, rule_names: Vec<&'src str>) -> Option<(Node, bool)> {
        for rule_name in rule_names {
            if let Some(next) = self.expect(rule_name) {
                return Some(next);
            }
        }
        None
    }
    
    #[inline]
    fn expect_constant(&mut self, _arg: Option<()>) -> Option<(Node, bool)> {
        if let Some(next) = self.tokens.peek(0) {
            if is_uppercase(&next.source) {
                let token = self.tokens.next().unwrap();
                return Some((node_from_token(token, "constant"), true));
            }
        }
        None
    }
    
    #[inline]
    fn expect_word(&mut self, _arg: Option<()>) -> Option<(Node, bool)> {
        if let Some(next) = self.tokens.peek(0) {
            if is_identifier(&next.source) {
                let token = self.tokens.next().unwrap();
                return Some((node_from_token(token, "word"), true));
            }
        }
        None
    }
    
    #[inline]
    fn expect_string(&mut self, _arg: Option<()>) -> Option<(Node, bool)> {
        if let Some(next) = self.tokens.peek(0) {
            if is_string(&next.source) {
                let token = self.tokens.next().unwrap();
                return Some((node_from_token(token, "string"), true));
            }
        }
        None
    }
    
    #[inline]
    fn expect_letter(&mut self, _arg: Option<()>) -> Option<(Node, bool)> {
        if let Some(next) = self.tokens.peek(0) {
            if next.source.len() == 1 {
                let token = self.tokens.next().unwrap();
                return Some((node_from_token(token, "letter"), true));
            }
        }
        None
    }
    
    #[inline]
    fn expect_number(&mut self, _arg: Option<()>) -> Option<(Node, bool)> {
        if let Some(next) = self.tokens.peek(0) {
            if is_number(&next.source) {
                let token = self.tokens.next().unwrap();
                return Some((node_from_token(token, "number"), true));
            }
        }
        None
    }
    
    #[inline]
    fn expect_tag(&mut self, arg: &'src str) -> Option<(Node, bool)> {
        if !self.tokens.at_end() {
            let current_token = self.tokens.peek_back(1).unwrap();
            return Some((
                Node {
                    name: Cow::Borrowed("error"),
                    tokens: vec![
                        Token {
                            source: Cow::Borrowed(arg),
                            index: current_token.index + 1,
                            line: current_token.line,
                        },
                        current_token.to_owned()
                    ],
                    children: vec![],
                },
                false,
            ));
        }
        None
    }
    
    #[inline]
    fn expect_any(&mut self, _arg: Option<()>) -> Option<(Node, bool)> {
        if let Some(next) = self.tokens.next() {
            return Some((node_from_token(next, "any"), false));
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
				name: Cow::Borrowed("file"),
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
				name: Cow::Borrowed("expression"),
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
				name: Cow::Borrowed("statement"),
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
				name: Cow::Borrowed("term"),
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
use_item!(&mut node, self._property_access(None), false);

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
		
		pub fn _loop(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("loop"),
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
				name: Cow::Borrowed("for_loop"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("for"), false);
use_item!(&mut node, self._for_loop_parameters(None), false);
use_item!(&mut node, self.expect("in"), true);
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
				name: Cow::Borrowed("for_loop_parameters"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self.expect(","), true);
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
				name: Cow::Borrowed("while_loop"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("while"), false);
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
				name: Cow::Borrowed("infinite_loop"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("loop"), false);
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
				name: Cow::Borrowed("block"),
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
				name: Cow::Borrowed("if_block"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("if"), true);
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
				name: Cow::Borrowed("else_if_block"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("else"), true);
use_item!(&mut node, self.expect("if"), true);
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
				name: Cow::Borrowed("else_block"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("else"), true);
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
				name: Cow::Borrowed("box_block"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("box"), true);
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
				name: Cow::Borrowed("brace_block"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("{"), true);
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
				name: Cow::Borrowed("arrow_block"),
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
		
		pub fn _assignment(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("assignment"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self._equal(None), false);
use_item!(&mut node, self._term(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _property_assignment(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("property_assignment"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._property_access(None), false);
use_item!(&mut node, self._equal(None), false);
use_item!(&mut node, self._term(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _property_op_assignment(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("property_op_assignment"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._property_access(None), false);
use_item!(&mut node, self._operator(None), false);
use_item!(&mut node, self._equal(None), false);
use_item!(&mut node, self._term(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _property_access(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("property_access"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._atom(None), false);
use_item!(&mut node, self.expect("."), false);
use_item!(&mut node, self._term(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _op_assignment(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("op_assignment"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self._operator(None), false);
use_item!(&mut node, self._equal(None), false);
use_item!(&mut node, self._term(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _operator(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("operator"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("+"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("-"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("*"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("/"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("^"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("%"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _type_definition(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("type_definition"),
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
				name: Cow::Borrowed("builtin_type_definition"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("type"), false);
use_item!(&mut node, self.expect_word(None), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect("="));

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _struct_type_definition(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("struct_type_definition"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._type_symbol(None), false);
use_item!(&mut node, self._equal(None), true);
use_item!(&mut node, self.expect("["), false);
loop { 
use_item!(&mut node, self._struct_member(None), false);
use_item!(&mut node, self.expect(","), true);
}use_item!(&mut node, self.expect("]"), true);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _struct_member(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("struct_member"),
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
				name: Cow::Borrowed("sum_type_definition"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._type_symbol(None), false);
use_item!(&mut node, self._equal(None), true);
use_item!(&mut node, self.expect("["), false);
loop { 
use_item!(&mut node, self._sum_member(None), false);
use_item!(&mut node, self.expect(","), true);
}use_item!(&mut node, self.expect("]"), true);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _sum_member(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("sum_member"),
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
				name: Cow::Borrowed("alias_type_definition"),
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
				name: Cow::Borrowed("type_symbol"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("type"), false);
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
				name: Cow::Borrowed("generics"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("["), false);
loop { 
use_item!(&mut node, self._generic_argument(None), false);
use_item!(&mut node, self.expect(","), false);
}use_item!(&mut node, self._closing_bracket(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _generic_argument(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("generic_argument"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._identifier(None), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect(":"));

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self.expect(":"), false);
use_item!(&mut node, self._type_hint(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _typed_declaration(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("typed_declaration"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._type_hint(None), false);
use_optional!(&mut node, self.expect("mut"), false);
use_item!(&mut node, self._identifier(None), false);
loop { 
use_item!(&mut node, self._equal(None), true);
use_item!(&mut node, self._term(None), false);
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
				name: Cow::Borrowed("type_hint"),
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
				name: Cow::Borrowed("array_type_hint"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._array_kw(None), false);
use_item!(&mut node, self.expect("["), false);
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
				name: Cow::Borrowed("map_type_hint"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._map_kw(None), false);
use_item!(&mut node, self.expect("["), false);
use_item!(&mut node, self._type_hint(None), false);
use_item!(&mut node, self.expect(","), false);
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
				name: Cow::Borrowed("function_type_hint"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._fun_kw(None), false);
use_item!(&mut node, self.expect("["), false);
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
				name: Cow::Borrowed("primitive_type_hint"),
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
		
		pub fn _array_kw(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("array_kw"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("array"), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _map_kw(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("map_kw"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("map"), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _fun_kw(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("fun_kw"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("fun"), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _struct_type_hint(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("struct_type_hint"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self.expect("["), false);
loop { 
use_item!(&mut node, self._struct_member_type_hint(None), false);
use_item!(&mut node, self.expect(","), false);
}use_item!(&mut node, self.expect("]"), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _struct_member_type_hint(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("struct_member_type_hint"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self.expect(":"), false);
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
				name: Cow::Borrowed("implement_type"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("use"), false);
use_optional!(&mut node, self._generics(None), false);
use_item!(&mut node, self._type_hint(None), false);
use_item!(&mut node, self.expect("{"), false);
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
				name: Cow::Borrowed("implement_trait"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("use"), false);
use_optional!(&mut node, self._generics(None), false);
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self.expect("with"), false);
use_item!(&mut node, self._type_hint(None), false);
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
				name: Cow::Borrowed("implement_trait_body"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("{"), false);
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
				name: Cow::Borrowed("trait_definition"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("trait"), false);
use_item!(&mut node, self._generics(None), false);
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self._equal(None), true);
use_item!(&mut node, self.expect("["), true);
loop { 
use_item!(&mut node, self._typed_declaration(None), false);
use_item!(&mut node, self.expect(","), false);
}use_item!(&mut node, self._closing_bracket(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _equal(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("equal"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("="), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect(">"));

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _atom(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("atom"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._empty(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
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
		
		pub fn _empty(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("empty"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("_"), true);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _group(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("group"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("("), true);
use_optional!(&mut node, self._term(None), false);
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
				name: Cow::Borrowed("function"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._parameters(None), false);
use_item!(&mut node, self._function_body(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _function_body(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("function_body"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._arrow(None), false);
use_item!(&mut node, self._empty(None), true);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._arrow(None), false);
use_item!(&mut node, self._group(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self._block(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _parameters(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("parameters"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
loop { 
use_item!(&mut node, self._parameter(None), false);
use_item!(&mut node, self.expect(","), true);
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
				name: Cow::Borrowed("parameter"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("_"), true);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_optional!(&mut node, self.expect("mut"), false);
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
				name: Cow::Borrowed("annotated_parameter"),
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
				name: Cow::Borrowed("log"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._log_keyword(None), false);
use_item!(&mut node, self.expect("("), true);
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
				name: Cow::Borrowed("function_call"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._identifier(None), false);
use_item!(&mut node, self.expect("("), true);
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
				name: Cow::Borrowed("arguments"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
loop { 
use_item!(&mut node, self._term(None), false);
use_item!(&mut node, self.expect(","), true);
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
				name: Cow::Borrowed("log_keyword"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("log"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("logs"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("log_line"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("logs_line"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _bracket_array(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("bracket_array"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("["), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect("."));
use_optional!(&mut node, self._array(None), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect("."));
use_item!(&mut node, self._closing_bracket(None), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _array(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("array"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
loop { 
use_item!(&mut node, self._term(None), false);
use_item!(&mut node, self.expect(","), true);
}use_optional!(&mut node, self._term(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _map(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("map"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("["), false);
loop { 
use_item!(&mut node, self._map_entry(None), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect("]"));
use_item!(&mut node, self.expect(","), false);
}use_item!(&mut node, self.expect("]"), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _map_entry(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("map_entry"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._atom(None), false);
use_item!(&mut node, self.expect(":"), false);
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
				name: Cow::Borrowed("range"),
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
				name: Cow::Borrowed("range_inclusive_inclusive"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("["), false);
use_item!(&mut node, self._range_term(None), false);
use_item!(&mut node, self.expect("."), true);
use_item!(&mut node, self.expect("."), true);
use_item!(&mut node, self._range_term(None), false);
use_item!(&mut node, self.expect("]"), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _range_inclusive_exclusive(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("range_inclusive_exclusive"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("["), false);
use_item!(&mut node, self._range_term(None), false);
use_item!(&mut node, self.expect("."), true);
use_item!(&mut node, self.expect("."), true);
use_item!(&mut node, self.expect("]"), false);
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
				name: Cow::Borrowed("range_exclusive_inclusive"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._range_term(None), false);
use_item!(&mut node, self.expect("["), false);
use_item!(&mut node, self.expect("."), true);
use_item!(&mut node, self.expect("."), true);
use_item!(&mut node, self._range_term(None), false);
use_item!(&mut node, self.expect("]"), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _range_exclusive_exclusive(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("range_exclusive_exclusive"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._range_term(None), false);
use_item!(&mut node, self.expect("["), false);
use_item!(&mut node, self.expect("."), true);
use_item!(&mut node, self.expect("."), true);
use_item!(&mut node, self.expect("]"), false);
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
				name: Cow::Borrowed("range_term"),
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
				name: Cow::Borrowed("no_op"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect("-"));
use_item!(&mut node, self._atom(None), false);
use_negative_lookahead!(self, self.tokens.index(), &mut self.expect("."));
use_negative_lookahead!(self, self.tokens.index(), &mut self._operator(None));
use_negative_lookahead!(self, self.tokens.index(), &mut self._keyword_operator(None));

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _operation(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("operation"),
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
				name: Cow::Borrowed("binary_op"),
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
				name: Cow::Borrowed("compare_op"),
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
				name: Cow::Borrowed("compare_suffix"),
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
				name: Cow::Borrowed("add"),
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
				name: Cow::Borrowed("add_suffix"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("+"), false);
use_item!(&mut node, self._add(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("-"), false);
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
				name: Cow::Borrowed("multiply"),
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
				name: Cow::Borrowed("multiply_suffix"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("*"), false);
use_item!(&mut node, self._multiply(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("/"), false);
use_item!(&mut node, self._multiply(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("%"), false);
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
				name: Cow::Borrowed("exponent"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._atom(None), false);
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
				name: Cow::Borrowed("exponent_suffix"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("^"), false);
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
				name: Cow::Borrowed("unary_op"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("-"), false);
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
				name: Cow::Borrowed("break"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("break"), false);
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
				name: Cow::Borrowed("return"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("return"), false);
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
				name: Cow::Borrowed("continue"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("continue"), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _boolean(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("boolean"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("true"), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("false"), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _none(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("none"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("none"), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _identifier(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("identifier"),
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
				name: Cow::Borrowed("char"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("\\"), false);
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
				name: Cow::Borrowed("string"),
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
				name: Cow::Borrowed("decimal"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect_number(None), false);
use_item!(&mut node, self.expect("."), true);
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
				name: Cow::Borrowed("number"),
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
				name: Cow::Borrowed("require_term"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self._term(None), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect_tag("<e 'missing expression'>"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _closing_paren(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("closing_paren"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect(")"), true);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect_tag("<e 'missing closing parenthesis'>"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _closing_brace(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("closing_brace"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("}"), true);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect_tag("<e 'missing closing brace'>"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _closing_bracket(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("closing_bracket"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("]"), true);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect_tag("<e 'missing closing bracket'>"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _keyword(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("keyword"),
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
				name: Cow::Borrowed("self"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("self"), false);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _reserved_word(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("reserved_word"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("for"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("while"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("else"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("if"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("mut"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("loop"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("box"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("break"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("return"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("continue"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("type"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("log"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("logs"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("log_line"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("logs_line"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("use"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("trait"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("self"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("array"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("fun"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("map"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _keyword_variable(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("keyword_variable"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("let"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("const"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _keyword_operator(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("keyword_operator"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("in"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("is"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("and"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("not"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("or"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("as"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("istype"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect(">"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect(">"), false);
use_item!(&mut node, self.expect("="), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("<"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
loop { 
use_item!(&mut node, self.expect("<"), false);
use_item!(&mut node, self.expect("="), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _arrow(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("arrow"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("="), true);
use_item!(&mut node, self.expect(">"), true);

			return Some((node, false));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _comment_delimiter(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("comment_delimiter"),
				tokens: vec![],
				children: vec![],
			};
			loop { 
use_item!(&mut node, self.expect("*"), false);
use_item!(&mut node, self.expect("*"), false);
use_item!(&mut node, self.expect("*"), false);

			return Some((node, true));
		
}
		self.reset_parse(&mut node, pos);
			None
		}
		
		pub fn _comment_content(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {
				name: Cow::Borrowed("comment_content"),
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
				name: Cow::Borrowed("comment"),
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


fn node_from_token<'src>(token: Token, name: &'src str) -> Node<'src> {
    Node { 
        name: Cow::Borrowed(name),
        tokens: vec![token], 
        children: vec![] 
    }
}