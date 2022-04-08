
use crate::{
    Error,
    Token,
    TokenType,
    Keyword,
};

struct SourceChars {
    source_string: String,
    start_index: usize,
    current_index: usize,
    current_line: i32,
    chars: Vec<char>
}

impl SourceChars {
    fn new(source: String) -> Self {
        let source_string = source.clone();
        let chars = source.chars().collect();
        Self {
            source_string: source_string,
            start_index: 0,
            current_index: 0,
            current_line: 1,
            chars: chars
        }
    }

    fn get_current(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.chars[self.current_index];
    }

    fn get_substring(&mut self) -> String {
        let start = self.start_index;
        let end = self.current_index + 1;

        let substring = self.source_string.get(start..end);
        if substring.is_some() {
            return String::from(substring.unwrap());
        } else {
            return String::from("");
        }
    }

    fn is_at_end(&mut self) -> bool {
        return self.current_index >= self.chars.len();
    }

    fn advance(&mut self) -> char {
        self.current_index += 1;
        if self.is_at_end() {
            return '\0';
        }
        return self.chars[self.current_index];
    }

    fn advance_line(&mut self) -> () {
        self.current_line += 1;
    }

    fn peek(&mut self) -> char {
        if self.current_index >= self.chars.len() - 1 {
            return '\0';
        } else { 
            let next_index = self.current_index + 1;
            return self.chars[next_index];
        }
    }

    // only allow two peeks forward
    fn peek_next(&mut self) -> char {
        let next_index = self.current_index + 2;
        if next_index >= self.chars.len() {
            return '\0';
        }
        return self.chars[next_index];
    }

    // combines peek() and advance()
    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() { 
            return false;
        }
        if self.chars[self.current_index] != expected { 
            return false;
        }
        self.advance();
        return true;
    }
}

pub struct Scanner {
    chars: Box<SourceChars>,
    pub tokens: Vec<Token>,
    pub errors: Vec<Error>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            chars: Box::new(SourceChars::new(source)),
            tokens: Vec::new(),
            errors: Vec::new()
        }
    }

    pub fn scan_tokens(&mut self) -> () {
        while !self.chars.is_at_end() {
            self.chars.start_index = self.chars.current_index;
            self.scan_token();

        }
        self.add_token(TokenType::Eof);

        for error in &mut self.errors {
            error.report();
        }
    }
    
    fn scan_token(&mut self) -> () {
        let c = self.chars.get_current();

        if c.is_alphanumeric() {
            if c.is_digit(10) {
                self.number();
            } else { // is [A-Za-z]
                self.identifier();
            }
        } else {
            match c {
                '=' => if self.chars.match_next('=') {
                        self.add_token(TokenType::Equal);
                    } else {
                        self.add_token(TokenType::Assign);
                    },
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '#' => while self.chars.peek() != '\n' && !self.chars.is_at_end() {
                        self.chars.advance();
                    },
                '>' => if self.chars.match_next('=') {
                        self.add_token(TokenType::GreaterEqual);
                    } else {
                        self.add_token(TokenType::Greater);
                    },
                '<' => if self.chars.match_next('=') {
                        self.add_token(TokenType::LessEqual);
                    } else { 
                        self.add_token(TokenType::Less);
                    },
                '+' => self.add_token(TokenType::Plus),
                '-' => self.add_token(TokenType::Minus),
                '/' => self.add_token(TokenType::Slash),
                '*' => self.add_token(TokenType::Asterisk),
                '!' => self.add_token(TokenType::Bang),
                '"' => self.string(),
                ' ' | '\r' | '\t' => (),
                '\n' => self.chars.advance_line(),
                other => self.errors.push(Error::new(
                        self.chars.current_line,
                        String::from(format!("An unexpected character was found: {}", other))
                ))
            }
        }

        self.chars.advance();
    }

    fn add_token(&mut self, token_type: TokenType) -> () {
        println!("Adding token {} '{}'", token_type, self.chars.get_substring());
        self.tokens.push(Token::new(
            token_type,
            self.chars.get_substring(),
            self.chars.current_line
        ));
    }

    fn string(&mut self) {
        while self.chars.peek() != '"' && !self.chars.is_at_end() {
            if self.chars.peek() == '\n'{
                self.chars.advance_line();
            }
            self.chars.advance();
        }

        if self.chars.is_at_end() {
            self.errors.push(Error {
                line: self.chars.current_line,
                message: String::from("Unterminated string.")
            });
            return;
        }

        self.chars.advance();
        self.add_token(TokenType::StringLiteral);
    }

    fn number(&mut self) -> () {
        while self.chars.peek().is_digit(10) {
            self.chars.advance();
        }
        if self.chars.peek() == '.' && self.chars.peek_next().is_digit(10) {
            self.chars.advance();
            while self.chars.peek().is_digit(10) {
                self.chars.advance();
            }
        }
        self.add_token(TokenType::NumberLiteral);
    }

    fn identifier(&mut self) -> () {
        while self.chars.peek().is_alphanumeric() || self.chars.peek() == '_' {
            self.chars.advance();
        }
        let string = &self.chars.get_substring() as &str;

        match string {
            "true" => self.add_token(TokenType::Keyword(Keyword::True)),
            "false" => self.add_token(TokenType::Keyword(Keyword::False)),
            "none" => self.add_token(TokenType::Keyword(Keyword::None)),
            "is" => self.add_token(TokenType::Keyword(Keyword::Is)),
            "not" => self.add_token(TokenType::Keyword(Keyword::Not)),
            "and" => self.add_token(TokenType::Keyword(Keyword::And)),
            "or" => self.add_token(TokenType::Keyword(Keyword::Or)),
            "if" => self.add_token(TokenType::Keyword(Keyword::If)),
            "else" => self.add_token(TokenType::Keyword(Keyword::Else)),
            _ => self.add_token(TokenType::Identifier),
        }
    }
}