use std::fmt::{Debug, Display, Formatter, Result};
use crate::{Scope, Value, Token};

pub trait Tree {
    fn resolve(&self, scope: &mut Scope) -> Value;
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Node {
    pub tokens: Vec<Token>,
    pub value: Value,
}

impl Node {
    fn get_value(&self) -> Value {
        self.value.clone()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Node Value: {}", self.value)
    }
}
impl Tree for Node {
    fn resolve(&self, _scope: &mut Scope) -> Value {
        self.get_value()
    }
}