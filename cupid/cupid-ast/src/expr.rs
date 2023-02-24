use crate::{
    attr::{Attr, GetAttr},
    stmt, types,
};
use derive_more::{From, IsVariant, TryInto, Unwrap};

pub mod block;
pub mod function;
pub mod function_call;
pub mod ident;
pub mod namespace;
pub mod value;

#[derive(
    Debug, Default, Clone, From, TryInto, IsVariant, Unwrap, serde::Serialize, serde::Deserialize,
)]
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
    Empty,
}

impl GetAttr for Expr {
    fn attr(&self) -> Attr {
        use Expr::*;
        match self {
            Block(b) => b.attr,
            Function(f) => f.attr,
            FunctionCall(f) => f.attr,
            Namespace(n) => n.attr,
            Ident(i) => i.attr,
            Value(v) => v.attr,
            Trait(t) => t.attr,
            Type(t) => t.attr,
            _ => panic!("No attributes found for node"),
        }
    }
}
