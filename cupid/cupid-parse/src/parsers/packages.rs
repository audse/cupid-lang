#![allow(clippy::all)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
use crate::parser::*;
use crate::*;

type ParseFun = dyn Fn(&mut BaseParser) -> Option<(ParseNode, bool)>;

#[derive(PartialEq, Eq, Debug, Clone, Default)]
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
	pub fn update(&mut self, source: String, file: usize) {
		self.tokens = Self::build(source, file);
		self.file = file;
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
            once!(&mut node, self.expect("("), false);once!(&mut node, inner(self), false);once!(&mut node, self._closing_paren(), false);
        });
        None
    }

    pub fn _brace(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("brace");

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("{"), false);once!(&mut node, inner(self), false);once!(&mut node, self._closing_brace(), false);
        });
        None
    }

    pub fn _bracket(&mut self, inner: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("bracket");

        alt! ((self, true, node, pos) {
            once!(&mut node, self.expect("["), false);once!(&mut node, inner(self), false);once!(&mut node, self._closing_bracket(), false);
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
            once!(&mut node, self.expect("package"), false);optional!(&mut node, self._name_space(), false);once!(&mut node, self._items(), false);
        });
        None
    }

    pub fn _name_space(&mut self) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("name_space");

        alt! ((self, false, node, pos) {
            once!(&mut node, self._item(), false);once!(&mut node, self.expect(":"), false);once!(&mut node, self.expect(":"), false);optional!(&mut node, self._name_space(), false);
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
}
