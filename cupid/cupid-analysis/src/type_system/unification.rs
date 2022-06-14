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

pub trait ToASTResult {
	fn ast_result(self, exp: &impl ToError) -> ASTResult<()>;
}

impl ToASTResult for UnifyResult {
	fn ast_result(self, exp: &impl ToError) -> ASTResult<()> {
		self.map_err(|e| e.to_ast(exp))
	}
}

/// Mutates types by replacing type variables with concrete types
/// 
/// # Examples
/// ```
/// use cupid_analysis::Unify;
/// 
/// let mut generic_array_type = cupid_builder::array!();
/// let expected_array_type = cupid_builder::array!(cupid_builder::primitive!("int").into());
/// 
/// generic_array_type.unify(&expected_array_type);
/// assert_eq!(generic_array_type, expected_array_type);
/// ```
#[allow(unused_variables)]
pub trait Unify {
	fn unify(&mut self, other: &Self) -> UnifyResult;

	/// Similar to `unify`, `unify_with` recursively mutates a type.
	/// Instead of trying to equate two types, this function just searches for a specified
	/// list of generics and replaces anything that matches an item in that list.
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult;

	fn partial_unify(&mut self, other: &Self) -> UnifyResult { todo!() }
	fn partial_unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult { todo!() }
}

/// Unifies an item, if a matching argument exists
/// # Examples
/// ```
/// use cupid_analysis::unify_option;
/// 
/// let mut generics = cupid_builder::generics!("a", "b", "c");
/// let args = cupid_builder::generics!(cupid_builder::primitive("int").into());
/// let expected_output = cupid_builder::generics!(
///     cupid_builder::primitive("int").into(), 
///     "b".into(), 
///     "c".into()
/// );
/// 
/// let mut args = args.iter();
/// for generic in generics.iter_mut() {
///     unify_option(generic, args.next());
/// }
/// assert_eq!(generics, expected_output);
/// ```
pub fn unify_option<T: Unify>(param: &mut T, arg: Option<&T>) -> UnifyResult {
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
pub fn unify_match<T: Unify>(list: &mut [T], other: &[T], mut find_fn: impl FnMut(&mut T, &T) -> bool) -> UnifyResult {
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
		let other = other.iter().find(|generic| generic.name == self.name);
		unify_option(self, other)
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
		self.methods
			.unify(&other.methods)
			.map_err(|e| e.to_ast(self))?;
		self.traits
			.unify(&other.traits)
			.map_err(|e| e.to_ast(self))?;
		Ok(())
	}
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		self.name.unify_with(other)?;
		self.fields
			.unify_with(other)
			.map_err(|e| e.to_ast(self))?;
		self.methods
			.unify_with(other)
			.map_err(|e| e.to_ast(self))?;
		self.traits
			.unify_with(other)
			.map_err(|e| e.to_ast(self))?;
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
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		for param in self.params.iter_mut() {
			param.type_hint.unify_with(other)?;
		}
		self.return_type.unify_with(other)?;
		Ok(())
	}
}

impl Unify for Vec<Method> {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		if self.len() != other.len() {
			return Err(ERR_CANNOT_UNIFY.into());
		}
		unify_match(self, other, |current, other| current.name == other.name)
	}
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		for method in self.iter_mut() {
			method.unify_with(other)?;
		}
		Ok(())
	}
}

impl Unify for Vec<Ident> {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		if self.len() != other.len() {
			return Err(ERR_CANNOT_UNIFY.into());
		}
		unify_match(self, other, |current, other| current.name == other.name)
	}
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		for ident in self.iter_mut() {
			ident.unify_with(other)?;
		}
		Ok(())
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
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		self.name.unify_with(other)?;
		self.type_hint.map_mut(|t| t.unify_with(other)).invert()?;
		Ok(())
	}
}

impl Unify for FieldSet {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		if self.len() != other.len() {
			return Err(ERR_CANNOT_UNIFY.into());
		}
		unify_match(self, other, |current, other_field| current.name == other_field.name)?;
		Ok(())
	}
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		for field in self.iter_mut() {
			field.unify_with(other)?;
		}
		Ok(())
	}
}

impl Unify for Trait {
	fn unify(&mut self, other: &Self) -> UnifyResult {
		self.name.unify(&other.name)?;
		self.methods
			.unify(&other.methods)
			.map_err(|e| e.to_ast(self))?;
		self.bounds
			.unify(&other.bounds)
			.map_err(|e| e.to_ast(self))?;
		Ok(())
	}
	fn unify_with(&mut self, other: &[Typed<Ident>]) -> UnifyResult {
		self.name.unify_with(other)?;
		self.methods
			.unify_with(other)
			.map_err(|e| e.to_ast(self))?;
		self.bounds
			.unify_with(other)
			.map_err(|e| e.to_ast(self))?;
		Ok(())
	}
}