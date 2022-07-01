use derive_more::{From, TryInto, IsVariant, Unwrap};
use crate::{stmt, types, attr::Attr};

pub mod block;
pub mod function;
pub mod function_call;
pub mod ident;
pub mod namespace;
pub mod value;

#[derive(Debug, Default, Clone, From, TryInto, IsVariant, Unwrap, serde::Serialize, serde::Deserialize)]
pub enum Expr {
    Block(block::Block),
    Function(function::Function),
    FunctionCall(function_call::FunctionCall),
    Ident(ident::Ident),
    Namespace(namespace::Namespace),
    Value(value::Value),
    Trait(types::traits::Trait),
    Type(types::typ::Type),
    Stmt(stmt::Stmt),
    #[default]
    Empty
}

impl Expr {
    pub fn attr(&self) -> Option<Attr> {
        use Expr::*;
        match self {
            Block(b) => Some(b.attr),
            Function(f) => Some(f.attr),
            FunctionCall(f) => Some(f.attr),
            Namespace(n) => Some(n.attr),
            Ident(i) => Some(i.attr),
            Value(v) => Some(v.attr),
            Trait(t) => Some(t.attr),
            Type(t) => Some(t.attr),
            _ => None
        }
    }
}