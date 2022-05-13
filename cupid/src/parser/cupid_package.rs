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
        Self {
            tokens: Self::build(source),
        }
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

    pub fn _list(&mut self, item: &ParseFun) -> Option<(ParseNode, bool)> {
        let (mut node, pos) = self.start_parse("list");

        alt! ((self, false, node, pos) {
			group! ((self, false, node, pos) {
				once!(&mut node, self._item(), false);
				once!(&mut node, self.expect(","), false);
			});
			optional!(&mut node, self._item(), false);
		});
        None
    }
}
