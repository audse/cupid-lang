use crate::*;

pub type UnifyResult = Result<(), UnifyErr>;
pub enum UnifyErr {
	ASTErr(Box<ASTErr>),
	Code(ErrCode),
}

impl From<ErrCode> for UnifyErr {
	fn from(code: ErrCode) -> Self { Self::Code(code) }
}

impl From<ASTErr> for UnifyErr {
	fn from(err: ASTErr) -> Self { Self::ASTErr(Box::new(err)) }
}

impl UnifyErr {
	pub fn to_ast(self, exp: &impl ToError) -> ASTErr {
		match self {
			Self::Code(code) => exp.as_err(code),
			Self::ASTErr(err) => *err
		}
	}
}

/// Mutates types by replacing type variables with concrete types
/// 
/// ## Example
/// ```
/// use cupid_ast::IsTyped;
/// use cupid_analysis::Unify;
/// 
/// let mut generic_array_type = cupid_builder::array!();
/// let expected_array_type = cupid_builder::array!(IsTyped(
///     "int".into(), 
///     cupid_builder::primitive!("int")
/// ));
/// 
/// _ = generic_array_type.unify(&expected_array_type);
/// assert_eq!(generic_array_type, expected_array_type);
/// ```
#[allow(unused_variables)]
pub trait Unify {
	fn unify(&mut self, other: &Self) -> UnifyResult { Ok(()) }
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult { Ok(()) }
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
			return Err(self.as_err(ERR_CANNOT_UNIFY).into())
		}
		self.attributes.generics
			.unify(&*other.attributes.generics)
			.map_err(|e| e.to_ast(self).into())
	}
	fn unify_with(&mut self, other: &[Typed<Ident>] ) -> UnifyResult {
		self.attributes.generics
			.unify(&other.to_vec())
			.map_err(|e| e.to_ast(self).into())
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
					return Err(self.as_err(ERR_CANNOT_UNIFY).into())
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
}

impl Unify for Type {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		self.name.unify(&other.name)?;
		self.fields
			.unify(&other.fields)
			.map_err(|e| e.to_ast(self))?;
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
		self.methods.unify_with(other)?;
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
}

impl Unify for Vec<Method> {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		if self.len() != other.len() {
			return Err(ERR_CANNOT_UNIFY.into());
		}
		let mut other_methods = other.iter();
		for method in self.iter_mut() {
			let other_method = match other.iter().find(|m| m.name == method.name) {
				Some(other_method) => Some(other_method),
				None => other_methods.next(),
			};
			if let Some(other_method) = other_method {
				method.unify(other_method)?;
			} else {
				return Err(method.as_err(ERR_CANNOT_UNIFY).into())
			}
		}
		Ok(())
	}
}

impl Unify for Field {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		self.name.unify(&other.name)?;
		if let (Some(self_type), Some(other_type)) = (&mut self.type_hint, &other.type_hint) {
			self_type.unify(other_type)?;
		}
		Ok(())
	}
}

/// Fieldsets can be unified in one of two ways:
/// 
/// 1. Matching fields by names
/// ```no_run
/// let self = [ key : string, value : string ]
/// let other (k, v) = [ key : k, value : v ]
/// # unifies: key => key, value => value
/// ```
/// 
/// 2. Matching fields in order
/// ```no_run
/// let self = [ key : string, value : string ]
/// let other (k, v) = [k, v]
/// # unifies: key => k, value => v
/// ```
impl Unify for FieldSet {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		if self.len() != other.len() {
			return Err(ERR_CANNOT_UNIFY.into());
		}
		let mut other_fields = other.iter();
		for field in self.iter_mut() {
			let other_field = match other.iter().find(|f| f.name == field.name) {
				Some(other_field) => Some(other_field),
				None => other_fields.next(),
			};
			if let Some(other_field) = other_field {
				field.unify(other_field)?;
			} else {
				return Err(field.as_err(ERR_CANNOT_UNIFY).into())
			}
		}
		Ok(())
	}
}

impl Unify for Trait {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		let generics = &**other.attributes().generics;
		self.methods.unify(&other.methods)?;
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