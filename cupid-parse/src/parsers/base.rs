#![allow(clippy::all)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
use crate::*;

type ParseFun = dyn Fn(&mut BaseParser) -> Option<(ParseNode, bool)>;

pub struct BaseParser {
	pub tokens: BiDirectionalIterator<Token>,
	pub file: usize,
}

impl Parser for BaseParser {
	fn tokens(&mut self) -> &mut BiDirectionalIterator<Token> {
		&mut self.tokens
	}
	fn file(&self) -> usize { self.file }
}

impl BaseParser {
	pub fn new(source: String, file: usize) -> Self {
		Self { tokens: Self::build(source, file), file }
	}
    
	/*RULES*/
}