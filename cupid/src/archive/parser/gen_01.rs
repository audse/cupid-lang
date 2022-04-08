use crate::Rule;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
    pub tokens: Vec<String>,
}

pub struct Parser {
    pub index: usize,
    pub tokens: Vec<String>,
}

impl Parser {
    fn mark(&self) -> usize {
        self.index
    }
    fn reset(&mut self, pos: usize) {
        self.index = pos;
    }
    fn current(&self) -> String {
        self.tokens[self.index].clone()
    }
    fn expect(&mut self, string: &str) -> Option<String> {
        let next = self.current();
        if next == string.to_string() {
            self.index += 1;
            return Some(next);
        }
        return None;
    }
    fn expect_number(&mut self) -> Option<String> {
        let next = self.current();
        let chars: Vec<char> = next.chars().collect();
        for char in chars {
            if !char.is_digit(10) {
                return None;
            }
        }
        self.index += 1;
        return Some(next);
    }
    fn expect_word(&mut self) -> Option<String> {
        let next = self.current();
        let chars: Vec<char> = next.chars().collect();
        for char in chars {
            if !char.is_alphabetic() {
                return None;
            }
        }
        self.index += 1;
        return Some(next);
    }

    pub fn operation(&mut self) -> Option<Node> {
        let pos = self.mark();

        if let (Some(atom), Some(operator), Some(atom2)) =
            (self.atom(), self.operator(), self.atom())
        {
            return Some(Node {
                name: String::from("operation"),
                children: vec![atom, operator, atom2],
                tokens: vec![],
            });
        }
        self.reset(pos);

        return None;
    }

    pub fn operator(&mut self) -> Option<Node> {
        let pos = self.mark();

        if let Some(_token0) = self.expect("+") {
            return Some(Node {
                name: String::from("operator"),
                children: vec![],
                tokens: vec![_token0],
            });
        }
        self.reset(pos);

        if let Some(_token0) = self.expect("-") {
            return Some(Node {
                name: String::from("operator"),
                children: vec![],
                tokens: vec![_token0],
            });
        }
        self.reset(pos);

        if let Some(_token0) = self.expect("*") {
            return Some(Node {
                name: String::from("operator"),
                children: vec![],
                tokens: vec![_token0],
            });
        }
        self.reset(pos);

        if let Some(_token0) = self.expect("/") {
            return Some(Node {
                name: String::from("operator"),
                children: vec![],
                tokens: vec![_token0],
            });
        }
        self.reset(pos);

        return None;
    }

    pub fn decimal(&mut self) -> Option<Node> {
        let pos = self.mark();

        if let (Some(number), Some(_token1), Some(number2)) =
            (self.expect_number(), self.expect("."), self.expect_number())
        {
            return Some(Node {
                name: String::from("decimal"),
                children: vec![],
                tokens: vec![number, _token1, number2],
            });
        }
        self.reset(pos);

        return None;
    }

    pub fn atom(&mut self) -> Option<Node> {
        let pos = self.mark();

        if let Some(decimal) = self.decimal() {
            return Some(Node {
                name: String::from("atom"),
                children: vec![decimal],
                tokens: vec![],
            });
        }
        self.reset(pos);

        if let Some(number) = self.expect_number() {
            return Some(Node {
                name: String::from("atom"),
                children: vec![],
                tokens: vec![number],
            });
        }
        self.reset(pos);

        if let Some(_token0) = self.expect("true") {
            return Some(Node {
                name: String::from("atom"),
                children: vec![],
                tokens: vec![_token0],
            });
        }
        self.reset(pos);

        if let Some(_token0) = self.expect("false") {
            return Some(Node {
                name: String::from("atom"),
                children: vec![],
                tokens: vec![_token0],
            });
        }
        self.reset(pos);

        if let Some(word) = self.expect_word() {
            return Some(Node {
                name: String::from("atom"),
                children: vec![],
                tokens: vec![word],
            });
        }
        self.reset(pos);

        return None;
    }
}
