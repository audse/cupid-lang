use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::*;

#[derive(Debug, Hash, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expression {
    File(Vec<Expression>),
    Block(Block),
    BoxBlock(BoxBlock),
    IfBlock(IfBlock),
    Operator(Operator),
    Assign(Assign),
    Declare(Declare),
    Node(Node),
    Symbol(Symbol),
    TypeSymbol(Symbol),
    Function(Function),
    FunctionCall(FunctionCall),
    Logger(Logger),
    Array(Array),
    Map(Map),
    Property(Property),
    Empty,
    WhileLoop(WhileLoop),
    ForInLoop(ForInLoop),
    DefineType(DefineType),
    DefineAlias(DefineAlias),
    Break(Break),
    Return(Return),
    BuiltInType(BuiltInType),
    DefineStruct(DefineStruct),
    DefineSum(DefineSum),
    ArrayTypeHint(ArrayTypeHint),
    MapTypeHint(MapTypeHint),
    StructTypeHint(StructTypeHint),
    FunctionTypeHint(FunctionTypeHint),
    PropertyAssign(PropertyAssign),
    Implement(Implement),
	DefineTrait(DefineTrait),
	ImplementTrait(ImplementTrait),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> DisplayResult {
        match self {
            Self::File(a) => a.iter().try_for_each(|e| write!(f, "{:?}", e)),
            Self::Node(node) => write!(f, "{}", node.value),
            Self::Array(array) => {
                let items: Vec<String> = array.items.iter().map(|i| i.to_string()).collect();
                write!(f, "array [{:#?}]", items.join(", "))
            },
            Self::Map(map) => {
                let items: Vec<String> = map.items
                    .iter()
                    .map(|(key, value)| format!("{key}: {value}"))
                    .collect();
                write!(f, "map [{:#?}]", items.join(", "))
            },
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
    pub fn new_char_node(string: String, tokens: Vec<Token>) -> Self {
        let c = string.chars().next().unwrap_or('\0');
        Self::new_node(Value::Char(c), tokens)
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
    pub fn new_declare(symbol: Expression, value_type: Expression, mutable: bool, deep_mutable: bool, value: Expression) -> Self {
        Self::Declare(Declare {
            symbol: Expression::to_symbol(symbol),
            value: Box::new(value),
            value_type: Box::new(value_type),
			mutable,
			deep_mutable,
        })
    }
    pub fn new_block(expressions: Vec<Expression>) -> Self {
        Self::Block(Block { expressions })
    }
    pub fn new_box_block(expressions: Vec<Expression>) -> Self {
        Self::BoxBlock(BoxBlock { expressions })
    }
    pub fn new_if_block(condition: Expression, body: Block, else_if_bodies: Vec<(Expression, Block)>, else_body: Option<Block>) -> Self {
        Self::IfBlock(IfBlock {
            condition: Box::new(condition),
            body,
			else_if_bodies,
            else_body,
        })
    }
	pub fn new_function(params: Vec<(Expression, Symbol)>, body: Expression, use_self: bool) -> Self {
		Expression::Function(Function {
			params,
			body: Box::new(body),
			use_self
		})
	}
	pub fn new_function_call(fun: Expression, args: Vec<Expression>) -> Self {
		Expression::FunctionCall(FunctionCall {
			fun: Expression::to_symbol(fun),
			args: Args(args),
		})
	}
    pub fn new_array(items: Vec<Expression>) -> Self {
        Self::Array(Array { items })
    }
    pub fn new_map(items: Vec<(Expression, Expression)>, token: Token) -> Self {
        Self::Map(Map {
            items,
            token
        })
    }
    pub fn new_property(map: Expression, term: Expression, token: Token) -> Self {
        Self::Property(Property {
            map: Box::new(map),
            term: Box::new(term),
            token
        })
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
    pub fn new_define_type(token: Token, type_symbol: Symbol, type_value: TypeKind) -> Self {
        Expression::DefineType(DefineType { token, type_symbol, type_value })
    }
    pub fn new_define_struct(token: Token, symbol: Symbol, members: Vec<(Symbol, Expression)>, generics: Vec<Symbol>) -> Self {
        Expression::DefineStruct(DefineStruct { token, symbol, members, generics })
    }
    pub fn new_define_sum(token: Token, symbol: Symbol, types: Vec<Expression>, generics: Vec<Symbol>) -> Self {
        Expression::DefineSum(DefineSum { token, symbol, types, generics })
    }
    pub fn new_define_type_alias(token: Token, symbol: Symbol, true_type: Expression, generics: Vec<Symbol>) -> Self {
        Expression::DefineAlias(DefineAlias { token, symbol, true_type: Box::new(true_type), generics })
    }
    pub fn new_break(token: Token, value: Expression) -> Self {
        Expression::Break(Break { token, value: Box::new(value) })
    }
    pub fn new_return(token: Token, value: Expression) -> Self {
        Expression::Return(Return { token, value: Box::new(value) })
    }
    pub fn new_property_assign(property: Property, value: Expression, operator: Token) -> Self {
        Expression::PropertyAssign(PropertyAssign { property, value: Box::new(value), operator })
    }
    pub fn new_continue(token: Token) -> Self {
        Self::new_node(Value::Continue, vec![token])
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
	pub fn get_value_and_type(&self, scope: &mut LexicalScope) -> Result<(Value, TypeKind), Value> {
		let value = match self.resolve(scope) {
			Value::Error(e) => return Err(Value::Error(e)),
			v => v,
		};
		if let Expression::Symbol(map_symbol) = &self {
			// get implementation of stored symbol's type
			match scope.get_type_of_symbol(&map_symbol) {
				Ok(type_value) => Ok((value, type_value)),
				Err(e) => Err(map_symbol.error(e)),
			}
		} else {
			// get implementation of non-symbol term
			match scope.get_type(&value) {
				Ok(type_value) => Ok((value, type_value)),
				Err(e) => panic!("{e}")
			}
		}
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
            Self::BoxBlock(b) => b.resolve(scope),
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
            Self::Property(x) => x.resolve(scope),
            Self::Array(x) => x.resolve(scope),
            Self::WhileLoop(x) => x.resolve(scope),
            Self::ForInLoop(x) => x.resolve(scope),
            Self::DefineType(x) => x.resolve(scope),
            Self::DefineAlias(x) => x.resolve(scope),
            Self::Break(x) => x.resolve(scope),
            Self::Return(x) => x.resolve(scope),
            Self::BuiltInType(x) => x.resolve(scope),
            Self::DefineStruct(x) => x.resolve(scope),
            Self::DefineSum(x) => x.resolve(scope),
            Self::ArrayTypeHint(x) => x.resolve(scope),
            Self::MapTypeHint(x) => x.resolve(scope),
            Self::StructTypeHint(x) => x.resolve(scope),
            Self::FunctionTypeHint(x) => x.resolve(scope),
            Self::PropertyAssign(x) => x.resolve(scope),
			Self::Implement(x) => x.resolve(scope),
			Self::DefineTrait(x) => x.resolve(scope),
			Self::ImplementTrait(x) => x.resolve(scope),
            _ => Value::None,
        }
    }
}