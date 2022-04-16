#![allow(clippy::all)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
use crate::{
    is_identifier, is_number, is_string, is_uppercase, BiDirectionalIterator, Token, Tokenizer,
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
                return Some((
                    node_from_token(self.tokens.next().unwrap(), rule_name),
                    true,
                ));
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
                        current_token.clone(),
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

    crate::parse! { self, _file, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._expression(), false);
    use_repeat!(&mut node, self._expression(), false);
    }}
            }}

    crate::parse! { self, _expression, node, pos, {
        crate::all_of! { self, node, pos, false, {
            use_item!(&mut node, self._expression_item(), false);
        }}
        crate::all_of! { self, node, pos, false, {
            use_item!(&mut node, self._comment(), false);
        }}
    }}

    crate::parse! { self, _expression_item, node, pos, {
        crate::all_of! { self, node, pos, false, {
    		use_item!(&mut node, self._loop(), false);
    	}}
    	crate::all_of! { self, node, pos, false, {
    		use_item!(&mut node, self._block(), false);
    	}}
    	crate::all_of! { self, node, pos, false, {
    		use_item!(&mut node, self._type_definition(), false);
    	}}
    	crate::all_of! { self, node, pos, false, {
    		use_item!(&mut node, self._declaration(), false);
    	}}
    	crate::all_of! { self, node, pos, false, {
    		use_item!(&mut node, self._symbol_declaration(), false);
    	}}
    	crate::all_of! { self, node, pos, false, {
    		use_item!(&mut node, self._typed_declaration(), false);
    	}}
    	crate::all_of! { self, node, pos, false, {
    		use_item!(&mut node, self._function(), false);
    	}}
    	crate::all_of! { self, node, pos, false, {
    		use_item!(&mut node, self._internal_property_assignment(), false);
    	}}
    	crate::all_of! { self, node, pos, false, {
    		use_item!(&mut node, self._property_assignment(), false);
    	}}
    	crate::all_of! { self, node, pos, false, {
    		use_item!(&mut node, self._assignment(), false);
    	}}
    	crate::all_of! { self, node, pos, false, {
    		use_item!(&mut node, self._term(), false);
    	}}
    }}

    crate::parse! { self, _loop, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._for_loop(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._while_loop(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._infinite_loop(), false);
    }}
            }}

    crate::parse! { self, _for_loop, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("for".to_string()), false);
    use_item!(&mut node, self._parameters(), false);
    use_item!(&mut node, self.expect("in".to_string()), true);
    use_item!(&mut node, self._term(), false);
    use_item!(&mut node, self._block(), false);
    }}
            }}

    crate::parse! { self, _while_loop, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("while".to_string()), false);
    use_item!(&mut node, self._expression_item(), false);
    use_item!(&mut node, self._block(), false);
    }}
            }}

    crate::parse! { self, _infinite_loop, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("loop".to_string()), false);
    use_item!(&mut node, self._block(), false);
    }}
            }}

    crate::parse! { self, _block, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._if_block(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._brace_block(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._arrow_block(), false);
    }}
            }}

    crate::parse! { self, _if_block, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("if".to_string()), true);
    use_item!(&mut node, self._expression(), false);
    use_item!(&mut node, self._block(), false);
    use_repeat!(&mut node, self._else_if_block(), false);
    use_optional!(&mut node, self._else_block(), false);
    }}
            }}

    crate::parse! { self, _else_if_block, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("else".to_string()), true);
    use_item!(&mut node, self.expect("if".to_string()), true);
    use_item!(&mut node, self._expression(), false);
    use_item!(&mut node, self._block(), false);
    }}
            }}

    crate::parse! { self, _else_block, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("else".to_string()), true);
    use_item!(&mut node, self._block(), false);
    }}
            }}

    crate::parse! { self, _brace_block, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("{".to_string()), true);
    use_repeat!(&mut node, self._expression(), false);
    use_item!(&mut node, self._closing_brace(), false);
    }}
            }}

    crate::parse! { self, _arrow_block, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._arrow(), true);
    use_item!(&mut node, self._require_expression(), false);
    }}
            }}

    crate::parse! { self, _declaration, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._symbol_declaration(), false);
    use_item!(&mut node, self._equal(), true);
    use_item!(&mut node, self._expression_item(), false);
    }}
            }}

    crate::parse! { self, _symbol_declaration, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._keyword_variable(), false);
    use_optional!(&mut node, self.expect("mut".to_string()), false);
    use_item!(&mut node, self._identifier(), false);
    }}
            }}

    crate::parse! { self, _assignment, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._identifier(), false);
    use_item!(&mut node, self._equal(), false);
    use_item!(&mut node, self._expression(), false);
    }}
            }}

    crate::parse! { self, _internal_property_assignment, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._internal_property_access(), false);
    use_item!(&mut node, self._equal(), false);
    use_item!(&mut node, self._expression(), false);
    }}
            }}

    crate::parse! { self, _property_assignment, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._property_access(), false);
    use_item!(&mut node, self._equal(), false);
    use_item!(&mut node, self._expression(), false);
    }}
            }}

    crate::parse! { self, _type_definition, node, pos, {
        crate::all_of! { self, node, pos, false, {
    		use_item!(&mut node, self.expect("type".to_string()), false);
    		use_item!(&mut node, self._identifier(), false);
    		use_item!(&mut node, self.expect("[".to_string()), true);
    		crate::all_of! { self, node, pos, false, {
                use_item!(&mut node, self._type_field(), false);
				use_item!(&mut node, self.expect(",".to_string()), false);
			}}
			use_item!(&mut node, self._closing_bracket(), false);
    	}}
    }}

    crate::parse! { self, _type_field, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._type(), false);
    use_item!(&mut node, self._identifier(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._identifier(), false);
    use_item!(&mut node, self._identifier(), false);
    }}
            }}

    crate::parse! { self, _typed_declaration, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect_word(None), false);
    use_optional!(&mut node, self.expect("mut".to_string()), false);
    use_item!(&mut node, self._identifier(), false);
    crate::all_of! { self, node, pos, false, {
                        use_item!(&mut node, self._equal(), true);

    use_item!(&mut node, self._expression_item(), false);

                        break
                    }}}}
            }}

    crate::parse! { self, _equal, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("=".to_string()), false);
    use_negative_lookahead!(self, self.tokens.index(), &mut self.expect(">".to_string()));
    }}
            }}

    crate::parse! { self, _term, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._log(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._structure(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._operation(), false);
    }}
            }}

    crate::parse! { self, _structure, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._dictionary(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._list(), false);
    }}
            }}

    crate::parse! { self, _value, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._group(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._function_call(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._internal_property_access(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._property_access(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._boolean(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._none(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._string(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._decimal(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._number(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._identifier(), false);
    }}
            }}

    crate::parse! { self, _group, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("(".to_string()), true);
    use_item!(&mut node, self._expression_item(), false);
    use_item!(&mut node, self._closing_paren(), false);
    }}
            }}

    crate::parse! { self, _function, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._parameters(), false);
    use_item!(&mut node, self._block(), false);
    }}
            }}

    crate::parse! { self, _parameters, node, pos, {
                crate::all_of! { self, node, pos, false, {
    crate::all_of! { self, node, pos, false, {
                        use_item!(&mut node, self._identifier(), false);

    use_item!(&mut node, self.expect(",".to_string()), true);


                    }}}}
            }}

    crate::parse! { self, _log, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._log_keyword(), false);
    use_item!(&mut node, self.expect("(".to_string()), true);
    use_item!(&mut node, self._arguments(), false);
    use_item!(&mut node, self._closing_paren(), false);
    }}
            }}

    crate::parse! { self, _function_call, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._identifier(), false);
    use_item!(&mut node, self.expect("(".to_string()), true);
    use_item!(&mut node, self._arguments(), false);
    use_item!(&mut node, self._closing_paren(), false);
    }}
            }}

    crate::parse! { self, _arguments, node, pos, {
                crate::all_of! { self, node, pos, false, {
    crate::all_of! { self, node, pos, false, {
                        use_item!(&mut node, self._term(), false);

    use_item!(&mut node, self.expect(",".to_string()), true);


                    }}}}
            }}

    crate::parse! { self, _log_keyword, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("log".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("logs".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("log_line".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("logs_line".to_string()), false);
    }}
            }}

    crate::parse! { self, _list, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("[".to_string()), false);
    crate::all_of! { self, node, pos, false, {
                        use_item!(&mut node, self._expression_item(), false);

    use_negative_lookahead!(self, self.tokens.index(), &mut self.expect("]".to_string()));

    use_item!(&mut node, self.expect(",".to_string()), false);


                    }}use_item!(&mut node, self.expect("]".to_string()), false);
    }}
            }}

    crate::parse! { self, _dictionary, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("[".to_string()), false);
    crate::all_of! { self, node, pos, false, {
                        use_item!(&mut node, self._dictionary_entry(), false);

    use_negative_lookahead!(self, self.tokens.index(), &mut self.expect("]".to_string()));

    use_item!(&mut node, self.expect(",".to_string()), false);


                    }}use_item!(&mut node, self.expect("]".to_string()), false);
    }}
            }}

    crate::parse! { self, _dictionary_entry, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._term(), false);
    use_item!(&mut node, self.expect(":".to_string()), false);
    use_item!(&mut node, self._expression_item(), false);
    }}
            }}

    crate::parse! { self, _internal_property_access, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("self".to_string()), false);
    use_item!(&mut node, self.expect(".".to_string()), false);
    use_item!(&mut node, self._require_property(), false);
    }}
            }}

    crate::parse! { self, _property_access, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._property_accessor(), false);
    use_item!(&mut node, self.expect(".".to_string()), false);
    use_item!(&mut node, self._require_property(), false);
    }}
            }}

    crate::parse! { self, _property_accessor, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._structure(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._function_call(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._identifier(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("self".to_string()), false);
    }}
            }}

    crate::parse! { self, _require_property, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._value(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect_tag("<e 'missing index or property name'>".to_string()), false);
    }}
            }}

    crate::parse! { self, _operation, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._binary_op(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._unary_op(), false);
    }}
            }}

    crate::parse! { self, _binary_op, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._compare_op(), false);
    }}
            }}

    crate::parse! { self, _compare_op, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._add(), false);
    use_item!(&mut node, self._keyword_operator(), false);
    use_item!(&mut node, self._compare_op(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._add(), false);
    }}
            }}

    crate::parse! { self, _add, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._multiply(), false);
    use_item!(&mut node, self.expect("+".to_string()), false);
    use_item!(&mut node, self._add(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._multiply(), false);
    use_item!(&mut node, self.expect("-".to_string()), false);
    use_item!(&mut node, self._add(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._multiply(), false);
    }}
            }}

    crate::parse! { self, _multiply, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._exponent(), false);
    use_item!(&mut node, self.expect("*".to_string()), false);
    use_item!(&mut node, self._multiply(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._exponent(), false);
    use_item!(&mut node, self.expect("/".to_string()), false);
    use_item!(&mut node, self._multiply(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._exponent(), false);
    }}
            }}

    crate::parse! { self, _exponent, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._value(), false);
    use_item!(&mut node, self.expect("^".to_string()), false);
    use_item!(&mut node, self._exponent(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._value(), false);
    }}
            }}

    crate::parse! { self, _unary_op, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._unary_operator(), false);
    use_item!(&mut node, self._require_expression(), false);
    }}
            }}

    crate::parse! { self, _unary_operator, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("+".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("-".to_string()), false);
    }}
            }}

    crate::parse! { self, _boolean, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("true".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("false".to_string()), false);
    }}
            }}

    crate::parse! { self, _none, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("none".to_string()), false);
    }}
            }}

    crate::parse! { self, _identifier, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_negative_lookahead!(self, self.tokens.index(), &mut self._keyword());
    use_item!(&mut node, self.expect_word(None), false);
    }}
            }}

    crate::parse! { self, _string, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect_string(None), false);
    }}
            }}

    crate::parse! { self, _decimal, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect_number(None), false);
    use_item!(&mut node, self.expect(".".to_string()), true);
    use_item!(&mut node, self.expect_number(None), false);
    }}
            }}

    crate::parse! { self, _number, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect_number(None), false);
    }}
            }}

    crate::parse! { self, _require_expression, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._expression_item(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect_tag("<e 'missing expression'>".to_string()), false);
    }}
            }}

    crate::parse! { self, _closing_paren, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect(")".to_string()), true);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect_tag("<e 'missing closing parenthesis'>".to_string()), false);
    }}
            }}

    crate::parse! { self, _closing_brace, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("}".to_string()), true);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect_tag("<e 'missing closing brace'>".to_string()), false);
    }}
            }}

    crate::parse! { self, _closing_bracket, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("]".to_string()), true);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect_tag("<e 'missing closing bracket'>".to_string()), false);
    }}
            }}

    crate::parse! { self, _keyword, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._keyword_variable(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._keyword_operator(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._keyword_control(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._type(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._boolean(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._none(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("for".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("while".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("else".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("if".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("loop".to_string()), false);
    }}
            }}

    crate::parse! { self, _keyword_variable, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("let".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("const".to_string()), false);
    }}
            }}

    crate::parse! { self, _keyword_operator, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("in".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("is".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("and".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("not".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("or".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect(">".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect(">".to_string()), false);
    use_item!(&mut node, self.expect("=".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("<".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("<".to_string()), false);
    use_item!(&mut node, self.expect("=".to_string()), false);
    }}
            }}

    crate::parse! { self, _keyword_control, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("break".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("return".to_string()), false);
    }}
            }}

    crate::parse! { self, _arrow, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("=".to_string()), true);
    use_item!(&mut node, self.expect(">".to_string()), true);
    }}
            }}

    crate::parse! { self, _type, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._none(), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("bool".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("int".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("dec".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("string".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("fun".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("maybe".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("list".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("dict".to_string()), false);
    }}
    crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("tuple".to_string()), false);
    }}
            }}

    crate::parse! { self, _comment_delimiter, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect("*".to_string()), false);
    use_item!(&mut node, self.expect("*".to_string()), false);
    use_item!(&mut node, self.expect("*".to_string()), false);
    }}
            }}

    crate::parse! { self, _comment_content, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self.expect_any(None), false);
    use_negative_lookahead!(self, self.tokens.index(), &mut self._comment_delimiter());
    }}
            }}

    crate::parse! { self, _comment, node, pos, {
                crate::all_of! { self, node, pos, false, {
    use_item!(&mut node, self._comment_delimiter(), true);
    use_repeat!(&mut node, self._comment_content(), false);
    use_item!(&mut node, self.expect_any(None), false);
    use_item!(&mut node, self._comment_delimiter(), true);
    }}
            }}
}

fn node_from_token(token: Token, name: String) -> Node {
    Node {
        name,
        tokens: vec![token],
        children: vec![],
    }
}
