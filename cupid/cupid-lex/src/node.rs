use std::fmt::{
	Display,
	Result as DisplayResult,
	Formatter,
};
use std::borrow::Cow;
use crate::token::Token;
use crate::InvertOption;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ParseNode {
	pub name: Cow<'static, str>,
	pub children: Vec<ParseNode>,
	pub tokens: Vec<Token>,
}

pub enum Index<'i> {
	Int(usize),
	Str(&'i str)
}

impl From<usize> for Index<'_> {
	fn from(i: usize) -> Self {
		Self::Int(i)
	}
}

impl<'i> From<&'i str> for Index<'i> {
	fn from(s: &'i str) -> Self {
		Self::Str(s)
	}
}

use Index::*;

impl ParseNode {
	pub fn source(&self) -> Cow<'static, str> {
		self.tokens[0].source.to_owned()
	}
	pub fn token(&self, index: usize) -> Token {
		self.tokens.get(index).unwrap_or_else(|| panic!("couldn't find token {index} in {self}")).to_owned()
	}
	pub fn has_token(&self, name: &str) -> bool {
		self.tokens.iter().any(|c| c.source == name)
	}
	pub fn has<'i, I: Into<Index<'i>>>(&mut self, name: I) -> bool {
		self.get_option(name).is_some()
	}
	pub fn get<'i, I: Into<Index<'i>> + std::fmt::Debug + Copy>(&mut self, index: I) -> &mut Self {
		self.get_option(index).unwrap_or_else(|| panic!("cannot find {index:?}"))
	}
	pub fn get_option<'i, I: Into<Index<'i>>>(&mut self, name: I) -> Option<&mut Self> {
		match name.into() {
			Int(i) => self.children.get_mut(i),
			Str(name) => self.children.iter_mut().find(|c| c.name == name)
		}
	}
	pub fn get_map<'a, I: Into<Index<'a>> + std::fmt::Debug + Copy, R, E>(&mut self, name: I, function: impl FnOnce(&mut Self) -> Result<R, E> ) -> Result<R, E> {
		function(self.get_option(name).unwrap_or_else(|| panic!("cannot find {name:?}")))
	}
	pub fn get_option_map<'i, I: Into<Index<'i>>, R, E>(&mut self, name: I, function: impl FnOnce(&mut Self) -> Result<R, E> ) -> Result<Option<R>, E> {
		self.get_option(name).map(function).invert()
	}
	pub fn get_all_named(&mut self, name: &str) -> Vec<&mut Self> {
		self.children.iter_mut().filter(|c| &*c.name == name).collect()
	}
	pub fn map_named<R, E>(&mut self, name: &str, function: impl FnMut(&mut Self) -> Result<R, E>) -> Result<Vec<R>, E> {
		self.get_all_named(name).into_iter().map(function).collect()
	}
	pub fn map_children_of<'i, I: Into<Index<'i>> + std::fmt::Debug + Copy, R, E>(&mut self, name: I, function: impl FnMut(&mut Self) -> Result<R, E>) -> Result<Vec<R>, E> {
		self.get(name).children.iter_mut().map(function).collect()
	}
}

impl From<(Token, &'static str)> for ParseNode {
	fn from(data: (Token, &'static str)) -> Self {
		ParseNode {
			name: Cow::Borrowed(data.1),
			tokens: vec![data.0],
			children: vec![],
		}
	}
}

impl Display for ParseNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "{:#?}", self)
	}
}