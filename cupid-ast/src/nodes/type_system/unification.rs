use crate::*;

pub type ASTResult<T> = Result<T, ASTErr>;
pub type UnifyResult = ASTResult<()>;

#[allow(unused_variables)]
pub trait Unify {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		Ok(())
	}
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		Ok(())
	}
	fn can_unify(&self, other: &Self) -> bool {
		false
	}
}

fn unify<T: Unify + ?Sized>(param: &mut T, arg: Option<&T>) -> UnifyResult {
	if let Some(arg) = arg {
		param.unify(arg)
	} else {
		Ok(())
	}
}

impl Unify for Ident {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		if self.name != other.name {
			return Err((self.src(), ERR_CANNOT_UNIFY))
		}
		self.attributes.generics.unify(&*other.attributes.generics)
	}
	fn unify_with(&mut self, other: &[Typed<Ident>] ) -> UnifyResult {
		self.attributes.generics.unify(&other.to_vec())
	}
	fn can_unify(&self, other: &Self) -> bool {
		self.name == other.name 
			&& self.attributes.generics.can_unify(&other.attributes.generics)
	}
}

impl Unify for Typed<Ident> {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		match (&self, other) {
			(Untyped(_), IsTyped(_, t)) => {
				self.to_typed(t.to_owned());
			},
			(IsTyped(..), Untyped(..)) => (),
			(Untyped(_), Untyped(_)) => (),
			(IsTyped(..), IsTyped(..)) => {
				if self != other {
					return Err((self.src(), ERR_CANNOT_UNIFY))
				}
			}
		};
		Ok(())
	}
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		for generic in other {
			if generic.name == self.name {
				self.unify(generic)?;
			}
		}
		Ok(())
	}
	fn can_unify(&self, other: &Self) -> bool {
		match (&self, other) {
			(Untyped(_), IsTyped(..)) => true,
			(IsTyped(..), Untyped(..)) => true,
			(Untyped(_), Untyped(_)) => true,
			(IsTyped(..), IsTyped(..)) => self == other
		}
	}
}

impl Unify for Vec<Typed<Ident>> {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		let mut other = other.iter();
		for ident in self.iter_mut() {
			if let Untyped(_) = ident {
				unify(ident, other.next())?;
			}
		}
		Ok(())
	}
	fn can_unify(&self, other: &Self) -> bool {
		let mut other = other.iter();
		for ident in self.iter() {
			if let (Untyped(_), Some(next)) = (ident, other.next()) {
				if !ident.can_unify(next) {
					return false;
				}
			}
		}
		true
	}
}

impl Unify for Type {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		self.name.unify(&other.name)?;
		
		if self.fields.len() != other.fields.len() {
			panic!("cannot unify");
		}

		self.fields.unify_with(&**other.attributes().generics)?;

		for method in &mut self.methods {
			method.name.unify(&other.name)?;
		}

		for trait_ident in &mut self.traits {
			trait_ident.unify(&other.name)?;
		}

		Ok(())
	}
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		self.name.unify_with(other)?;
		self.fields.unify_with(other)?;
		
		for method in self.methods.iter_mut() {
			method.unify_with(other)?;
		}

		for trait_ident in self.traits.iter_mut() {
			trait_ident.unify_with(other)?;
		}

		Ok(())
	}
}

impl Unify for Method {
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		// TODO
		self.name.unify_with(other)?;
		Ok(())
	}
	fn can_unify(&self, other: &Self) -> bool {
		self.name.can_unify(&other.name)
	}
}

impl Unify for FieldSet {
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		for (_, field_type) in (*self).iter_mut() {
			field_type.unify_with(other)?;
		}
		Ok(())
	}
	// fn can_unify(&self, other: &Self) -> bool {
	// 	let other = other.iter();
	// 	for (_, field_type) in self.iter() {
	// 		if let Some(next) = field_type.next() {
	// 			field_type.can_unify(other)
	// 		}
	// 	}
	// }
}

impl Unify for Trait {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		let generics = &**other.attributes().generics;

		for method in self.methods.iter_mut() {
			method.name.unify(&other.name)?;
		}

		for bound in self.bounds.iter_mut() {
			bound.unify_with(generics)?;
		}

		Ok(())
	}
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		self.name.unify_with(other)?;

		for method in self.methods.iter_mut() {
			method.unify_with(other)?;
		}

		for bound in self.bounds.iter_mut() {
			bound.unify_with(other)?;
		}

		Ok(())
	}
}