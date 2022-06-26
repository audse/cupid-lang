use crate::{infer::Infer, Str};
use cupid_ast::{expr::ident::Ident, types::typ::Type};

impl<Item: Infer<Str>> Infer<Ident> for Item {
    fn infer(&self) -> Ident {
        Ident {
            name: self.infer(),
            ..Ident::default()
        }
    }
}

impl<Item: Infer<Ident>> Infer<Type> for Item {
    fn infer(&self) -> Type {
        Type {
            ident: self.infer(),
            base: cupid_ast::types::typ::BaseType::Struct,
            ..Default::default()
        }
    }
}
