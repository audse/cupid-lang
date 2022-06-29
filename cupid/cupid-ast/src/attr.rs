use crate::Address;

#[derive(Debug, Default, Copy, Clone)]
pub struct Attr {
    pub source: Address,
    pub scope: Address,
}

pub trait GetAttr {
    fn attr(&self) -> Attr;
    fn attr_mut(&mut self) -> &mut Attr;
}