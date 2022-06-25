use derive_more::{From, TryInto, IsVariant, Unwrap};
use crate::{types, attr::Attr};

pub mod block;
pub mod function;
pub mod ident;
pub mod value;

#[derive(Debug, Default, Clone, From, TryInto, IsVariant, Unwrap)]
pub enum Expr {
    Block(block::Block),
    Function(function::Function),
    Ident(ident::Ident),
    Value(value::Value),
    Trait(types::traits::Trait),
    Type(types::typ::Type),
    #[default]
    Empty
}

impl Expr {
    pub fn attr(&self) -> Attr {
        use Expr::*;
        match self {
            Block(b) => b.attr,
            Function(f) => f.attr,
            Ident(i) => i.attr,
            Value(v) => v.attr,
            Trait(t) => t.attr,
            Type(t) => t.attr,
            _ => panic!()
        }
    }
}