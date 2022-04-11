use std::fmt::{Display, Formatter, Result};
use crate::*;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Expression {
    File(Vec<Expression>),
    Block(Block),
    IfBlock(IfBlock),
    Operator(Operator),
    Assign(Assign),
    Declare(Declare),
    Node(Node),
    Symbol(Symbol),
    Function(Function),
    FunctionCall(FunctionCall),
    Logger(Logger),
    Empty,
}

pub struct Exp {
    pub exp: Expression,
    pub errors: Vec<Expression>
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::File(a) => a.iter().try_for_each(|e| write!(f, "{:?}", e)),
            e => write!(f, "{:?}", e),
        }
    }
}

impl Expression {
    pub fn new_node(value: Value, tokens: Vec<Token>) -> Self {
        Self::Node(Node { value, tokens, })
    }
    pub fn new_string_node(string: String, tokens: Vec<Token>) -> Self {
		// let string_slice = if string.len() > 1 {
		// 	string[1..string.len() - 1].to_string()
		// } else { string };
        if string.len() > 1 {
            if let Some(first) = string.chars().nth(0) {
                if first == '"' || first == '\'' {
                    let mut new_string = string.clone();
                    new_string.remove(0);
                    new_string.pop();
                    return Self::new_node(Value::String(new_string), tokens);
                }
            }
        }
        Self::new_node(Value::String(string), tokens)
    }
    pub fn new_int_node(int: i32, tokens: Vec<Token>) -> Self {
        Self::new_node(Value::Integer(int), tokens)
    }
    pub fn new_dec_node(front: i32, back: u32, tokens: Vec<Token>) -> Self {
        Self::new_node(Value::Decimal(front, back), tokens)
    }
    pub fn new_bool_node(boo: bool, tokens: Vec<Token>) -> Self {
        Self::new_node(Value::Boolean(boo), tokens)
    }
    pub fn new_none_node(tokens: Vec<Token>) -> Self {
        Self::new_node(Value::None, tokens)
    }
    pub fn new_operator(operator: Token, left: Expression, right: Expression) -> Self {
        Self::Operator(Operator::new(operator, left, right))
    }
	pub fn new_symbol(identifier: Expression) -> Self {
        let (identifier, tokens) = Expression::to_value(identifier);
		Self::Symbol(Symbol(identifier, tokens))
	}
    pub fn new_assign(operator: Token, symbol: Expression, value: Expression) -> Self {
        Self::Assign(Assign {
            operator,
            symbol: Expression::to_symbol(symbol),
            value: Box::new(value),
        })
    }
    pub fn new_declare(symbol: Expression, r#type: Type, mutable: bool, deep_mutable: bool, value: Expression) -> Self {
        Self::Declare(Declare {
            symbol: Expression::to_symbol(symbol),
            value: Box::new(value),
            r#type,
			mutable,
			deep_mutable,
        })
    }
    pub fn new_block(expressions: Vec<Expression>) -> Self {
        Self::Block(Block { expressions })
    }
    pub fn new_if_block(condition: Expression, body: Block, else_if_bodies: Vec<(Expression, Block)>, else_body: Option<Block>) -> Self {
        Self::IfBlock(IfBlock {
            condition: Box::new(condition),
            body,
			else_if_bodies,
            else_body,
        })
    }
	pub fn new_function(params: Vec<Symbol>, body: Expression) -> Self {
		Expression::Function(Function {
			params,
			body: Box::new(body),
		})
	}
	pub fn new_function_call(fun: Expression, args: Vec<Expression>) -> Self {
		Expression::FunctionCall(FunctionCall {
			fun: Expression::to_symbol(fun),
			args: Args(args),
		})
	}
	pub fn to_symbol(expression: Self) -> Symbol {
		if let Expression::Symbol(symbol) = expression {
			symbol
		} else { 
			panic!("Node is not a symbol: {:?}", expression) 
		}
	}
	pub fn to_value(expression: Self) -> (Value, Vec<Token>) {
		if let Expression::Node(Node { value, tokens }) = expression {
			(value, tokens)
		} else { 
			panic!("Expression is not a node: {:?}", expression) 
		}
	}
	pub fn to_block(expression: Self) -> Block {
		if let Expression::Block(Block { expressions }) = expression {
			Block { expressions }
		} else {
			panic!("Expected a block, got: {:?}", expression)
		}
	}
    pub fn resolve_file(&self, scope: &mut Scope) -> Vec<Value> {
        match self {
            Self::File(x) => x.iter().map(|y| y.resolve(scope)).collect(),
            _ => vec![Value::None]
        }
    }
}

impl Tree for Expression {
    fn resolve(&self, scope: &mut Scope) -> Value {
        match self {
            Self::Node(n) => n.resolve(scope),
            Self::Operator(o) => o.resolve(scope),
            Self::Symbol(s) => s.resolve(scope),
            Self::Assign(a) => a.resolve(scope),
            Self::Declare(a) => a.resolve(scope),
            Self::Block(b) => b.resolve(scope),
            Self::IfBlock(b) => b.resolve(scope),
            Self::Function(b) => b.resolve(scope),
            Self::FunctionCall(b) => b.resolve(scope),
            Self::File(x) => x.iter().map(|y| y.resolve(scope)).last().unwrap_or(Value::None),
            Self::Logger(x) => x.resolve(scope),
            _ => Value::None,
        }
    }
}
