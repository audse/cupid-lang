use std::fmt::{Display, Formatter, Result};
use std::collections::HashMap;
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
    Map(Map),
    PropertyAccess(PropertyAccess),
    PropertyAssign(PropertyAssign),
    Empty,
    WhileLoop(WhileLoop),
    ForInLoop(ForInLoop),
    DefineType(DefineType),
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
        if string.len() > 1 {
            if let Some(first) = string.chars().next() {
                if first == '"' || first == '\'' { 
                    let mut new_string = string.clone(); //"
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
		Self::Symbol(Symbol { identifier, token: tokens[0].clone() })
	}
    pub fn new_assign(operator: Token, symbol: Expression, value: Expression) -> Self {
        Self::Assign(Assign {
            operator,
            symbol: Expression::to_symbol(symbol),
            value: Box::new(value),
        })
    }
    pub fn new_declare(symbol: Expression, r#type: TypeSymbol, mutable: bool, deep_mutable: bool, value: Expression) -> Self {
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
    pub fn new_map(entries: Vec<(Expression, (usize, Expression))>, token: Token, r#type: Type) -> Self {
        Self::Map(Map {
            entries: HashMap::from_iter(entries.into_iter()),
            token,
            r#type,
        })
    }
    pub fn new_property_access(map: Expression, term: Expression, token: Token) -> Self {
        Self::PropertyAccess(PropertyAccess {
            map: Box::new(map),
            term: Box::new(term),
            operator: token
        })
    }
    pub fn new_property_assign(access: Expression, value: Expression, token: Token) -> Self {
        if let Expression::PropertyAccess(access) = access {
            return Self::PropertyAssign(PropertyAssign {
                access,
                value: Box::new(value),
                operator: token
            });
        }
        unreachable!()
    }
    pub fn new_while_loop(condition: Expression, body: Expression, token: Token) -> Self {
        Self::WhileLoop(WhileLoop {
            body: Self::to_block(body),
            condition: Box::new(condition),
            token
        })
    }
    pub fn new_for_in_loop(params: Vec<Symbol>, map: Expression, body: Expression, token: Token) -> Self {
        Self::ForInLoop(ForInLoop {
            params,
            map: Box::new(map),
            body: Self::to_block(body),
            token
        })
    }
    pub fn new_define_type(token: Token, type_value: Type) -> Self {
        Expression::DefineType(DefineType { token, type_value })
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
        if let Expression::Block(block) = expression {
            block
        } else {
            panic!("Expected a block, got: {:?}", expression)
        }
    }
    pub fn resolve_file(&self, scope: &mut LexicalScope) -> Vec<Value> {
        if let Expression::File(file) = self {
            let mut values = vec![];
            for exp in file {
                values.push(exp.resolve(scope))
            }
            return values;
        }
        vec![]
    }
}

impl Tree for Expression {
    fn resolve(&self, scope: &mut LexicalScope) -> Value {
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
            Self::File(x) => {
                let mut values = vec![];
                for exp in x {
                    values.push(exp.resolve(scope))
                }
                values.pop().unwrap_or(Value::None)
            },
            Self::Logger(x) => x.resolve(scope),
            Self::Map(x) => x.resolve(scope),
            Self::PropertyAccess(x) => x.resolve(scope),
            Self::PropertyAssign(x) => x.resolve(scope),
            Self::WhileLoop(x) => x.resolve(scope),
            Self::ForInLoop(x) => x.resolve(scope),
            Self::DefineType(x) => x.resolve(scope),
            _ => Value::None,
        }
    }
}
