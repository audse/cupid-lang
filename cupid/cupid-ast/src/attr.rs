use crate::Address;

#[derive(Debug, Default, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Attr {
    pub source: Address,
    pub scope: Address,
}

pub trait GetAttr {
    fn attr(&self) -> Attr;
    fn attr_mut(&mut self) -> &mut Attr;
}

impl<T: GetAttr> GetAttr for std::cell::RefCell<T> {
    fn attr(&self) -> Attr {
        self.borrow().attr()
    }
    fn attr_mut(&mut self) -> &mut Attr {
        self.get_mut().attr_mut()
    }
}
