use std::{cell::RefCell, fmt, ops, rc::Rc};

#[derive(Clone)]
pub struct Pointer<T>(pub Rc<RefCell<T>>);

impl<T> Pointer<T> {
    pub fn new(inner: T) -> Self {
        Pointer(Rc::new(RefCell::new(inner)))
    }
}

impl<T> ops::Deref for Pointer<T> {
    type Target = Rc<RefCell<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for Pointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: fmt::Debug> fmt::Debug for Pointer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.borrow().fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for Pointer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.borrow().fmt(f)
    }
}
