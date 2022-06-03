#![allow(clippy::all)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
use crate::*;

type ParseFun = dyn Fn(&mut CupidParser) -> Option<(ParseNode, bool)>;

#[derive(Debug, Clone, Default)]
pub struct CupidParser {
	pub tokens: BiDirectionalIterator<Token>,
	pub file: usize,
}

impl Parser for CupidParser {
	fn tokens(&mut self) -> &mut BiDirectionalIterator<Token> {
		&mut self.tokens
	}
	fn file(&self) -> usize { self.file }
}

impl CupidParser {
	pub fn new(source: String, file: usize) -> Self {
		Self { tokens: Self::build(source, file), file }
	}
	pub fn update(&mut self, source: String, file: usize) {
		self.tokens = Self::build(source, file);
		self.file = file;
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
				once!(&mut node, self._expression_item(), false);
			});
				None
			}
			
			pub fn _expression_item(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("expression_item");
				
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
				once!(&mut node, self._type_def(), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self._trait_def(), false);
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
				once!(&mut node, self.expect(r"for"), false);
once!(&mut node, self._for_loop_parameters(), false);
once!(&mut node, self.expect(r"in"), true);
once!(&mut node, self._operation(), false);
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
				once!(&mut node, self.expect(r"while"), false);
once!(&mut node, self._operation(), false);
once!(&mut node, self._block(), false);
			});
				None
			}
			
			pub fn _infinite_loop(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("infinite_loop");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"loop"), false);
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
				once!(&mut node, self.expect(r"if"), true);
once!(&mut node, self._operation(), false);
once!(&mut node, self._block(), false);
repeat!(&mut node, self._else_if_block(), false);
optional!(&mut node, self._else_block(), false);
			});
				None
			}
			
			pub fn _else_if_block(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("else_if_block");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"else"), true);
once!(&mut node, self.expect(r"if"), true);
once!(&mut node, self._operation(), false);
once!(&mut node, self._block(), false);
			});
				None
			}
			
			pub fn _else_block(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("else_block");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"else"), true);
once!(&mut node, self._block(), false);
			});
				None
			}
			
			pub fn _box_block(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("box_block");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"box"), true);
once!(&mut node, self._brace_block(), false);
			});
				None
			}
			
			pub fn _brace_block(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("brace_block");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"{"), true);
repeat!(&mut node, self._expression(), false);
once!(&mut node, self.expect(r"}"), false);
			});
				None
			}
			
			pub fn _arrow_block(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("arrow_block");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self._arrow(), true);
once!(&mut node, self._expression_item(), false);
			});
				None
			}
			
			pub fn _typed_declaration(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("typed_declaration");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._type_hint(), false);
optional!(&mut node, self.expect(r"mut"), false);
once!(&mut node, self._identifier(), false);

				group! ((self, false, node, pos) {
					once!(&mut node, self._equal(), true);
once!(&mut node, self._term(), false);
					break;
				});
			
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
				once!(&mut node, self.expect(r"+"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"-"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"*"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"/"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"^"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"%"), false);
			});
				None
			}
			
			pub fn _atom(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("atom");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self._empty(), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self._pointer(), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self._builtin_function_call(), false);
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
				once!(&mut node, self._identifier(), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self._boolean(), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self._none(), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self._regex(), false);
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
				None
			}
			
			pub fn _pointer(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("pointer");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"&"), false);
optional!(&mut node, self.expect(r"mut"), false);
once!(&mut node, self._identifier(), false);
			});
				None
			}
			
			pub fn _empty(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("empty");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"_"), true);
			});
				None
			}
			
			pub fn _group(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("group");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"("), true);
optional!(&mut node, self._term(), false);
once!(&mut node, self._closing_paren(), false);
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
				use_negative_lookbehind!(self, self.tokens().index(), self.expect(r"."));
once!(&mut node, self._builtin_function(), false);
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
				once!(&mut node, self.expect(r"log"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"logs"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"log_line"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"logs_line"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"debug"), false);
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
				once!(&mut node, self.expect(r"["), false);
once!(&mut node, self._list(&Self::_map_entry), false);
once!(&mut node, self.expect(r"]"), false);
			});
				None
			}
			
			pub fn _map_entry(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("map_entry");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._atom(), false);
once!(&mut node, self.expect(r":"), false);
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
				once!(&mut node, self.expect(r"["), false);
once!(&mut node, self._range_term(), false);
once!(&mut node, self.expect(r"."), true);
once!(&mut node, self.expect(r"."), true);
once!(&mut node, self._range_term(), false);
once!(&mut node, self.expect(r"]"), false);
			});
				None
			}
			
			pub fn _range_inclusive_exclusive(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("range_inclusive_exclusive");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"["), false);
once!(&mut node, self._range_term(), false);
once!(&mut node, self.expect(r"."), true);
once!(&mut node, self.expect(r"."), true);
once!(&mut node, self.expect(r"]"), false);
once!(&mut node, self._range_term(), false);
			});
				None
			}
			
			pub fn _range_exclusive_inclusive(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("range_exclusive_inclusive");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._range_term(), false);
once!(&mut node, self.expect(r"["), false);
once!(&mut node, self.expect(r"."), true);
once!(&mut node, self.expect(r"."), true);
once!(&mut node, self._range_term(), false);
once!(&mut node, self.expect(r"]"), false);
			});
				None
			}
			
			pub fn _range_exclusive_exclusive(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("range_exclusive_exclusive");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._range_term(), false);
once!(&mut node, self.expect(r"["), false);
once!(&mut node, self.expect(r"."), true);
once!(&mut node, self.expect(r"."), true);
once!(&mut node, self.expect(r"]"), false);
once!(&mut node, self._range_term(), false);
			});
				None
			}
			
			pub fn _range_term(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("range_term");
				
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
				use_negative_lookahead!(self, self.tokens().index(), self.expect(r"-"));
use_negative_lookahead!(self, self.tokens().index(), self.expect(r"not"));
once!(&mut node, self._atom(), false);
use_negative_lookahead!(self, self.tokens().index(), self.expect(r","));
use_negative_lookahead!(self, self.tokens().index(), self.expect(r"."));
use_negative_lookahead!(self, self.tokens().index(), self.expect(r"("));
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
				once!(&mut node, self._logic_op(), false);
			});
				None
			}
			
			pub fn _logic_op(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("logic_op");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._compare_op(), false);
optional!(&mut node, self._logic_op_suffix(), false);
			});
				None
			}
			
			pub fn _logic_op_suffix(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("logic_op_suffix");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"and"), false);
once!(&mut node, self._logic_op(), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"or"), false);
once!(&mut node, self._logic_op(), false);
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
				once!(&mut node, self.expect(r"+"), false);
once!(&mut node, self._add(), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"-"), false);
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
				once!(&mut node, self.expect(r"*"), false);
once!(&mut node, self._multiply(), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"/"), false);
once!(&mut node, self._multiply(), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"%"), false);
once!(&mut node, self._multiply(), false);
			});
				None
			}
			
			pub fn _exponent(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("exponent");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._type_cast(), false);
optional!(&mut node, self._exponent_suffix(), false);
			});
				None
			}
			
			pub fn _exponent_suffix(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("exponent_suffix");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"^"), false);
once!(&mut node, self._exponent(), false);
			});
				None
			}
			
			pub fn _type_cast(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("type_cast");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._property(), false);
optional!(&mut node, self._type_cast_suffix(), false);
			});
				None
			}
			
			pub fn _type_cast_suffix(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("type_cast_suffix");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"as"), false);
once!(&mut node, self._type_hint(), false);
			});
				None
			}
			
			pub fn _property(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("property");
				
			alt_inverse! ((self, false, node, pos) {
				once!(&mut node, self._function_call(), false);
optional!(&mut node, self._property_suffix(), false);
			});
				None
			}
			
			pub fn _property_suffix(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("property_suffix");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"."), false);
once!(&mut node, self._property(), false);
			});
				None
			}
			
			pub fn _function_call(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("function_call");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._atom(), false);
optional!(&mut node, self._function_call_suffix(), false);
			});
				None
			}
			
			pub fn _function_call_suffix(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("function_call_suffix");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"("), false);
once!(&mut node, self._arguments(), false);
once!(&mut node, self.expect(r")"), false);
			});
				None
			}
			
			pub fn _unary_op(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("unary_op");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"-"), false);
once!(&mut node, self._atom(), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"not"), false);
once!(&mut node, self._atom(), false);
			});
				None
			}
			
			pub fn _break(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("break");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"break"), false);
optional!(&mut node, self._term(), false);
			});
				None
			}
			
			pub fn _return(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("return");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"return"), false);
optional!(&mut node, self._term(), false);
			});
				None
			}
			
			pub fn _continue(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("continue");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"continue"), false);
			});
				None
			}
			
			pub fn _boolean(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("boolean");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"true"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"false"), false);
			});
				None
			}
			
			pub fn _none(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("none");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"none"), false);
			});
				None
			}
			
			pub fn _char(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("char");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"\"), false);
optional!(&mut node, self.expect(r"\"), false);
once!(&mut node, self.expect_letter(), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"\"), false);
once!(&mut node, self.expect(r"\"), false);
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
once!(&mut node, self.expect(r"."), true);
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
			
			pub fn _keyword_operator(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("keyword_operator");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"in"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"is"), false);
use_negative_lookahead!(self, self.tokens().index(), self.expect(r"not"));
use_negative_lookahead!(self, self.tokens().index(), self.expect(r"type"));
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"is"), false);
once!(&mut node, self.expect(r"not"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"is"), false);
once!(&mut node, self.expect(r"type"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"type"), false);
once!(&mut node, self.expect(r"of"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"and"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"not"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"or"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"as"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r">"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r">"), false);
once!(&mut node, self.expect(r"="), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"<"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"<"), false);
once!(&mut node, self.expect(r"="), false);
			});
				None
			}
			
			pub fn _arrow(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("arrow");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"="), false);
once!(&mut node, self.expect(r">"), false);
			});
				None
			}
			
			pub fn _comment_delimiter(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("comment_delimiter");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"*"), false);
once!(&mut node, self.expect(r"*"), false);
once!(&mut node, self.expect(r"*"), false);
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
				once!(&mut node, self.expect(r"split"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"split_at"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"split_n"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"replace"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"replace_n"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"char"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"push"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"pop"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"remove"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"insert"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"length"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"set"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"get"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"remove"), false);
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
				once!(&mut node, self.expect(r"package"), false);
optional!(&mut node, self._name_space(), false);
once!(&mut node, self._items(), false);
			});
				None
			}
			
			pub fn _name_space(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("name_space");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._item(), false);
once!(&mut node, self.expect(r":"), false);
once!(&mut node, self.expect(r":"), false);
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
				once!(&mut node, self.expect(r"["), false);

				group! ((self, false, node, pos) {
					once!(&mut node, self._item(), false);
once!(&mut node, self.expect(r","), false);
					
				});
			
once!(&mut node, self.expect(r"]"), false);
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
			
			pub fn _type_hint(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("type_hint");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._identifier(), false);
optional!(&mut node, self._bracket_list(&Self::_type_hint), false);
			});
				None
			}
			
			pub fn _type_def(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("type_def");
				
			alt! ((self, false, node, pos) {
				use_negative_lookahead!(self, self.tokens().index(), self.expect(r"is"));
once!(&mut node, self.expect(r"type"), false);
use_negative_lookahead!(self, self.tokens().index(), self.expect(r"of"));
once!(&mut node, self._type_hint(), false);
once!(&mut node, self._equal(), true);
once!(&mut node, self._type_value(), false);
			});
				None
			}
			
			pub fn _type_value(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("type_value");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._bracket_list(&Self::_type_field), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self._type_hint(), false);
			});
				None
			}
			
			pub fn _type_field(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("type_field");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._identifier(), false);
once!(&mut node, self._type_hint(), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self._identifier(), false);
			});
				None
			}
			
			pub fn _trait_def(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("trait_def");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"trait"), false);
once!(&mut node, self._type_hint(), false);
once!(&mut node, self._equal(), true);
once!(&mut node, self._trait_value(), false);
			});
				None
			}
			
			pub fn _trait_value(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("trait_value");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._methods(), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self._method_function(), false);
			});
				None
			}
			
			pub fn _implement_type(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("implement_type");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"use"), false);
once!(&mut node, self._type_hint(), false);
optional!(&mut node, self._implement_trait(), false);
once!(&mut node, self._equal(), true);
once!(&mut node, self._methods(), false);
			});
				None
			}
			
			pub fn _implement_trait(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("implement_trait");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"with"), false);
once!(&mut node, self._type_hint(), false);
			});
				None
			}
			
			pub fn _methods(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("methods");
				
			alt! ((self, false, node, pos) {
				optional!(&mut node, self._bracket_list(&Self::_method), false);
			});
				None
			}
			
			pub fn _method(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("method");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._type_hint(), false);
once!(&mut node, self.expect(r":"), false);
once!(&mut node, self._method_function(), false);
			});
				None
			}
			
			pub fn _method_function(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("method_function");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._parameters(), false);
once!(&mut node, self._return_type(), false);
optional!(&mut node, self._function_body(), false);
			});
				None
			}
			
			pub fn _function(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("function");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._parameters(), false);
once!(&mut node, self._return_type(), false);
once!(&mut node, self._function_body(), false);
			});
				None
			}
			
			pub fn _function_body(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("function_body");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self._arrow(), false);
once!(&mut node, self._empty(), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self._arrow(), false);
once!(&mut node, self._group(), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self._block(), false);
			});
				None
			}
			
			pub fn _return_type(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("return_type");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"-"), false);
once!(&mut node, self.expect(r">"), false);
once!(&mut node, self._type_hint(), false);
			});
				None
			}
			
			pub fn _parameters(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("parameters");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"_"), false);
			});

			alt! ((self, false, node, pos) {
				once!(&mut node, self._list(&Self::_parameter), false);
			});
				None
			}
			
			pub fn _parameter(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("parameter");
				
			alt! ((self, false, node, pos) {
				optional!(&mut node, self.expect(r"mut"), false);
once!(&mut node, self._identifier(), false);
once!(&mut node, self._type_hint(), false);
			});
				None
			}
			
			pub fn _list(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("list");
				
			alt! ((self, true, node, pos) {
				
				group! ((self, false, node, pos) {
					once!(&mut node, inner(self), false);
once!(&mut node, self.expect(r","), false);
					
				});
			
optional!(&mut node, inner(self), false);
			});
				None
			}
			
			pub fn _paren(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("paren");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"("), false);
once!(&mut node, inner(self), false);
once!(&mut node, self._closing_paren(), false);
			});
				None
			}
			
			pub fn _paren_list(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("paren_list");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"("), false);
once!(&mut node, self._list(inner), false);
once!(&mut node, self._closing_paren(), false);
			});
				None
			}
			
			pub fn _brace(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("brace");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"{"), false);
once!(&mut node, inner(self), false);
once!(&mut node, self._closing_brace(), false);
			});
				None
			}
			
			pub fn _bracket(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("bracket");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"["), false);
once!(&mut node, inner(self), false);
once!(&mut node, self._closing_bracket(), false);
			});
				None
			}
			
			pub fn _bracket_list(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("bracket_list");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"["), false);
once!(&mut node, self._list(inner), false);
once!(&mut node, self._closing_bracket(), false);
			});
				None
			}
			
			pub fn _closing_paren(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("closing_paren");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r")"), true);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect_tag("<e 'missing closing parenthesis'>"), false);
			});
				None
			}
			
			pub fn _closing_brace(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("closing_brace");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"}"), true);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect_tag("<e 'missing closing brace'>"), false);
			});
				None
			}
			
			pub fn _closing_bracket(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("closing_bracket");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"]"), true);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect_tag("<e 'missing closing bracket'>"), false);
			});
				None
			}
			
			pub fn _regex(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("regex");
				
			alt! ((self, false, node, pos) {
				once!(&mut node, self.expect(r"/"), false);
once!(&mut node, self._regex_inner(), false);
once!(&mut node, self.expect(r"/"), false);
			});
				None
			}
			
			pub fn _regex_inner(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("regex_inner");
				
			alt! ((self, false, node, pos) {
				use_negative_lookahead!(self, self.tokens().index(), self.expect(r"/"));
once!(&mut node, self.expect_any(), false);
			});
				None
			}
			
			pub fn _equal(&mut self) -> Option<(ParseNode, bool)> {
				let (mut node, pos) = self.start_parse("equal");
				
			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"="), false);
use_negative_lookahead!(self, self.tokens().index(), self.expect(r">"));
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
				once!(&mut node, self.expect(r"for"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"while"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"else"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"if"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"mut"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"loop"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"box"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"break"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"return"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"continue"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"type"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"log"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"logs"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"log_line"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"logs_line"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"use"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"with"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"trait"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"let"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"const"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"in"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"is"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"and"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"not"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"or"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"as"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"istype"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"true"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"false"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"none"), false);
			});

			alt! ((self, true, node, pos) {
				once!(&mut node, self.expect(r"package"), false);
			});
				None
			}
			
}