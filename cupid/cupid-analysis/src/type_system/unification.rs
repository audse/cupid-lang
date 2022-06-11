use crate::*;

pub type UnifyResult = Result<(), UnifyErr>;

#[derive(Debug)]
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
/// generic_array_type.unify(&expected_array_type);
/// assert_eq!(generic_array_type, expected_array_type);
/// ```
#[allow(unused_variables)]
pub trait Unify {
	fn unify(&mut self, other: &Self) -> UnifyResult;
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult;
}

fn unify_option<T: Unify>(param: &mut T, arg: Option<&T>) -> UnifyResult {
	if let Some(arg) = arg {
		param.unify(arg)
	} else {
		Ok(())
	}
}

/// Unifies a list of items by doing one of the following:
/// 
/// 1. Unify each item by finding a "matching" type (with `find_fn`)
/// ```
/// use cupid_analysis::Unify;
/// 
/// // e.g. `type result (t, e) = [ ok : t, error : e ]`
/// let mut generic_result_fields = cupid_builder::fields!("ok" => "t", "error" => "e");
/// 
/// // e.g. `type add_result (e) = [ ok: int, error: e ]`
/// let concrete_result_fields = cupid_builder::fields!(
///     "ok": cupid_builder::primitive!("int").into(),
///     "error": cupid_ast::Untyped("e".into())
/// );
/// 
/// generic_result_fields.unify(&concrete_result_fields);
/// assert_eq!(generic_result_fields, concrete_result_fields);
/// ```
/// 
/// 2. Unify each item based on its position
/// ```
/// use cupid_analysis::Unify;
/// 
/// // e.g. `map (k, v)`
/// let mut generic_types = cupid_builder::generics!("key", "value");
/// 
/// // e.g. `custom_type (string, int)
/// let concrete_types = cupid_builder::generics!(
///     cupid_builder::primitive!("string").into(),
///     cupid_builder::primitive!("int").into()
/// );
/// 
/// generic_types.unify(&concrete_types);
/// assert_eq!(generic_types, concrete_types);
/// ```
fn unify_match<T: Unify>(list: &mut [T], other: &[T], mut find_fn: impl FnMut(&mut T, &T) -> bool) -> UnifyResult {
	if list.len() != other.len() {
		return Err(ERR_CANNOT_UNIFY.into());
	}
	let mut other_iter = other.iter();
	for item in list.iter_mut() {
		let other_item = match other.iter().find(|i| find_fn(item, i)) {
			Some(other_item) => Some(other_item),
			None => other_iter.next()
		};
		unify_option(item, other_item)?;
	}
	Ok(())
}

impl Unify for Ident {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		if self.name != other.name {
			return Err(self.as_err(ERR_CANNOT_UNIFY).into())
		}
		self.attributes.generics
			.unify(&other.attributes.generics)
			.map_err(|e| e.to_ast(self).into())
	}
	fn unify_with(&mut self, other: &[Typed<Ident>] ) -> UnifyResult {
		self.attributes.generics
			.unify_with(other)
			.map_err(|e| e.to_ast(self).into())
	}
}

impl Unify for Typed<Ident> {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		match (&self, other) {
			(Untyped(_), IsTyped(n, t)) => {
				_ = std::mem::replace(self, IsTyped(n.to_owned(), t.to_owned()));
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

impl Unify for GenericList {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		let mut other = other.iter();
		for ident in self.iter_mut() {
			if let Untyped(_) = ident {
				unify_option(ident, other.next())?;
			}
		}
		Ok(())
	}
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		unify_match(self, other, |self_generic, other_generic| self_generic.name == other_generic.name)
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
	fn unify(&mut self, other: &Self) -> UnifyResult {
		self.name.unify(&other.name)?;
		self.value.unify(&other.value)?;
		Ok(())
	}
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		self.name.unify_with(other)?;
		self.value.unify_with(other)?;
		Ok(())
	}
}

impl Unify for Function {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		if self.params.len() != other.params.len() {
			return Err(self.as_err(ERR_CANNOT_UNIFY).into())
		}
		// params must match in order
		let mut other_params = other.params.iter();
		for param in self.params.iter_mut() {
			unify_option(&mut param.type_hint, other_params.next().map(|p| &p.type_hint))?;
		}
		self.return_type.unify(&other.return_type)?;
		Ok(())
	}
	fn unify_with(&mut self, _other: &[Typed<Ident>]) -> UnifyResult {
		todo!()
	}
}

impl Unify for Vec<Method> {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		unify_match(self, other, |current, other| current.name == other.name)
	}
	fn unify_with(&mut self, _other: &[Typed<Ident>]) -> UnifyResult {
		todo!()
	}
}

impl Unify for Field {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		self.name.unify(&other.name)?;
		match (&mut self.type_hint, &other.type_hint) {
			(Some(self_type), Some(other_type)) => self_type.unify(other_type),
			(None, None) => Ok(()),
			_ => Err(self.as_err(ERR_CANNOT_UNIFY).into())
		}?;
		Ok(())
	}
	fn unify_with(&mut self, _other: &[Typed<Ident>]) -> UnifyResult {
		todo!()
	}
}

impl Unify for FieldSet {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		unify_match(self, other, |current, other_field| current.name == other_field.name)?;
		Ok(())
	}
	fn unify_with(&mut self, _other: &[Typed<Ident>]) -> UnifyResult {
		todo!()
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
		self.methods.unify_with(other)?;

		for bound in self.bounds.iter_mut() {
			bound.unify_with(other)?;
		}

		Ok(())
	}
}