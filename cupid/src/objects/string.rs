use std::fmt;

use crate::{gc::GcObject, objects::ObjectType};

#[repr(C)]
pub struct Str {
    pub header: GcObject,
    pub s: String,
    pub hash: usize,
}

impl Str {
    pub fn from_string(s: String) -> Self {
        let hash = Str::hash_string(&s);
        Str {
            header: GcObject::new(ObjectType::Str),
            s,
            hash,
        }
    }

    fn hash_string(s: &str) -> usize {
        let mut hash: usize = 2166136261;
        for b in s.bytes() {
            hash ^= b as usize;
            hash = hash.wrapping_mul(16777619);
        }
        hash
    }
}

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}
