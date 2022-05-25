#![allow(clippy::all)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
use crate::*;

type ParseFun = dyn Fn(&mut TypesParser) -> Option<(ParseNode, bool)>;

pub struct TypesParser {
    pub tokens: BiDirectionalIterator<Token>,
	pub file: usize,
}

impl Parser for TypesParser {
    fn tokens(&mut self) -> &mut BiDirectionalIterator<Token> {
        &mut self.tokens
    }
	fn file(&self) -> usize { self.file }
}

impl TypesParser {
    pub fn new(source: String, file: usize) -> Self {
        Self { tokens: Self::build(source, file), file }
    }
	pub fn update(&mut self, source: String, file: usize) {
		self.tokens = Self::build(source, file);
		self.file = file;
	}

    fn _builtin_type(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("builtin_type");

        alt! {
                        (self, false, node, pos) {
                            once!(&mut node, self.expect("type"), false);
        once!(&mut node, self.expect_word(), false);
        use_negative_lookahead!(self, self.tokens().index(), self.expect("="));
                        }
                    };
        None
    }

    fn _type_symbol(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("type_symbol");

        alt! {
                        (self, true, node, pos) {
                            once!(&mut node, self.expect("type"), false);
        optional!(&mut node, self._generics(), false);
        once!(&mut node, self._identifier(), false);
                        }
                    };
        None
    }

    fn _struct_type_definition(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("struct_type_definition");

        alt! {
                        (self, false, node, pos) {
                            once!(&mut node, self._type_symbol(), false);
        once!(&mut node, self._equal(), true);
        once!(&mut node, self._bracket_list(&Self::_struct_member), false);
                        }
                    };
        None
    }

    fn _struct_member(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("struct_member");

        alt! {
                        (self, false, node, pos) {
                            once!(&mut node, self._type_hint(), false);
        once!(&mut node, self._identifier(), false);
                        }
                    };
        None
    }

    fn _sum_type_definition(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("sum_type_definition");

        alt! {
                        (self, false, node, pos) {
                            once!(&mut node, self._type_symbol(), false);
        once!(&mut node, self._equal(), true);
        once!(&mut node, self._bracket_list(&Self::_sum_member), false);
                        }
                    };
        None
    }

    fn _sum_member(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("sum_member");

        alt! {
            (self, false, node, pos) {
                once!(&mut node, self._type_hint(), false);
            }
        };
        None
    }

    fn _alias_type_definition(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("alias_type_definition");

        alt! {
                        (self, false, node, pos) {
                            once!(&mut node, self._type_symbol(), false);
        once!(&mut node, self._equal(), true);
        once!(&mut node, self._type_hint(), false);
                        }
                    };
        None
    }

    fn _generics(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("generics");

        alt! {
                        (self, false, node, pos) {
                            once!(&mut node, self.expect("["), false);
        once!(&mut node, self._list(&Self::_generic_argument), false);
        once!(&mut node, self._closing_bracket(), false);
                        }
                    };
        None
    }

    fn _generic_argument(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("generic_argument");

        alt! {
                        (self, false, node, pos) {
                            once!(&mut node, self._identifier(), false);

                        group! {
                            (self, false, node, pos) {
                                once!(&mut node, self.expect(":"), false);
        once!(&mut node, self._type_hint(), false);
                                break;
                            }
                        };

                        }
                    };
        None
    }

    fn _type_hint(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("type_hint");

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self._array_type_hint(), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self._function_type_hint(), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self._map_type_hint(), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self._struct_type_hint(), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self._primitive_type_hint(), false);
            }
        };
        None
    }

    fn _array_type_hint(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("array_type_hint");

        alt! {
                        (self, false, node, pos) {
                            once!(&mut node, self._array_kw(), false);
        once!(&mut node, self._bracket(&Self::_type_hint), false);
                        }
                    };
        None
    }

    fn _map_type_hint(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("map_type_hint");

        alt! {
                        (self, false, node, pos) {
                            once!(&mut node, self._map_kw(), false);
        once!(&mut node, self._bracket(&Self::_map_type_hint_inner), false);
                        }
                    };
        None
    }

    fn _map_type_hint_inner(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("map_type_hint_inner");

        alt! {
                        (self, true, node, pos) {
                            once!(&mut node, self._type_hint(), false);
        once!(&mut node, self.expect(","), false);
        once!(&mut node, self._type_hint(), false);
                        }
                    };
        None
    }

    fn _function_type_hint(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("function_type_hint");

        alt! {
                        (self, false, node, pos) {
                            once!(&mut node, self._fun_kw(), false);
        once!(&mut node, self._bracket(&Self::_type_hint), false);
                        }
                    };
        None
    }

    fn _primitive_type_hint(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("primitive_type_hint");

        alt! {
            (self, false, node, pos) {
                once!(&mut node, self._identifier(), false);
            }
        };
        None
    }

    fn _array_kw(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("array_kw");

        alt! {
            (self, false, node, pos) {
                once!(&mut node, self.expect("array"), false);
            }
        };
        None
    }

    fn _map_kw(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("map_kw");

        alt! {
            (self, false, node, pos) {
                once!(&mut node, self.expect("map"), false);
            }
        };
        None
    }

    fn _fun_kw(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("fun_kw");

        alt! {
            (self, false, node, pos) {
                once!(&mut node, self.expect("fun"), false);
            }
        };
        None
    }

    fn _struct_type_hint(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("struct_type_hint");

        alt! {
                        (self, false, node, pos) {
                            once!(&mut node, self._identifier(), false);
        once!(&mut node, self._bracket_list(&Self::_struct_member_type_hint), false);
                        }
                    };
        None
    }

    fn _struct_member_type_hint(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("struct_member_type_hint");

        alt! {
                        (self, false, node, pos) {
                            once!(&mut node, self._identifier(), false);
        once!(&mut node, self.expect(":"), false);
        once!(&mut node, self._type_hint(), false);
                        }
                    };
        None
    }

    fn _equal(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("equal");

        alt! {
                        (self, false, node, pos) {
                            once!(&mut node, self.expect("="), false);
        use_negative_lookahead!(self, self.tokens().index(), self.expect(">"));
                        }
                    };
        None
    }

    fn _list(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("list");

        alt! {
                        (self, true, node, pos) {

                        group! {
                            (self, false, node, pos) {
                                once!(&mut node, inner(self), false);
        once!(&mut node, self.expect(","), false);

                            }
                        };

        optional!(&mut node, inner(self), false);
                        }
                    };
        None
    }

    fn _paren(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("paren");

        alt! {
                        (self, true, node, pos) {
                            once!(&mut node, self.expect("("), false);
        once!(&mut node, inner(self), false);
        once!(&mut node, self._closing_paren(), false);
                        }
                    };
        None
    }

    fn _paren_list(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("paren_list");

        alt! {
                        (self, true, node, pos) {
                            once!(&mut node, self.expect("("), false);
        once!(&mut node, self._list(inner), false);
        once!(&mut node, self._closing_paren(), false);
                        }
                    };
        None
    }

    fn _brace(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("brace");

        alt! {
                        (self, true, node, pos) {
                            once!(&mut node, self.expect("{"), false);
        once!(&mut node, inner(self), false);
        once!(&mut node, self._closing_brace(), false);
                        }
                    };
        None
    }

    fn _bracket(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("bracket");

        alt! {
                        (self, true, node, pos) {
                            once!(&mut node, self.expect("["), false);
        once!(&mut node, inner(self), false);
        once!(&mut node, self._closing_bracket(), false);
                        }
                    };
        None
    }

    fn _bracket_list(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("bracket_list");

        alt! {
                        (self, true, node, pos) {
                            once!(&mut node, self.expect("["), false);
        once!(&mut node, self._list(inner), false);
        once!(&mut node, self._closing_bracket(), false);
                        }
                    };
        None
    }

    fn _closing_paren(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("closing_paren");

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect(")"), true);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect_tag("<e 'missing closing parenthesis'>"), false);
            }
        };
        None
    }

    fn _closing_brace(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("closing_brace");

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("}"), true);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect_tag("<e 'missing closing brace'>"), false);
            }
        };
        None
    }

    fn _closing_bracket(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("closing_bracket");

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("]"), true);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect_tag("<e 'missing closing bracket'>"), false);
            }
        };
        None
    }

    fn _identifier(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("identifier");

        alt! {
                        (self, false, node, pos) {
                            use_negative_lookahead!(self, self.tokens().index(), self._reserved_word());
        once!(&mut node, self.expect_word(), false);
                        }
                    };
        None
    }

    fn _reserved_word(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("reserved_word");

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("for"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("while"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("else"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("if"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("mut"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("loop"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("box"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("break"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("return"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("continue"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("type"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("log"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("logs"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("log_line"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("logs_line"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("use"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("with"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("trait"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("self"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("array"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("fun"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("map"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("let"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("const"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("in"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("is"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("and"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("not"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("or"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("as"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("istype"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("self"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("true"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("false"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("none"), false);
            }
        };

        alt! {
            (self, true, node, pos) {
                once!(&mut node, self.expect("package"), false);
            }
        };
        None
    }
}
