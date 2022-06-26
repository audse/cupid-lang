use crate::{infer::Infer, Str};

// Integers

impl Infer<Str> for i32 {
    fn infer(&self) -> Str { "int".into() }
}

impl Infer<Str> for usize {
    fn infer(&self) -> Str { "int".into() }
}

// Booleans

impl Infer<Str> for bool {
    fn infer(&self) -> Str { "bool".into() }
}

// Strings

impl Infer<Str> for String {
    fn infer(&self) -> Str { "string".into() }
}

impl Infer<Str> for &str {
    fn infer(&self) -> Str { "string".into() }
}

impl Infer<Str> for Str {
    fn infer(&self) -> Str { "string".into() }
}

// Chars

impl Infer<Str> for char {
    fn infer(&self) -> Str { "char".into() }
}

// Decimals

impl Infer<Str> for f64 {
    fn infer(&self) -> Str { "dec".into() }
}

impl Infer<Str> for (i32, u32) {
    fn infer(&self) -> Str { "dec".into() }
}

// None

impl Infer<Str> for () {
    fn infer(&self) -> Str { "none".into() }
}