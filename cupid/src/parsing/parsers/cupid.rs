#![allow(clippy::all)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
use crate::*;

type ParseFun = dyn Fn(&mut CupidParser) -> Option<(ParseNode, bool)>;

pub struct CupidParser {
    pub tokens: BiDirectionalIterator<Token>,
}

impl Parser for CupidParser {
    fn tokens(&mut self) -> &mut BiDirectionalIterator<Token> {
        &mut self.tokens
    }
}

impl CupidParser {
    pub fn new(source: String) -> Self {
        Self {
            tokens: Self::build(source),
        }
    }

    pub fn _file(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("file");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._expression(), false);
        repeat!(&mut node, self._expression(), false);
                    });
        None
    }

    pub fn _expression(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("expression");

        alt! ((self, true, node, pos) {
            once!(&mut node, self._package(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._comment(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._statement(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._term(), false);
        });
        None
    }

    pub fn _statement(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("statement");

        alt! ((self, true, node, pos) {
            once!(&mut node, self._type_definition(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._trait_definition(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._implement_type(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._implement_trait(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._typed_declaration(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._break(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._return(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._continue(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._op_assignment(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._assignment(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._property_assignment(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._property_op_assignment(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._log(), false);
        });
        None
    }

    pub fn _term(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("term");

        alt! ((self, true, node, pos) {
            once!(&mut node, self._loop(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._block(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._function(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._no_op(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._property_access(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._operation(), false);
        });
        None
    }

    pub fn _loop(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("loop");

        alt! ((self, true, node, pos) {
            once!(&mut node, self._for_loop(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._while_loop(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._infinite_loop(), false);
        });
        None
    }

    pub fn _for_loop(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("for_loop");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("for"), false);
        once!(&mut node, self._for_loop_parameters(), false);
        once!(&mut node, self.expect("in"), true);
        once!(&mut node, self._term(), false);
        once!(&mut node, self._block(), false);
                    });
        None
    }

    pub fn _for_loop_parameters(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("for_loop_parameters");

        alt! ((self, false, node, pos) {
            once!(&mut node, self._list(&Self::_identifier), false);
        });
        None
    }

    pub fn _while_loop(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("while_loop");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("while"), false);
        once!(&mut node, self._term(), false);
        once!(&mut node, self._block(), false);
                    });
        None
    }

    pub fn _infinite_loop(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("infinite_loop");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("loop"), false);
        once!(&mut node, self._block(), false);
                    });
        None
    }

    pub fn _block(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("block");

        alt! ((self, false, node, pos) {
            once!(&mut node, self._if_block(), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self._box_block(), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self._brace_block(), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self._arrow_block(), false);
        });
        None
    }

    pub fn _if_block(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("if_block");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("if"), true);
        once!(&mut node, self._term(), false);
        once!(&mut node, self._block(), false);
        repeat!(&mut node, self._else_if_block(), false);
        optional!(&mut node, self._else_block(), false);
                    });
        None
    }

    pub fn _else_if_block(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("else_if_block");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("else"), true);
        once!(&mut node, self.expect("if"), true);
        once!(&mut node, self._term(), false);
        once!(&mut node, self._block(), false);
                    });
        None
    }

    pub fn _else_block(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("else_block");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("else"), true);
        once!(&mut node, self._block(), false);
                    });
        None
    }

    pub fn _box_block(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("box_block");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("box"), true);
        once!(&mut node, self._brace_block(), false);
                    });
        None
    }

    pub fn _brace_block(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("brace_block");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("{"), true);
        repeat!(&mut node, self._expression(), false);
        once!(&mut node, self._closing_brace(), false);
                    });
        None
    }

    pub fn _arrow_block(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("arrow_block");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self._arrow(), true);
        once!(&mut node, self._expression(), false);
                    });
        None
    }

    pub fn _assignment(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("assignment");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._identifier(), false);
        once!(&mut node, self._equal(), false);
        once!(&mut node, self._term(), false);
                    });
        None
    }

    pub fn _property_assignment(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("property_assignment");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._property_access(), false);
        once!(&mut node, self._equal(), false);
        once!(&mut node, self._term(), false);
                    });
        None
    }

    pub fn _property_op_assignment(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("property_op_assignment");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._property_access(), false);
        once!(&mut node, self._operator(), false);
        once!(&mut node, self._equal(), false);
        once!(&mut node, self._term(), false);
                    });
        None
    }

    pub fn _property_access(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("property_access");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._atom(), false);
        once!(&mut node, self.expect("."), false);
        once!(&mut node, self._term(), false);
                    });
        None
    }

    pub fn _op_assignment(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("op_assignment");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._identifier(), false);
        once!(&mut node, self._operator(), false);
        once!(&mut node, self._equal(), false);
        once!(&mut node, self._term(), false);
                    });
        None
    }

    pub fn _operator(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("operator");

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("+"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("-"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("*"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("/"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("^"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("%"), false);
        });
        None
    }

    pub fn _atom(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("atom");

        alt! ((self, true, node, pos) {
            once!(&mut node, self._empty(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._builtin_function_call(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._function_call(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._range(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._map(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._bracket_array(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._group(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._unary_op(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._boolean(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._none(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._string(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._char(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._decimal(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._number(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._self(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._identifier(), false);
        });
        None
    }

    pub fn _empty(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("empty");

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("_"), true);
        });
        None
    }

    pub fn _group(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("group");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("("), true);
        optional!(&mut node, self._term(), false);
        once!(&mut node, self._closing_paren(), false);
                    });
        None
    }

    pub fn _function(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("function");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._parameters(), false);
        once!(&mut node, self._function_body(), false);
                    });
        None
    }

    pub fn _function_body(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("function_body");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self._arrow(), false);
        once!(&mut node, self._empty(), true);
                    });

        alt! ((self, true, node, pos) {
                        once!(&mut node, self._arrow(), false);
        once!(&mut node, self._group(), false);
                    });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._block(), false);
        });
        None
    }

    pub fn _parameters(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("parameters");

        alt! ((self, false, node, pos) {
            once!(&mut node, self._list(&Self::_parameter), false);
        });
        None
    }

    pub fn _parameter(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("parameter");

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("_"), true);
        });

        alt! ((self, true, node, pos) {
                        optional!(&mut node, self.expect("mut"), false);
        once!(&mut node, self._self(), false);
                    });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._annotated_parameter(), false);
        });
        None
    }

    pub fn _annotated_parameter(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("annotated_parameter");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._type_hint(), false);
        once!(&mut node, self._identifier(), false);
                    });
        None
    }

    pub fn _log(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("log");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._log_keyword(), false);
        once!(&mut node, self._paren(&Self::_arguments), false);
                    });
        None
    }

    pub fn _builtin_function_call(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("builtin_function_call");

        alt! ((self, false, node, pos) {
            use_negative_lookbehind!(self, self.tokens().index(), self.expect("."));
        	once!(&mut node, self._builtin_function(), false);
        	once!(&mut node, self._paren(&Self::_arguments), false);
        });
        None
    }

    pub fn _function_call(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("function_call");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._identifier(), false);
        once!(&mut node, self._paren(&Self::_arguments), false);
                    });
        None
    }

    pub fn _arguments(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("arguments");

        alt! ((self, false, node, pos) {
            once!(&mut node, self._list(&Self::_term), false);
        });
        None
    }

    pub fn _log_keyword(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("log_keyword");

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("log"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("logs"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("log_line"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("logs_line"), false);
        });
        None
    }

    pub fn _bracket_array(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("bracket_array");

        alt! ((self, false, node, pos) {
            once!(&mut node, self._bracket_list(&Self::_term), false);
        });
        None
    }

    pub fn _map(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("map");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("["), false);
        once!(&mut node, self._list(&Self::_map_entry), false);
        once!(&mut node, self.expect("]"), false);
                    });
        None
    }

    pub fn _map_entry(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("map_entry");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._atom(), false);
        once!(&mut node, self.expect(":"), false);
        once!(&mut node, self._term(), false);
                    });
        None
    }

    pub fn _range(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("range");

        alt! ((self, false, node, pos) {
            once!(&mut node, self._range_inclusive_inclusive(), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self._range_inclusive_exclusive(), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self._range_exclusive_inclusive(), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self._range_exclusive_exclusive(), false);
        });
        None
    }

    pub fn _range_inclusive_inclusive(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("range_inclusive_inclusive");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("["), false);
        once!(&mut node, self._range_term(), false);
        once!(&mut node, self.expect("."), true);
        once!(&mut node, self.expect("."), true);
        once!(&mut node, self._range_term(), false);
        once!(&mut node, self.expect("]"), false);
                    });
        None
    }

    pub fn _range_inclusive_exclusive(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("range_inclusive_exclusive");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("["), false);
        once!(&mut node, self._range_term(), false);
        once!(&mut node, self.expect("."), true);
        once!(&mut node, self.expect("."), true);
        once!(&mut node, self.expect("]"), false);
        once!(&mut node, self._range_term(), false);
                    });
        None
    }

    pub fn _range_exclusive_inclusive(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("range_exclusive_inclusive");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._range_term(), false);
        once!(&mut node, self.expect("["), false);
        once!(&mut node, self.expect("."), true);
        once!(&mut node, self.expect("."), true);
        once!(&mut node, self._range_term(), false);
        once!(&mut node, self.expect("]"), false);
                    });
        None
    }

    pub fn _range_exclusive_exclusive(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("range_exclusive_exclusive");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._range_term(), false);
        once!(&mut node, self.expect("["), false);
        once!(&mut node, self.expect("."), true);
        once!(&mut node, self.expect("."), true);
        once!(&mut node, self.expect("]"), false);
        once!(&mut node, self._range_term(), false);
                    });
        None
    }

    pub fn _range_term(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("range_term");

        alt! ((self, true, node, pos) {
            once!(&mut node, self._function_call(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._group(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._unary_op(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._number(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._identifier(), false);
        });
        None
    }

    pub fn _no_op(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("no_op");

        alt! ((self, true, node, pos) {
                        use_negative_lookahead!(self, self.tokens().index(), self.expect("-"));
        once!(&mut node, self._atom(), false);
        use_negative_lookahead!(self, self.tokens().index(), self.expect("."));
        use_negative_lookahead!(self, self.tokens().index(), self._operator());
        use_negative_lookahead!(self, self.tokens().index(), self._keyword_operator());
                    });
        None
    }

    pub fn _operation(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("operation");

        alt! ((self, true, node, pos) {
            once!(&mut node, self._binary_op(), false);
        });
        None
    }

    pub fn _binary_op(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("binary_op");

        alt! ((self, true, node, pos) {
            once!(&mut node, self._type_cast(), false);
        });
        None
    }

    pub fn _type_cast(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("type_cast");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._compare_op(), false);
        optional!(&mut node, self._type_cast_suffix(), false);
                    });
        None
    }

    pub fn _type_cast_suffix(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("type_cast_suffix");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("as"), false);
        once!(&mut node, self._type_hint(), false);
                    });
        None
    }

    pub fn _compare_op(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("compare_op");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._add(), false);
        optional!(&mut node, self._compare_suffix(), false);
                    });
        None
    }

    pub fn _compare_suffix(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("compare_suffix");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self._keyword_operator(), false);
        once!(&mut node, self._compare_op(), false);
                    });
        None
    }

    pub fn _add(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("add");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._multiply(), false);
        optional!(&mut node, self._add_suffix(), false);
                    });
        None
    }

    pub fn _add_suffix(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("add_suffix");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("+"), false);
        once!(&mut node, self._add(), false);
                    });

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("-"), false);
        once!(&mut node, self._add(), false);
                    });
        None
    }

    pub fn _multiply(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("multiply");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._exponent(), false);
        optional!(&mut node, self._multiply_suffix(), false);
                    });
        None
    }

    pub fn _multiply_suffix(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("multiply_suffix");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("*"), false);
        once!(&mut node, self._multiply(), false);
                    });

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("/"), false);
        once!(&mut node, self._multiply(), false);
                    });

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("%"), false);
        once!(&mut node, self._multiply(), false);
                    });
        None
    }

    pub fn _exponent(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("exponent");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._atom(), false);
        optional!(&mut node, self._exponent_suffix(), false);
                    });
        None
    }

    pub fn _exponent_suffix(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("exponent_suffix");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("^"), false);
        once!(&mut node, self._exponent(), false);
                    });
        None
    }

    pub fn _unary_op(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("unary_op");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("-"), false);
        once!(&mut node, self._atom(), false);
                    });
        None
    }

    pub fn _break(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("break");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("break"), false);
        optional!(&mut node, self._term(), false);
                    });
        None
    }

    pub fn _return(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("return");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("return"), false);
        optional!(&mut node, self._term(), false);
                    });
        None
    }

    pub fn _continue(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("continue");

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("continue"), false);
        });
        None
    }

    pub fn _boolean(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("boolean");

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("true"), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("false"), false);
        });
        None
    }

    pub fn _none(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("none");

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("none"), false);
        });
        None
    }

    pub fn _char(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("char");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("\\"), false);
        once!(&mut node, self.expect_letter(), false);
                    });
        None
    }

    pub fn _string(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("string");

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect_string(), false);
        });
        None
    }

    pub fn _decimal(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("decimal");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect_number(), false);
        once!(&mut node, self.expect("."), true);
        once!(&mut node, self.expect_number(), false);
                    });
        None
    }

    pub fn _number(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("number");

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect_number(), false);
        });
        None
    }

    pub fn _self(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("self");

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("self"), false);
        });
        None
    }

    pub fn _keyword_operator(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("keyword_operator");

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("in"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("is"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("and"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("not"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("or"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("as"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("istype"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect(">"), false);
        });

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect(">"), false);
        once!(&mut node, self.expect("="), false);
                    });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("<"), false);
        });

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("<"), false);
        once!(&mut node, self.expect("="), false);
                    });
        None
    }

    pub fn _arrow(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("arrow");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("="), true);
        once!(&mut node, self.expect(">"), true);
                    });
        None
    }

    pub fn _comment_delimiter(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("comment_delimiter");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("*"), false);
        once!(&mut node, self.expect("*"), false);
        once!(&mut node, self.expect("*"), false);
                    });
        None
    }

    pub fn _comment_content(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("comment_content");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect_any(), false);
        use_negative_lookahead!(self, self.tokens().index(), self._comment_delimiter());
                    });
        None
    }

    pub fn _comment(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("comment");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._comment_delimiter(), true);
        repeat!(&mut node, self._comment_content(), false);
        once!(&mut node, self.expect_any(), false);
        once!(&mut node, self._comment_delimiter(), true);
                    });
        None
    }

    pub fn _builtin_function(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("builtin_function");

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("split"), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("split_at"), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("replace"), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("replace_n"), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("char"), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("push"), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("pop"), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("remove"), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("insert"), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("length"), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("set"), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("get"), false);
        });

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("remove"), false);
        });
        None
    }

    pub fn _packages(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("packages");

        alt! ((self, false, node, pos) {
            repeat!(&mut node, self._package(), false);
        });
        None
    }

    pub fn _package(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("package");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("package"), false);
        optional!(&mut node, self._name_space(), false);
        once!(&mut node, self._items(), false);
                    });
        None
    }

    pub fn _name_space(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("name_space");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._item(), false);
        once!(&mut node, self.expect(":"), false);
        once!(&mut node, self.expect(":"), false);
        optional!(&mut node, self._name_space(), false);
                    });
        None
    }

    pub fn _items(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("items");

        alt! ((self, true, node, pos) {
            once!(&mut node, self._item_group(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._item(), false);
        });
        None
    }

    pub fn _item_group(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("item_group");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("["), false);

                        group! ((self, false, node, pos) {
                            once!(&mut node, self._item(), false);
        once!(&mut node, self.expect(","), false);

                        });

        once!(&mut node, self.expect("]"), false);
                    });
        None
    }

    pub fn _item(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("item");

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect_word(), false);
        });
        None
    }

    pub fn _objects(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("objects");

        alt! ((self, false, node, pos) {
            once!(&mut node, self._list(&Self::expect_word), false);
        });
        None
    }

    pub fn _type_definition(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("type_definition");

        alt! ((self, true, node, pos) {
            once!(&mut node, self._builtin_type_definition(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._struct_type_definition(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._sum_type_definition(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._alias_type_definition(), false);
        });
        None
    }

    pub fn _builtin_type_definition(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("builtin_type_definition");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("type"), false);
        once!(&mut node, self.expect_word(), false);
        use_negative_lookahead!(self, self.tokens().index(), self.expect("="));
                    });
        None
    }

    pub fn _struct_type_definition(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("struct_type_definition");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._type_symbol(), false);
        once!(&mut node, self._equal(), true);
        once!(&mut node, self._bracket_list(&Self::_struct_member), false);
                    });
        None
    }

    pub fn _struct_member(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("struct_member");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._type_hint(), false);
        once!(&mut node, self._identifier(), false);
                    });
        None
    }

    pub fn _sum_type_definition(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("sum_type_definition");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._type_symbol(), false);
        once!(&mut node, self._equal(), true);
        once!(&mut node, self._bracket_list(&Self::_sum_member), false);
                    });
        None
    }

    pub fn _sum_member(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("sum_member");

        alt! ((self, false, node, pos) {
            once!(&mut node, self._type_hint(), false);
        });
        None
    }

    pub fn _alias_type_definition(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("alias_type_definition");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._type_symbol(), false);
        once!(&mut node, self._equal(), true);
        once!(&mut node, self._type_hint(), false);
                    });
        None
    }

    pub fn _type_symbol(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("type_symbol");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("type"), false);
        optional!(&mut node, self._generics(), false);
        once!(&mut node, self._identifier(), false);
                    });
        None
    }

    pub fn _generics(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("generics");

        alt! ((self, false, node, pos) {
            once!(&mut node, self._bracket_list(&Self::_generic_argument), false);
        });
        None
    }

    pub fn _generic_argument(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("generic_argument");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._identifier(), false);
        use_negative_lookahead!(self, self.tokens().index(), self.expect(":"));
                    });

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._identifier(), false);
        once!(&mut node, self.expect(":"), false);
        once!(&mut node, self._type_hint(), false);
                    });
        None
    }

    pub fn _typed_declaration(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("typed_declaration");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._type_hint(), false);
        optional!(&mut node, self.expect("mut"), false);
        once!(&mut node, self._identifier(), false);
        once!(&mut node, self._equal(), true);
        once!(&mut node, self._term(), false);
                    });
        None
    }

    pub fn _type_hint(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("type_hint");

        alt! ((self, true, node, pos) {
            once!(&mut node, self._array_type_hint(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._function_type_hint(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._map_type_hint(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._struct_type_hint(), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self._primitive_type_hint(), false);
        });
        None
    }

    pub fn _array_type_hint(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("array_type_hint");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._array_kw(), false);
        once!(&mut node, self._bracket(&Self::_type_hint), false);
                    });
        None
    }

    pub fn _map_type_hint(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("map_type_hint");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._map_kw(), false);
        once!(&mut node, self._bracket_list(&Self::_type_hint), false);
                    });
        None
    }

    pub fn _function_type_hint(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("function_type_hint");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._fun_kw(), false);
        once!(&mut node, self._bracket(&Self::_type_hint), false);
                    });
        None
    }

    pub fn _primitive_type_hint(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("primitive_type_hint");

        alt! ((self, false, node, pos) {
            once!(&mut node, self._identifier(), false);
        });
        None
    }

    pub fn _array_kw(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("array_kw");

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("array"), false);
        });
        None
    }

    pub fn _map_kw(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("map_kw");

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("map"), false);
        });
        None
    }

    pub fn _fun_kw(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("fun_kw");

        alt! ((self, false, node, pos) {
            once!(&mut node, self.expect("fun"), false);
        });
        None
    }

    pub fn _struct_type_hint(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("struct_type_hint");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._identifier(), false);
        once!(&mut node, self._bracket_list(&Self::_struct_member_type_hint), false);
                    });
        None
    }

    pub fn _struct_member_type_hint(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("struct_member_type_hint");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self._identifier(), false);
        once!(&mut node, self.expect(":"), false);
        once!(&mut node, self._type_hint(), false);
                    });
        None
    }

    pub fn _implement_type(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("implement_type");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("use"), false);
        optional!(&mut node, self._generics(), false);
        once!(&mut node, self._type_hint(), false);
        once!(&mut node, self._brace(&Self::_declarations), false);
                    });
        None
    }

    pub fn _implement_trait(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("implement_trait");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("use"), false);
        optional!(&mut node, self._generics(), false);
        once!(&mut node, self._identifier(), false);
        once!(&mut node, self.expect("with"), false);
        once!(&mut node, self._type_hint(), false);
        optional!(&mut node, self._implement_trait_body(), false);
                    });
        None
    }

    pub fn _implement_trait_body(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("implement_trait_body");

        alt! ((self, true, node, pos) {
            once!(&mut node, self._brace(&Self::_declarations), false);
        });
        None
    }

    pub fn _trait_definition(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("trait_definition");

        alt! ((self, false, node, pos) {
                        once!(&mut node, self.expect("trait"), false);
        once!(&mut node, self._generics(), false);
        once!(&mut node, self._identifier(), false);
        once!(&mut node, self._equal(), true);
        once!(&mut node, self._bracket_list(&Self::_typed_declaration), false);
                    });
        None
    }

    pub fn _equal(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("equal");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("="), false);
        use_negative_lookahead!(self, self.tokens().index(), self.expect(">"));
                    });
        None
    }

    pub fn _declarations(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("declarations");

        alt! ((self, true, node, pos) {
            repeat!(&mut node, self._typed_declaration(), false);
        });
        None
    }

    pub fn _list(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("list");

        alt! ((self, true, node, pos) {

                        group! ((self, false, node, pos) {
                            once!(&mut node, inner(self), false);
        once!(&mut node, self.expect(","), false);

                        });

        optional!(&mut node, inner(self), false);
                    });
        None
    }

    pub fn _paren(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("paren");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("("), false);
        once!(&mut node, inner(self), false);
        once!(&mut node, self._closing_paren(), false);
                    });
        None
    }

    pub fn _paren_list(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("paren_list");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("("), false);
        once!(&mut node, self._list(inner), false);
        once!(&mut node, self._closing_paren(), false);
                    });
        None
    }

    pub fn _brace(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("brace");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("{"), false);
        once!(&mut node, inner(self), false);
        once!(&mut node, self._closing_brace(), false);
                    });
        None
    }

    pub fn _bracket(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("bracket");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("["), false);
        once!(&mut node, inner(self), false);
        once!(&mut node, self._closing_bracket(), false);
                    });
        None
    }

    pub fn _bracket_list(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("bracket_list");

        alt! ((self, true, node, pos) {
                        once!(&mut node, self.expect("["), false);
        once!(&mut node, self._list(inner), false);
        once!(&mut node, self._closing_bracket(), false);
                    });
        None
    }

    pub fn _closing_paren(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("closing_paren");

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect(")"), true);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect_tag("<e 'missing closing parenthesis'>"), false);
        });
        None
    }

    pub fn _closing_brace(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("closing_brace");

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("}"), true);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect_tag("<e 'missing closing brace'>"), false);
        });
        None
    }

    pub fn _closing_bracket(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("closing_bracket");

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("]"), true);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect_tag("<e 'missing closing bracket'>"), false);
        });
        None
    }

    pub fn _identifier(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("identifier");

        alt! ((self, false, node, pos) {
                        use_negative_lookahead!(self, self.tokens().index(), self._reserved_word());
        once!(&mut node, self.expect_word(), false);
                    });
        None
    }

    pub fn _reserved_word(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("reserved_word");

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("for"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("while"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("else"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("if"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("mut"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("loop"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("box"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("break"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("return"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("continue"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("type"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("log"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("logs"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("log_line"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("logs_line"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("use"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("with"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("trait"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("self"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("array"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("fun"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("map"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("let"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("const"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("in"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("is"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("and"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("not"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("or"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("as"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("istype"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("self"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("true"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("false"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("none"), false);
        });

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("package"), false);
        });
        None
    }
}
