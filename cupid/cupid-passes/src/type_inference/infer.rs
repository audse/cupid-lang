use cupid_types::infer::Infer;
use crate::{name_resolution as prev_pass, Value, Type, Ident};

impl Infer<Type> for prev_pass::Decl {
    fn infer(&self) -> Type {
        Type::none()
    }
}

impl Infer<Type> for prev_pass::Function {
    fn infer(&self) -> Type {
        todo!()
    }
}

impl Infer<Type> for Value {
	fn infer(&self) -> Type {
        use Value::*;
        let attr = self.attr();
		match self {
			// VBoolean(x, ..) => Ident::new(x.infer(), attr),
			// VChar(x, ..) => Ident::new(x.infer(), attr),
			// VDecimal(x, y, ..) => Ident::new((*x, *y).infer(), attr),
			// VInteger(x, ..) => Ident::new(x.infer(), attr),
			// VString(x, ..) => Ident::new(x.infer(), attr),
            // VType(x, ..) => x.infer(),
			// _ => Ident::new(().infer(), attr),
            _ => Ident::default(),
		}.into()
	}
}

impl Infer<Ident> for Type {
    fn infer(&self) -> Ident {
        Ident::new("type!", self.attr)
    }
}