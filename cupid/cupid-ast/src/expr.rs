use derive_more::{From, TryInto, IsVariant, Unwrap};

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
    #[default]
    Empty
}