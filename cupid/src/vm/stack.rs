use std::ptr::null_mut;

use crate::{value::Value, vm::Frames};

#[derive(Debug, Clone, Copy)]
pub struct Stack<T: Copy + std::fmt::Debug> {
    pub stack: [T; SIZE],
    pub top: *mut T,
}

pub const SIZE: usize = Frames::MAX * (std::u8::MAX as usize) + 1;

impl<T: Copy + std::fmt::Debug> Stack<T> {
    pub fn push(&mut self, v: T) {
        unsafe {
            *self.top = v;
            self.top = self.top.offset(1);
        }
    }

    pub fn pop(&mut self) -> T {
        unsafe {
            self.top = self.top.offset(-1);
            *self.top
        }
    }

    pub fn peek(&self, n: usize) -> T {
        unsafe { *self.top.offset(-1 - n as isize) }
    }

    pub fn truncate(&mut self, index: usize) {
        unsafe { self.top = self.stack.as_mut_ptr().add(index) }
    }

    pub fn len(&self) -> usize {
        unsafe { self.top.offset_from(self.stack.as_ptr()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set_at(&mut self, n: usize, value: T) {
        unsafe {
            let pos = self.top.offset(-1 - (n as isize));
            *pos = value
        }
    }
}

impl Default for Stack<Value> {
    fn default() -> Self {
        Self {
            stack: [Value::Nil; SIZE],
            top: null_mut(),
        }
    }
}
