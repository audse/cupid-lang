use crate::*;

pub trait ToError: UseAttributes {
	fn to_err<T>(&self, code: usize) -> ASTResult<T> {
		Err(self.as_err(code))
	}
	fn to_err_mut<T>(&mut self, code: usize) -> ASTResult<T> {
		Err(self.as_err(code))
	}
	fn as_err(&self, code: usize) -> crate::ASTErr;
}

impl ToError for Block {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Block(self.to_owned()), code)
	}
}

impl ToError for Declaration {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Declaration(self.to_owned()), code)
	}
}

impl ToError for Exp {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(self.to_owned(), code)
	}
}

impl ToError for Field {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		self.name.as_err(code)
	}
}

impl ToError for Function {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Function(self.to_owned()), code)
	}
}

impl ToError for FunctionCall {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::FunctionCall(Box::new(self.to_owned())), code)
	}
}

impl ToError for Ident {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Ident(self.to_owned()), code)
	}
}

impl ToError for Implement {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Implement(self.to_owned()), code)
	}
}

impl ToError for Property {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Property(Box::new(self.to_owned())), code)
	}
}

impl ToError for PropertyTerm {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(self.to_owned().into(), code)
	}
}

impl ToError for Method {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Function(self.to_owned().value), code)
	}
}

impl ToError for SymbolValue {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Value(self.value.to_owned().unwrap_or_default()), code)
	}
}

impl ToError for Trait {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Value(VTrait(self.to_owned())), code)
	}
}

impl ToError for TraitDef {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::TraitDef(self.to_owned()), code)
	}
}

impl ToError for Type {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Value(VType(self.to_owned())), code)
	}
}

impl ToError for TypeDef {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::TypeDef(self.to_owned()), code)
	}
}

impl ToError for Value {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		(Exp::Value(self.to_owned()), code)
	}
}

impl<T: ToError + Default + std::fmt::Debug> ToError for Typed<T> {
	fn as_err(&self, code: usize) -> crate::ASTErr {
		self.inner().as_err(code)
	}
}