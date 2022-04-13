use crate::{LexicalScope, Value, Token};

pub trait Tree {
    fn resolve(&self, scope: &mut LexicalScope) -> Value;
}

#[derive(Debug, Hash, Clone)]
pub struct Node {
    pub tokens: Vec<Token>,
    pub value: Value,
}

impl Node {
    fn get_value(&self) -> Value {
        self.value.clone()
    }
}

impl Tree for Node {
    fn resolve(&self, _scope: &mut LexicalScope) -> Value {
        self.get_value()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Node {}