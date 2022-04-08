use crate::{
    Error,
    TokenType,
    Token,
    Symbol,
    Keyword,
    Literal,
    Operator,
    Assign,
};

pub struct Source {
    pub source: String,
    pub start_index: usize,
    pub index: usize,
    pub line: usize,
    pub chars: Vec<char>
}

impl Source {
    pub fn new(string: &str) -> Self {
        Self {
            source: string.to_string(),
            start_index: 0,
            index: 0,
            line: 1,
            chars: string.clone().chars().collect()
        }
    }

    pub fn current(&self) -> Option<&char> {
        return self.chars.get(self.index);
    }

    pub fn get_string(&self) -> Option<&str> {
        let start = self.start_index;
        let end = self.index + 1;
        return self.source.get(start..end);
    }

    pub fn is_done(&self) -> bool {
        return self.index >= self.chars.len();
    }

    pub fn peek(&self, amount: usize) -> Option<&char> {
        return self.chars.get(self.index + amount);
    }

    pub fn advance(&mut self) -> Option<&char> {
        self.index += 1;
        let current_char = self.chars.get(self.index).unwrap_or(&'\0');
        if *current_char == '\n' { self.advance_line(); }
        return self.chars.get(self.index);
    }

    pub fn advance_line(&mut self) -> () {
        self.line += 1;
    }

    pub fn match_next(&mut self, expected: char) -> bool {
        if self.is_done() { return false; }
        let current_char = self.chars.get(self.index + 1).unwrap_or(&'\0');
        if *current_char != expected { return false; }
        self.advance();
        return true;
    }
}

pub struct Lexer {
    chars: Box<Source>,
    pub tokens: Vec<Token>,
    pub errors: Vec<Error>,
    pub debug: bool,
}

impl Lexer {
    pub fn new(source: String, debug: bool) -> Self {
        Self {
            chars: Box::new(Source::new(source.as_str())),
            tokens: Vec::new(),
            errors: Vec::new(),
            debug,
        }
    }

    pub fn scan(&mut self) -> () {
        while !self.chars.is_done() {
            self.chars.start_index = self.chars.index;
            self.scan_token();
            self.chars.advance();
        }
        self.add_token(TokenType::Eof);
    }

    pub fn scan_string(&mut self, source: String) -> () {
        self.chars = Box::new(Source::new(source.as_str()));
        self.tokens = vec![];
        self.errors = vec![];
        self.scan();
    }

    fn scan_token(&mut self) -> () {
        let c = self.chars.current().unwrap_or(&'\0');
        let token_type = TokenType::from_char(c).unwrap_or(TokenType::Eof);
        match token_type {
            TokenType::Digit => self.number(),
            TokenType::Letter => self.letter(),
            TokenType::Symbol(Symbol::Quote) => self.string(),
            TokenType::Whitespace => (),
            TokenType::Symbol(Symbol::Hashtag) => self.line_comment(),
            TokenType::Symbol(op) => self.symbol(op),
            _ => self.add_token(token_type)
        }
        // TODO multiline comments
    }

    fn number(&mut self) {
        loop { // before the decimal point
            if let Some(c) = self.chars.peek(1) {
                if !c.is_digit(10) { break; }
            } else { break; }
            self.chars.advance();
        }

        // check for decimal point
        if let (Some(a), Some(b)) = (self.chars.peek(1), self.chars.peek(2)) {
            if *a == '.' && b.is_digit(10) {
                loop { // after the decimal point
                    self.chars.advance();
                    if let Some(c) = self.chars.peek(1) {
                        if !c.is_digit(10) { break; }
                    } else { break; }
                }
                self.add_token(TokenType::Literal(Literal::Decimal));
                return;
            }
        }
        self.add_token(TokenType::Literal(Literal::Number));
    }

    fn letter(&mut self) {
        loop {
            if let Some(c) = self.chars.peek(1) {
                if !c.is_alphanumeric() && *c != '_' { break; }
                self.chars.advance();
            } else { break; }
        }
        let string = self.chars.get_string().unwrap_or("");
        let keyword = Keyword::from_str(string);
        match keyword {
            Some(key) => match key {
                Keyword::Is => self.add_token(TokenType::Assign(Assign::Equal)),
                Keyword::Not => self.add_token(TokenType::Operator(Operator::Not)),
                Keyword::And => self.add_token(TokenType::Operator(Operator::And)),
                Keyword::Or => self.add_token(TokenType::Operator(Operator::Or)),
                k => self.add_token(TokenType::Keyword(k)),
            },
            None => self.add_token(TokenType::Identifier)
        }
    }

    fn string(&mut self) {
        loop {
            if let Some(c) = self.chars.peek(1) {
                if *c == '"' || *c == '\'' { break; }
                self.chars.advance();
            } else { break; }
        }
        if self.chars.is_done() {
            self.errors.push(Error { 
                line: self.chars.line,
                index: self.chars.index,
                source: "\"".to_string(),
                message: "Unterminated string".to_string() 
            });
        }
        self.chars.advance(); // consume closing quote mark
        
        // remove opening and closing quotes
        let source_string = self.chars.get_string().unwrap_or("").replace(&['"', '\''], "");
        if self.debug {
            println!("Adding string from \"{}\"", source_string);
        }
        // add token
        self.tokens.push(Token::new(
            TokenType::Literal(Literal::String), 
            source_string.as_str(), 
            self.chars.line, 
            self.chars.start_index
        ));
    }

    fn line_comment(&mut self) {
        loop {
            if let Some(p) = self.chars.peek(1) {
                if *p == '\n' || self.chars.is_done() { break; }
                self.chars.advance();
            } else { break; }
        }
    }

    fn symbol(&mut self, symbol: Symbol) {
        let next_equal = self.chars.match_next('=');
        let add_token_type = match next_equal {
            true => match symbol {
                Symbol::Equal => Some(TokenType::Operator(Operator::Equal)),
                Symbol::Bang => Some(TokenType::Operator(Operator::NotEqual)),
                Symbol::Less => Some(TokenType::Operator(Operator::LessEqual)),
                Symbol::Greater => Some(TokenType::Operator(Operator::GreaterEqual)),
                Symbol::Plus => Some(TokenType::Assign(Assign::AddEqual)),
                Symbol::Minus => Some(TokenType::Assign(Assign::SubtractEqual)),
                Symbol::Star => Some(TokenType::Assign(Assign::MultiplyEqual)),
                Symbol::Slash => Some(TokenType::Assign(Assign::DivideEqual)),
                _ => None
            },
            false => match symbol {
                Symbol::Equal => {
                    let next_arrow = self.chars.match_next('>');
                    if next_arrow {
                        Some(TokenType::Keyword(Keyword::Arrow))
                    } else {
                        Some(TokenType::Assign(Assign::Equal))
                    }
                },
                Symbol::Bang => Some(TokenType::Operator(Operator::Not)),
                Symbol::Less => Some(TokenType::Operator(Operator::Less)),
                Symbol::Greater => Some(TokenType::Operator(Operator::Greater)),
                Symbol::Plus => Some(TokenType::Operator(Operator::Add)),
                Symbol::Minus => Some(TokenType::Operator(Operator::Subtract)),
                Symbol::Star => Some(TokenType::Operator(Operator::Multiply)),
                Symbol::Slash => Some(TokenType::Operator(Operator::Divide)),
                symbol => Some(TokenType::Symbol(symbol)),
            },
        };
        if let Some(token_type) = add_token_type {
            self.add_token(token_type);
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let source_string = self.chars.get_string().unwrap_or("");
        if self.debug {
            println!("Adding {} from \"{}\"", token_type, source_string);
        }
        self.tokens.push(Token::new(token_type, source_string, self.chars.line, self.chars.start_index));
    }
}

#[test]
fn test_lexer() {

    fn assert(string: &str, answer: usize) {
        let mut lexer = Lexer::new(string.to_string(), true);
        lexer.scan();
        assert_eq!(lexer.tokens.len(), answer);
    }

    assert("abc", 2);
    assert("123.456", 2);
    assert("1 == 2", 4);
    assert("false != true", 4);
    assert("this is that += 1", 6);
    assert("# commented", 1);
    assert("1 * 2 * 3 + 4", 8);
    assert("x y z", 4)
}
