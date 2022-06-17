type Str = std::borrow::Cow<'static, str>;

pub trait Infer<Returns> {
    fn infer(&self) -> Returns;
}

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

// Array

impl<T, Item: Infer<T>> Infer<Option<T>> for Vec<Item> {
    fn infer(&self) -> Option<T> { 
        self.get(0).map(|item| item.infer())
    }
}

// Tuple

impl<T> Infer<Vec<T>> for Vec<&dyn Infer<T>> {
    fn infer(&self) -> Vec<T> {
        self.iter().map(|item| item.infer()).collect()
    }
}

// None

impl Infer<Str> for () {
    fn infer(&self) -> Str { "none".into() }
}

// Utils

impl<T, Item: Infer<T>> Infer<Option<T>> for Option<Item> {
    fn infer(&self) -> Option<T> {
        self.as_ref().map(|item| item.infer())
    }
}