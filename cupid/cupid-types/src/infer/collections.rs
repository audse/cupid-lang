use crate::infer::Infer;

// Array

impl<T: PartialEq, Item: Infer<T>> Infer<Option<T>> for Vec<Item> {
    fn infer(&self) -> Option<T> { 
        let typ = self.get(0).map(|item| item.infer());
        if let Some(typ) = typ && self.iter().all(|item| item.infer() == typ) {
            Some(typ)
        } else {
            panic!("all items must be the same type")
        }
    }
}

// Tuple

impl<T> Infer<Vec<T>> for Vec<&dyn Infer<T>> {
    fn infer(&self) -> Vec<T> {
        self.iter().map(|item| item.infer()).collect()
    }
}