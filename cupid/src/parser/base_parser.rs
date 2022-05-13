#![allow(clippy::all)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
use crate::*;

type ParseFun = dyn Fn(&mut BaseParser) -> Option<(ParseNode, bool)>;

#[derive(PartialEq, Eq)]
pub struct BaseParser {
	pub tokens: BiDirectionalIterator<Token>,
}

impl Parser for BaseParser {
	fn tokens(&mut self) -> &mut BiDirectionalIterator<Token> {
		&mut self.tokens
	}
}

impl BaseParser {
	pub fn new(source: String) -> Self {
		Self { tokens: Self::build(source) }
	}
    
    /*RULES*/

}