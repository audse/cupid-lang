use std::hash::{Hash, Hasher};
use crate::{LexicalScope, Value, Token};

pub trait Tree {
    fn resolve(&self, scope: &mut LexicalScope) -> Value;
}

#[derive(Debug, Clone)]
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

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.tokens.hash(state);
    }
}