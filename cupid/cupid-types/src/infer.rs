pub mod collections;
pub mod expr;
pub mod ident;
pub mod primitives;
pub mod stmt;

pub trait Infer<Returns> {
    fn infer(&self) -> Returns;
}


// Utils

impl<T, Item: Infer<T>> Infer<Option<T>> for Option<Item> {
    fn infer(&self) -> Option<T> {
        self.as_ref().map(|item| item.infer())
    }
}