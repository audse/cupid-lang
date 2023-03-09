use std::ptr::null;

pub struct FrameState<'frame, 'chunk> {
    pub frame: &'frame mut CallFrame,
    pub chunk: &'chunk Chunk,
}

impl FrameState<'_, '_> {
    pub fn instruction(&self) -> Instruction {
        unsafe { *self.frame.ip }
    }

    pub fn set_instruction(&mut self, offset: isize) {
        self.frame.ip = unsafe { self.frame.ip.offset(offset) };
    }

    pub fn set(&mut self, frames: &mut Frames) {
        self.frame = frames.current_frame();
        self.chunk = frames.current_chunk();
    }
}

use crate::{
    chunk::{Chunk, Instruction},
    gc::GcRef,
    objects::Closure,
};

#[derive(Debug, Clone)]
pub struct Frames {
    pub frames: [CallFrame; Frames::MAX],
    pub count: usize,
}

impl Default for Frames {
    fn default() -> Self {
        Self {
            frames: [CallFrame {
                closure: GcRef::dangling(),
                ip: null(),
                slot: 0,
            }; Frames::MAX],
            count: 0,
        }
    }
}

impl Frames {
    pub const MAX: usize = 64;

    pub fn frame(&self) -> &CallFrame {
        &self.frames[self.count - 1]
    }

    pub fn current_frame<'this, 'frame>(&'this mut self) -> &'frame mut CallFrame {
        unsafe { &mut *(&mut self.frames[self.count - 1] as *mut CallFrame) }
    }

    pub fn current_chunk<'this, 'chunk>(&'this mut self) -> &'chunk Chunk {
        &self.current_frame().closure.function.chunk
    }

    pub fn state<'this, 'frame, 'chunk>(&'this mut self) -> FrameState<'frame, 'chunk> {
        FrameState {
            frame: self.current_frame(),
            chunk: self.current_chunk(),
        }
    }

    pub fn increment(&mut self, next: CallFrame) {
        self.frames[self.count] = next;
        self.count += 1;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CallFrame {
    pub closure: GcRef<Closure>,
    pub ip: *const Instruction,
    pub slot: usize,
}

impl CallFrame {
    pub fn new(closure: GcRef<Closure>, slot: usize) -> Self {
        CallFrame {
            closure,
            ip: closure.function.chunk.code.as_ptr(),
            slot,
        }
    }

    pub fn offset(&self) -> usize {
        unsafe {
            let chunk = &self.closure.function.chunk;
            let pos = self.ip.offset_from(chunk.code.as_ptr());
            pos as usize
        }
    }

    pub fn line(&self) -> usize {
        self.closure.function.chunk.lines[self.offset() - 1]
    }
}
