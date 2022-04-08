use std::fmt::{
    Debug,
    Display,
    Formatter,
    Result,
};
use crate::{
    CupidScope,
    CupidValue,
};

pub trait Tree {
    fn resolve(&self, scope: &mut CupidScope) -> CupidValue;
}

#[derive(Debug, Hash, Clone)]
pub struct CupidNode {
    pub value: CupidValue,
}

impl CupidNode {
    fn get_value(&self) -> CupidValue {
        return self.value.clone();
    }
}

impl Display for CupidNode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Node Value: {}", self.value)
    }
}
impl Tree for CupidNode {
    fn resolve(&self, _scope: &mut CupidScope) -> CupidValue {
        return self.get_value();
    }
}