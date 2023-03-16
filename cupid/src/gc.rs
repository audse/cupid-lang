use std::fmt::Debug;
use std::ptr::NonNull;
use std::{alloc, mem};
use std::{
    ops::{Deref, DerefMut},
    sync::atomic::AtomicUsize,
    usize,
};

use crate::{
    objects::{
        Array, BoundMethod, Class, Closure, Function, Instance, ObjectType, RoleImpl, Str, Upvalue,
    },
    table::Table,
    value::Value,
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GcObject {
    marked: bool,
    next: Option<NonNull<GcObject>>,
    obj_type: ObjectType,
}

impl GcObject {
    pub fn new(obj_type: ObjectType) -> Self {
        Self {
            marked: false,
            next: None,
            obj_type,
        }
    }
}

#[derive(Debug)]
pub struct GcRef<T: Debug> {
    pointer: NonNull<T>,
}

impl<T: Debug> GcRef<T> {
    pub fn dangling() -> GcRef<T> {
        GcRef {
            pointer: NonNull::dangling(),
        }
    }
}

impl<T: Debug> Deref for GcRef<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.pointer.as_ref() }
    }
}

impl<T: Debug> DerefMut for GcRef<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.pointer.as_mut() }
    }
}

impl<T: Debug> Copy for GcRef<T> {}

impl<T: Debug> Clone for GcRef<T> {
    fn clone(&self) -> GcRef<T> {
        *self
    }
}

impl<T: Debug> Eq for GcRef<T> {}

impl<T: Debug> PartialEq for GcRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.pointer == other.pointer
    }
}

#[derive(Debug, Clone)]
pub struct Gc {
    next_gc: usize,
    first: Option<NonNull<GcObject>>,
    strings: Table,
    grey_stack: Vec<NonNull<GcObject>>,
}

impl Default for Gc {
    fn default() -> Self {
        Self {
            next_gc: 1024 * 1024,
            first: None,
            strings: Table::default(),
            grey_stack: Vec::new(),
        }
    }
}

impl Gc {
    const HEAP_GROW_FACTOR: usize = 2;

    pub fn alloc<T: Debug + 'static>(&mut self, object: T) -> GcRef<T> {
        unsafe {
            let boxed = Box::new(object);
            let pointer = NonNull::new_unchecked(Box::into_raw(boxed));
            let mut header: NonNull<GcObject> = mem::transmute(pointer.as_ref());
            header.as_mut().next = self.first.take();
            self.first = Some(header);

            GcRef { pointer }
        }
    }

    pub fn intern(&mut self, s: impl Into<String>) -> GcRef<Str> {
        let ls = Str::from_string(s.into());
        if let Some(value) = self.strings.find_string(&ls.s, ls.hash) {
            value
        } else {
            let reference = self.alloc(ls);
            self.strings.set(reference, Value::Nil);
            reference
        }
    }

    pub fn collect_garbage(&mut self) {
        self.trace_references();
        self.remove_white_strings();
        self.sweep();
        self.next_gc = GLOBAL.bytes_allocated() * Gc::HEAP_GROW_FACTOR;
    }

    fn trace_references(&mut self) {
        while let Some(pointer) = self.grey_stack.pop() {
            self.blacken_object(pointer);
        }
    }

    fn blacken_object(&mut self, pointer: NonNull<GcObject>) {
        let object_type = unsafe { &pointer.as_ref().obj_type };

        match object_type {
            ObjectType::Array => {
                let _: &Array = unsafe { mem::transmute(pointer.as_ref()) };
            }
            ObjectType::Function => {
                let function: &Function = unsafe { mem::transmute(pointer.as_ref()) };
                self.mark_object(function.name);
                for &constant in &function.chunk.constants {
                    self.mark_value(constant);
                }
            }
            ObjectType::Closure => {
                let closure: &Closure = unsafe { mem::transmute(pointer.as_ref()) };
                self.mark_object(closure.function);
                for &upvalue in &closure.upvalues {
                    self.mark_object(upvalue);
                }
            }
            ObjectType::Str => {}
            ObjectType::Upvalue => {
                let upvalue: &Upvalue = unsafe { mem::transmute(pointer.as_ref()) };
                if let Some(obj) = upvalue.closed {
                    self.mark_value(obj)
                }
            }
            ObjectType::Class => {
                let class: &Class = unsafe { mem::transmute(pointer.as_ref()) };
                self.mark_object(class.name);
                self.mark_table(&class.methods);
            }
            ObjectType::Instance => {
                let instance: &Instance = unsafe { mem::transmute(pointer.as_ref()) };
                self.mark_object(instance.class);
                self.mark_table(&instance.fields);
            }
            ObjectType::BoundMethod => {
                let method: &BoundMethod = unsafe { mem::transmute(pointer.as_ref()) };
                self.mark_value(method.receiver);
                self.mark_object(method.method);
            }
            ObjectType::Role => {
                let role: &RoleImpl = unsafe { mem::transmute(pointer.as_ref()) };
                self.mark_object(role.class);
                self.mark_table(&role.methods);
            }
        }
    }

    pub fn mark_value(&mut self, value: Value) {
        match value {
            Value::Array(value) => self.mark_object(value),
            Value::BoundMethod(value) => self.mark_object(value),
            Value::Class(value) => self.mark_object(value),
            Value::Closure(value) => self.mark_object(value),
            Value::Function(value) => self.mark_object(value),
            Value::Instance(value) => self.mark_object(value),
            Value::String(value) => self.mark_object(value),
            _ => (),
        }
    }

    pub fn mark_object<T: 'static + Debug>(&mut self, mut reference: GcRef<T>) {
        unsafe {
            let mut header: NonNull<GcObject> = mem::transmute(reference.pointer.as_mut());
            header.as_mut().marked = true;
            self.grey_stack.push(header);
        }
    }

    pub fn mark_table(&mut self, table: &Table) {
        for (k, v) in table.iter() {
            self.mark_object(k);
            self.mark_value(v);
        }
    }

    pub fn should_gc(&self) -> bool {
        GLOBAL.bytes_allocated() > self.next_gc
    }

    fn sweep(&mut self) {
        let mut previous: Option<NonNull<GcObject>> = None;
        let mut current: Option<NonNull<GcObject>> = self.first;
        while let Some(mut object) = current {
            unsafe {
                let object_ptr = object.as_mut();
                current = object_ptr.next;
                if object_ptr.marked {
                    object_ptr.marked = false;
                    previous = Some(object);
                } else if let Some(mut previous) = previous {
                    previous.as_mut().next = object_ptr.next
                } else {
                    self.first = object_ptr.next
                }
            }
        }
    }

    fn remove_white_strings(&mut self) {
        for (k, _v) in self.strings.iter() {
            let header: &GcObject = unsafe { mem::transmute(k.pointer.as_ref()) };
            if !header.marked {
                self.strings.delete(k);
            }
        }
    }
}

struct GlobalAllocator {
    bytes_allocated: AtomicUsize,
}

impl GlobalAllocator {
    fn bytes_allocated(&self) -> usize {
        self.bytes_allocated.load(std::sync::atomic::Ordering::Relaxed)
    }
}

unsafe impl alloc::GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        self.bytes_allocated
            .fetch_add(layout.size(), std::sync::atomic::Ordering::Relaxed);
        mimalloc::MiMalloc.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::Layout) {
        mimalloc::MiMalloc.dealloc(ptr, layout);
        self.bytes_allocated
            .fetch_sub(layout.size(), std::sync::atomic::Ordering::Relaxed);
    }
}

#[global_allocator]
static GLOBAL: GlobalAllocator = GlobalAllocator {
    bytes_allocated: AtomicUsize::new(0),
};
