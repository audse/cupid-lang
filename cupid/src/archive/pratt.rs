#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Atom(char),
    Op(char),
    Eof,
}

struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut tokens = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
                _ => Token::Op(c)
            }) // convert tokens to alphanumeric or operator
            .collect::<Vec<_>>();
        tokens.reverse();
        return Lexer { tokens };
    }

    fn next(&mut self) -> Token {
        return self.tokens.pop().unwrap_or(Token::Eof);
    }

    fn peek(&mut self) -> Token {
        return self.tokens.last().copied().unwrap_or(Token::Eof);
    }
}

enum S {
    Atom(char),
    Cons(char, Vec<S>),
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(i) => return write!(f, "{}", i),
            S::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?;
                }
                return write!(f, ")");
            }
        }
    }
}

fn expr(input: &str) -> S {
    let mut lexer = Lexer::new(input);
    return expr_bin_op(&mut lexer, 0);
}

fn expr_bin_op(lexer: &mut Lexer, min_bp: u8) -> S {
    let mut left = match lexer.next() {
        Token::Atom(it) => S::Atom(it),
        Token::Op('(') => {
                let left = expr_bin_op(lexer, 0);
                assert_eq!(lexer.next(), Token::Op(')'));
                left
            },
        Token::Op(op) => {
                let ((), right_bp) = prefix_binding_power(op);
                let right = expr_bin_op(lexer, right_bp);
                S::Cons(op, vec![right])
            },
        token => panic!("bad token: {:?} (must supply an alphanumeric character)", token),
    };

    loop {
        let operator = match lexer.peek() {
            Token::Eof => break,
            Token::Op(operator) => operator,
            token => panic!("bad token: {:?} (must supply an operator)", token),
        };

        if let Some((left_bp, ())) = postfix_binding_power(operator) {
            if left_bp < min_bp {
                break;
            }
            lexer.next();

            left = if operator == '[' {
                let right = expr_bin_op(lexer, 0);
                assert_eq!(lexer.next(), Token::Op(']'));
                S::Cons(operator, vec![left, right])
            } else {
                S::Cons(operator, vec![left])
            };

            continue;
        }

        if let Some((left_bp, right_bp)) = infix_binding_power(operator) {
            if left_bp < min_bp {
                break;
            }

            lexer.next();
            let right = expr_bin_op(lexer, right_bp);

            left = S::Cons(operator, vec![left, right]);
            continue;
        }
        break;
    }

    return left;
}

fn prefix_binding_power(op: char) -> ((), u8) {
    match op {
        '+' | '-' => ((), 5),
        token => panic!("bad operator: {:?}", token)
    }
}

fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    let result = match op { // lowest priority == lowest numbers
        '+' | '-' => (1, 2),
        '*' | '/' => (3, 4),
        '.' => (10, 9), // function composition
        _ => return None,
    };
    return Some(result);
}

fn postfix_binding_power(op: char) -> Option<(u8, ())> {
    let result = match op {
        '!' | '[' => (7, ()), // factorial | indexing
        _ => return None,
    };
    return Some(result);
}

#[test]
fn tests() {
    let s = expr("1");
    assert_eq!(s.to_string(), "1");

    let s = expr("1 + 2 * 3");
    assert_eq!(s.to_string(), "(+ 1 (* 2 3))");

    let s = expr("a + b * c * d + e");
    assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");

    let s = expr("f . g . h");
    assert_eq!(s.to_string(), "(. f (. g h))");

    let s = expr("--1 * 2");
    assert_eq!(s.to_string(), "(* (- (- 1)) 2)");

    let s = expr("--f . g");
    assert_eq!(s.to_string(), "(- (- (. f g)))");

    let s = expr("-9!");
    assert_eq!(s.to_string(), "(- (! 9))");
    
    let s = expr("f . g !");
    assert_eq!(s.to_string(), "(! (. f g))");

    let s = expr("(((0)))");
    assert_eq!(s.to_string(), "0");

    let s = expr("x[0][1]");
    assert_eq!(s.to_string(), "([ ([ x 0) 1)");
}