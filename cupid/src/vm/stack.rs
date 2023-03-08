use std::ptr::null_mut;

use crate::value::Value;

use super::Frames;

pub struct Stack {
    pub stack: [Value; Stack::SIZE],
    pub top: *mut Value,
}

impl Stack {
    pub const SIZE: usize = Frames::MAX * (std::u8::MAX as usize) + 1;

    pub fn push(&mut self, v: Value) {
        unsafe {
            *self.top = v;
            self.top = self.top.offset(1);
        }
    }

    pub fn pop(&mut self) -> Value {
        unsafe {
            self.top = self.top.offset(-1);
            *self.top
        }
    }

    pub fn peek(&self, n: usize) -> Value {
        unsafe { *self.top.offset(-1 - n as isize) }
    }

    pub fn truncate(&mut self, index: usize) {
        unsafe { self.top = self.stack.as_mut_ptr().add(index) }
    }

    pub fn len(&self) -> usize {
        unsafe { self.top.offset_from(self.stack.as_ptr()) as usize }
    }

    pub fn set_at(&mut self, n: usize, value: Value) {
        unsafe {
            let pos = self.top.offset(-1 - (n as isize));
            *pos = value
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            stack: [Value::Nil; Stack::SIZE],
            top: null_mut(),
        }
    }
}
