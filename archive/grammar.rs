use crate::{is_identifier, is_string, is_uppercase, BiDirectionalIterator, Tokenizer};

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
    pub tokens: Vec<String>,
}

pub struct Parser {
    pub tokens: BiDirectionalIterator<String>,
    pub index: usize,
}

impl Parser {
    pub fn new(source: String) -> Self {
        let mut tokenizer = Tokenizer::new(source.as_str());
        tokenizer.scan();
        println!("{:?}", tokenizer.tokens);
        Self {
            index: 0,
            tokens: BiDirectionalIterator::new(tokenizer.tokens),
        }
    }
    fn expect(&mut self, rule_name: &str) -> Option<String> {
        self.tokens.expect(rule_name.to_string())
    }
    fn expect_one(&mut self, rule_names: Vec<&str>) -> Option<String> {
        for rule_name in rule_names {
            if let Some(next) = self.expect(rule_name) {
                return Some(next);
            }
        }
        return None;
    }
    fn expect_constant(&mut self) -> Option<String> {
        if let Some(next) = self.tokens.peek(0) {
            if is_uppercase(next.clone()) {
                return self.tokens.next();
            }
        }
        return None;
    }
    fn expect_word(&mut self) -> Option<String> {
        if let Some(next) = self.tokens.peek(0) {
            if is_identifier(next.clone()) {
                return self.tokens.next();
            }
        }
        return None;
    }
    fn expect_string(&mut self) -> Option<String> {
        if let Some(next) = self.tokens.peek(0) {
            if is_string(next.clone()) {
                return self.tokens.next();
            }
        }
        return None;
    }

    pub fn _grammar(&mut self) -> Option<Node> {
        let pos = self.tokens.index();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];
        while let Some(rule01) = self._rule() {
            children.push(rule01);
        }
        return Some(Node {
            name: String::from("grammar"),
            tokens,
            children,
        });
        tokens = vec![];
        children = vec![];
        self.tokens.goto(pos);

        return None;
    }

    pub fn _rule(&mut self) -> Option<Node> {
        let pos = self.tokens.index();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];
        if let Some(_token0) = self.expect("-") {
            if let Some(word1) = self.expect_word() {
                tokens.push(word1);
                if let Some(_token2) = self.expect(":") {
                    if let Some(item3) = self._item() {
                        children.push(item3);
                        while let Some(item34) = self._item() {
                            children.push(item34);
                        }
                        while let Some(alt45) = self._alt() {
                            children.push(alt45);
                        }
                        return Some(Node {
                            name: String::from("rule"),
                            tokens,
                            children,
                        });
                    }
                }
            }
        }
        tokens = vec![];
        children = vec![];
        self.tokens.goto(pos);

        return None;
    }

    pub fn _alt(&mut self) -> Option<Node> {
        let pos = self.tokens.index();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];
        if let Some(_token0) = self.expect("|") {
            if let Some(item1) = self._item() {
                children.push(item1);
                while let Some(item12) = self._item() {
                    children.push(item12);
                }
                return Some(Node {
                    name: String::from("alt"),
                    tokens,
                    children,
                });
            }
        }
        tokens = vec![];
        children = vec![];
        self.tokens.goto(pos);

        return None;
    }

    pub fn _item(&mut self) -> Option<Node> {
        let pos = self.tokens.index();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];
        if let Some(word0) = self._word() {
            children.push(word0);
            return Some(Node {
                name: String::from("item"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.tokens.goto(pos);

        if let Some(string0) = self._string() {
            children.push(string0);
            return Some(Node {
                name: String::from("item"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.tokens.goto(pos);

        return None;
    }

    pub fn _string(&mut self) -> Option<Node> {
        let pos = self.tokens.index();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];
        if let Some(prefix0) = self._prefix() {
            children.push(prefix0);
        }
        if let Some(string1) = self.expect_string() {
            tokens.push(string1);
            if let Some(suffix2) = self._suffix() {
                children.push(suffix2);
            }
            return Some(Node {
                name: String::from("string"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.tokens.goto(pos);

        return None;
    }

    pub fn _word(&mut self) -> Option<Node> {
        let pos = self.tokens.index();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];
        if let Some(prefix0) = self._prefix() {
            children.push(prefix0);
        }
        if let Some(word1) = self.expect_word() {
            tokens.push(word1);
            if let Some(suffix2) = self._suffix() {
                children.push(suffix2);
            }
            return Some(Node {
                name: String::from("word"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.tokens.goto(pos);

        return None;
    }

    pub fn _prefix(&mut self) -> Option<Node> {
        let pos = self.tokens.index();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];
        if let Some(_token0) = self.expect("~") {
            tokens.push(_token0);
            return Some(Node {
                name: String::from("prefix"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.tokens.goto(pos);

        if let Some(_token0) = self.expect("!") {
            tokens.push(_token0);
            return Some(Node {
                name: String::from("prefix"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.tokens.goto(pos);

        if let Some(_token0) = self.expect("&") {
            tokens.push(_token0);
            return Some(Node {
                name: String::from("prefix"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.tokens.goto(pos);

        return None;
    }

    pub fn _suffix(&mut self) -> Option<Node> {
        let pos = self.tokens.index();
        let mut tokens: Vec<String> = vec![];
        let mut children: Vec<Node> = vec![];
        if let Some(_token0) = self.expect("*") {
            tokens.push(_token0);
            return Some(Node {
                name: String::from("suffix"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.tokens.goto(pos);

        if let Some(_token0) = self.expect("+") {
            tokens.push(_token0);
            return Some(Node {
                name: String::from("suffix"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.tokens.goto(pos);

        if let Some(_token0) = self.expect("?") {
            tokens.push(_token0);
            return Some(Node {
                name: String::from("suffix"),
                tokens,
                children,
            });
        }
        tokens = vec![];
        children = vec![];
        self.tokens.goto(pos);

        return None;
    }
}
