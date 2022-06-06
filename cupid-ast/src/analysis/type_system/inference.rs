use crate::*;

// pub enum InferValue<'val> {
// 	Value(&'val Value),
// 	Val(&'val Val)
// }

// impl<'val> From<&'val Value> for InferValue<'val> {
// 	fn from(v: &'val Value) -> Self {
// 		Self::Value(v)
// 	}
// }

// impl<'val> From<&'val Val> for InferValue<'val> {
// 	fn from(v: &'val Val) -> Self {
// 		Self::Val(v)
// 	}
// }

// impl InferValue<'_> {
// 	fn val(&self) -> &Val {
// 		match self {
// 			Self::Value(v) => &*v.val,
// 			Self::Val(v) => v
// 		}
// 	}
// }

pub trait AsValue: Default {
	fn as_type(&self) -> Option<&Type> { None }
	fn as_type_mut(&mut self) -> Option<&mut Type> { None }
	fn as_trait(&self) -> Option<&Trait> { None }
	fn as_trait_mut(&mut self) -> Option<&mut Trait> { None }
	fn as_function(&self) -> Option<&Function> { None }
	fn as_function_mut(&mut self) -> Option<&mut Function> { None }
}

impl<T: AsValue> AsValue for Value<T> {
	fn as_type(&self) -> Option<&Type> { self.value.as_type() }
	fn as_type_mut(&mut self) -> Option<&mut Type> { self.value.as_type_mut() }
	fn as_trait(&self) -> Option<&Trait> { self.value.as_trait() }
	fn as_trait_mut(&mut self) -> Option<&mut Trait> { self.value.as_trait_mut() }
	fn as_function(&self) -> Option<&Function> { self.value.as_function() }
	fn as_function_mut(&mut self) -> Option<&mut Function> { self.value.as_function_mut() }
}

impl AsValue for i32 {}
impl AsValue for String {}
impl AsValue for bool {}
impl AsValue for char {}
impl AsValue for Decimal {}
impl AsValue for Nothing {}
impl AsValue for Placeholder {}
impl<T: AsValue> AsValue for Vec<T> {}

impl AsValue for Type<'_> {
	fn as_type(&self) -> Option<&Type> { Some(&self) }
	fn as_type_mut(&mut self) -> Option<&mut Type> { Some(&mut self) }
}

impl AsValue for Trait<'_> {
	fn as_trait(&self) -> Option<&Trait> { Some(&self) }
	fn as_trait_mut(&mut self) -> Option<&mut Trait> { Some(&mut self) }
}

impl AsValue for Function<'_> {
	fn as_function(&self) -> Option<&Function> { Some(&self.value) }
	fn as_function_mut(&mut self) -> Option<&mut Function> { Some(&mut self) }
}

pub trait InferType: AsValue {
	fn infer(&self, scope: &mut Env) -> ASTResult<Type>;
}

impl<T: InferType> InferType for Value<T> {
	fn infer(&self, scope: &mut Env) -> ASTResult<Type> {
		self.value.infer(scope)
	}
}

macro_rules! impl_infer {
	($t:ty, $method:ident $(, $g:ident )?) => {
		impl$(<$g: InferType> )? InferType for $t {
			fn infer(&self, scope: &mut Env) -> ASTResult<Type> {
				$method(self, scope)
			}
		}
	};
	($t:ty, $name:tt) => {
		impl InferType for $t {
			fn infer(&self, scope: &mut Env) -> ASTResult<Type> {
				get_primitive($name, scope)
			}
		}
	};
}

impl std::fmt::Display for Decimal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}.{}", self.0, self.1)
	}
}

impl std::fmt::Display for Nothing {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "nothing")
	}
}

impl std::fmt::Display for Placeholder {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "<placeholder>")
	}
}

// impl<T: InferType> std::fmt::Display for Vec<T> {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		write!(f, "{}", fmt_list!(&self.0, ", "))
// 	}
// }

impl_infer! { bool, "bool" }
impl_infer! { i32, "int" }
impl_infer! { char, "char" }
impl_infer! { String, "string" }
impl_infer! { Decimal, "dec" }
impl_infer! { Nothing, infer_nothing }
impl_infer! { Placeholder, infer_nothing }
impl_infer! { Vec<T>, infer_tuple, T }
impl_infer! { Function<'_>, infer_function }

impl InferType for Type<'_> {
	fn infer(&self, scope: &mut Env) -> ASTResult<Type> {
		Ok(TYPE.to_owned())
	}
}

impl InferType for Trait<'_> {
	fn infer(&self, scope: &mut Env) -> ASTResult<Type> {
		Ok(TRAIT.to_owned())
	}
}

fn infer_nothing<'ast, T: InferType>(_: &T, scope: &mut Env) -> ASTResult<Type<'ast>> {
	Ok(NOTHING.to_owned())
}

// pub fn infer_type<'val, V>(value: &'val V, scope: &mut Env) -> ASTResult<Type> where InferValue<'val>: From<&'val V>, V: ErrorContext {
// 	use Val::*;
// 	let val: InferValue = value.into();
// 	match &val.val() {
// 		Array(array) => infer_array(array, scope),
// 		Boolean(_) => get_primitive("bool", scope),
// 		Char(_) => get_primitive("char", scope),
// 		Decimal(..) => get_primitive("dec", scope),
// 		Integer(_) => get_primitive("int", scope),
// 		String(_) => get_primitive("string", scope),
// 		Tuple(tuple) => infer_tuple(tuple, scope),
// 		Function(function) => infer_function(function, scope),
// 		None | BuiltinPlaceholder => get_primitive("nothing", scope),
// 		_ => value.to_err(ERR_CANNOT_INFER)
// 	}
// }

fn get_primitive<'ast>(primitive: &'static str, scope: &'ast mut Env) -> ASTResult<Type<'ast>> {
	let ident = Ident::new_name(primitive);
	scope.get_type(&ident)
}

// fn infer_array(array: &[Val], scope: &mut Env) -> ASTResult<Type> {
// 	let mut ident = Ident::build().name("array".into());

// 	if let Some(first_element) = array.first() {
// 		let first_element = infer_type(first_element, scope)?;
// 		ident = ident.one_generic(IsTyped(first_element.to_ident(), first_element));
// 	}

// 	get_type_and_unify(ident.build(), scope)
// }

fn infer_tuple<'ast, T: InferType>(tuple: &'ast Tuple<T>, scope: &'ast mut Env) -> ASTResult<Type<'ast>> {
	let mut ident = Ident::new_name("tuple");

	let mut types = vec![];
	for item in &tuple.0 {
		types.push(IsTyped(Ident::default(), item.infer(scope)?));
	}

	ident.attributes.generics = GenericList(types);
	
	get_type_and_unify(ident, scope)
}

// fn infer_tuple(tuple: &[Val], scope: &mut Env) -> ASTResult<Type> {
// 	let mut ident = Ident::new_name("tuple");

// 	let mut types = vec![];
// 	for item in tuple {
// 		types.push(IsTyped(Ident::default(), infer_type(item, scope)?.to_owned()));
// 	}

// 	ident.attributes.generics = GenericList(types);
	
// 	get_type_and_unify(ident, scope)
// }

pub fn infer_function<'ast>(function: &'ast Function, scope: &mut Env) -> ASTResult<Type<'ast>> {
	let mut ident = Ident::new_name("fun");
	
	let mut fields = function.params.iter().map(|p| p.into()).collect::<Vec<Field>>();
	let return_type = function.return_type.type_of(scope)?.into_owned();
	fields.push((
		Some("returns".into()), 
		return_type.into()
	));

	let generics = fields.iter().map(|(_, f)| f.to_owned()).collect::<Vec<Typed<Ident>>>();
	ident.attributes.generics = GenericList(generics);

	let mut fun_type = get_type_and_unify(ident, scope)?;
	fun_type.fields = FieldSet(fields);
	fun_type.base_type = BaseType::Function;
	Ok(fun_type)
}

// pub fn infer_function(function: &Function, scope: &mut Env) -> ASTResult<Type> {
// 	let mut ident = Ident::new_name("fun");
	
// 	let mut fields = function.params.iter().map(|p| p.into()).collect::<Vec<Field>>();
// 	let return_type = function.return_type.type_of(scope)?.into_owned();
// 	fields.push((
// 		Some("returns".into()), 
// 		return_type.into()
// 	));

// 	let generics = fields.iter().map(|(_, f)| f.to_owned()).collect::<Vec<Typed<Ident>>>();
// 	ident.attributes.generics = GenericList(generics);

// 	let mut fun_type = get_type_and_unify(ident, scope)?;
// 	fun_type.fields = FieldSet(fields);
// 	fun_type.base_type = BaseType::Function;
// 	Ok(fun_type)
// }

fn get_type_and_unify<'ast>(ident: Ident, scope: &'ast mut Env) -> ASTResult<Type<'ast>> {
	let mut type_value = scope.get_type(&ident)?;
	type_value.unify_with(&*ident.attributes.generics)?;
	Ok(type_value)
}