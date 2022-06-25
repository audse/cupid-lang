use crate::Address;

#[derive(Debug, Default, Copy, Clone)]
pub struct Attr {
    pub source: Address,
    pub scope: Address,
}