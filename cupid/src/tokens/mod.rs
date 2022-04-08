use std::fmt;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Symbol(Symbol),
    Literal(Literal),
    Keyword(Keyword),
    Operator(Operator),
    Assign(Assign),
    // Compare(Compare),
    Identifier,
    Whitespace,
    Eof,
    Digit,
    Letter,
}

impl TokenType {
    
    pub fn get_op(operator: Self) -> Operator {
        match operator {
            TokenType::Operator(op) => op,
            op => panic!("{} is not an operator", op)
        }
    }
    
    pub fn get_assign(assigner: Self) -> Assign {
        match assigner {
            TokenType::Assign(a) => a,
            a => panic!("{} is not an assigner", a)
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Assign {
    Equal,
    AddEqual,
    SubtractEqual,
    MultiplyEqual,
    DivideEqual,
}

impl fmt::Display for Assign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Hashtag,
    Quote,
    Bang,
    Equal,
    Plus,
    Minus,
    Slash,
    Star,
    Less,
    Greater,
    Dot,
    Comma,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Literal {
    String,
    Number,
    Decimal,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Keyword {
    If,
    Else,
    True,
    False,
    None,
    Is,
    Not,
    And,
    Or,
    Arrow,
    Let,
    Const,
    Mut,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Keyword {
    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "true" => Some(Self::True),
            "false" => Some(Self::False),
            "none" => Some(Self::None),
            "is" => Some(Self::Is),
            "not" => Some(Self::Not),
            "and" => Some(Self::And),
            "or" => Some(Self::Or),
            "=>" => Some(Self::Arrow),
            "let" => Some(Self::Let),
            "const" => Some(Self::Const),
            "mut" => Some(Self::Mut),
            _ => None
        }
    }
}

impl TokenType {
    pub fn from_char(current_char: &char) -> Option<Self> {
        match *current_char {
            '0'..='9' => Some(Self::Digit),
            'a'..='z' | 'A'..='Z' | '_' => Some(Self::Letter),
            '+' => Some(Self::Symbol(Symbol::Plus)),
            '-' => Some(Self::Symbol(Symbol::Minus)),
            '/' => Some(Self::Symbol(Symbol::Slash)),
            '*' => Some(Self::Symbol(Symbol::Star)),
            '(' => Some(Self::Symbol(Symbol::LeftParen)),
            ')' => Some(Self::Symbol(Symbol::RightParen)),
            '{' => Some(Self::Symbol(Symbol::LeftBrace)),
            '}' => Some(Self::Symbol(Symbol::RightBrace)),
            '#' => Some(Self::Symbol(Symbol::Hashtag)),
            '<' => Some(Self::Symbol(Symbol::Less)),
            '>' => Some(Self::Symbol(Symbol::Greater)),
            '=' => Some(Self::Symbol(Symbol::Equal)),
            '"' => Some(Self::Symbol(Symbol::Quote)),
            '\'' => Some(Self::Symbol(Symbol::Quote)),
            '!' => Some(Self::Symbol(Symbol::Bang)),
            ',' => Some(Self::Symbol(Symbol::Comma)),
            ' ' | '\r' | '\t' => Some(Self::Whitespace),
            '.' => Some(Self::Symbol(Symbol::Dot)),
            _ => None,
        }
    }
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Hash)]
pub struct Token {
    pub source: String,
    pub token_type: TokenType,
    pub line: usize,
    pub index: usize,
}

impl Token {
    pub fn new(token_type: TokenType, source: &str, line: usize, index: usize) -> Self {
        Self {
            source: source.to_string(),
            token_type,
            line,
            index,
        }
    }
    
    pub fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            token_type: self.token_type,
            line: self.line,
            index: self.index
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}