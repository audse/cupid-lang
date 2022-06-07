use crate::*;

// pub enum InferValue {
// 	Value(&'val Value),
// 	Val(&'val Val)
// }

// impl From<&'val Value> for InferValue {
// 	fn from(v: &'val Value) -> Self {
// 		Self::Value(v)
// 	}
// }

// impl From<&'val Val> for InferValue {
// 	fn from(v: &'val Val) -> Self {
// 		Self::Val(v)
// 	}
// }

// impl InferValue {
// 	fn val(&self) -> &Val {
// 		match self {
// 			Self::Value(v) => &*v.val,
// 			Self::Val(v) => v
// 		}
// 	}
// }

// pub trait InferType {
// 	fn infer(&self, scope: &mut Env) -> ASTResult<Type>;
// }

// impl<T: InferType + Clone + Default + PartialEq + Eq + Hash + 'static> InferType for Value {
// 	fn infer(&self, scope: &mut Env) -> ASTResult<Type> {
// 		self.value.infer(scope)
// 	}
// }

// macro_rules! impl_infer {
// 	($t:ty, $method:ident $(, $g:ident )?) => {
// 		impl$(<$g: InferType + Clone + Default + PartialEq + Eq + Hash + 'static>)? InferType for $t {
// 			fn infer(&self, scope: &mut Env) -> ASTResult<Type> {
// 				$method(self, scope)
// 			}
// 		}
// 	};
// 	($t:ty, $name:tt) => {
// 		impl InferType for $t {
// 			fn infer(&self, scope: &mut Env) -> ASTResult<Type> {
// 				get_primitive($name, scope)
// 			}
// 		}
// 	};
// }

// impl std::fmt::Display for Decimal {
// 	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
// 		write!(f, "{}.{}", self.0, self.1)
// 	}
// }

// impl std::fmt::Display for Nothing {
// 	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
// 		write!(f, "nothing")
// 	}
// }

// impl std::fmt::Display for Placeholder {
// 	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
// 		write!(f, "<placeholder>")
// 	}
// }

// // impl<T: InferType> std::fmt::Display for Vec<T> {
// // 	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
// // 		write!(f, "{}", fmt_list!(&self.0, ", "))
// // 	}
// // }

// impl_infer! { bool, "bool" }
// impl_infer! { usize, "int" }
// impl_infer! { i32, "int" }
// impl_infer! { char, "char" }
// impl_infer! { String, "string" }
// impl_infer! { Decimal, "dec" }
// impl_infer! { Nothing, infer_nothing }
// impl_infer! { Placeholder, infer_nothing }
// impl_infer! { Vec<T>, infer_tuple, T }
// impl_infer! { Function, infer_function }

// impl InferType for Type {
// 	fn infer(&self, _scope: &mut Env) -> ASTResult<Type> {
// 		Ok(type_type())
// 	}
// }

// impl InferType for Trait {
// 	fn infer(&self, _scope: &mut Env) -> ASTResult<Type> {
// 		Ok(trait_type())
// 	}
// }

// fn infer_nothing<T: InferType>(_: &T, _scope: &mut Env) -> ASTResult<Type> {
// 	Ok(nothing_type())
// }

pub fn infer_type(value: &Value, scope: &mut Env) -> ASTResult<Type> {
	match value {
		VArray(array, ..) => infer_array(array, scope),
		VBoolean(..) => get_primitive("bool", scope),
		VChar(..) => get_primitive("char", scope),
		VDecimal(..) => get_primitive("dec", scope),
		VInteger(..) => get_primitive("int", scope),
		VString(..) => get_primitive("string", scope),
		VTuple(tuple, ..) => infer_tuple(tuple, scope),
		VFunction(function) => infer_function(function, scope),
		VNone(..) | VBuiltinPlaceholder(..) => get_primitive("nothing", scope),
		_ => value.to_err(ERR_CANNOT_INFER)
	}
}

fn get_primitive(primitive: &'static str, scope: &mut Env) -> ASTResult<Type> {
	let ident = Ident::new_name(primitive);
	scope.get_type(&ident)
}

fn infer_array(array: &[Value], scope: &mut Env) -> ASTResult<Type> {
	let mut ident = Ident::build().name("array".into());

	if let Some(first_element) = array.first() {
		let first_element = infer_type(first_element, scope)?;
		ident = ident.one_generic(IsTyped(first_element.to_ident(), first_element));
	}

	get_type_and_unify(ident.build(), scope)
}

// fn infer_tuple<T: InferType + Default + Clone + PartialEq + Eq + Hash>(tuple: &[T], scope: &mut Env) -> ASTResult<Type> {
// 	let mut ident = Ident::new_name("tuple");

// 	let mut types = vec![];
// 	for item in tuple {
// 		types.push(IsTyped(Ident::default(), item.infer(scope)?));
// 	}

// 	ident.attributes.generics = GenericList(types);
	
// 	get_type_and_unify(ident, scope)
// }

fn infer_tuple(tuple: &[Value], scope: &mut Env) -> ASTResult<Type> {
	let mut ident = Ident::new_name("tuple");

	let mut types = vec![];
	for item in tuple {
		types.push(IsTyped(Ident::default(), infer_type(item, scope)?.to_owned()));
	}

	ident.attributes.generics = GenericList(types);
	
	get_type_and_unify(ident, scope)
}

// pub fn infer_function(function: &Function, scope: &mut Env) -> ASTResult<Type> {
// 	let mut ident = Ident::new_name("fun");
	
// 	let mut fields = function.params.iter().map(|p| p.into()).collect::<Vec<Field>>();
// 	let return_type = function.return_type
// 		.get_type()
// 		.map_err(|e| (Exp::Function(function.to_owned()), e))?
// 		.to_owned();
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

pub fn infer_function(function: &Function, scope: &mut Env) -> ASTResult<Type> {
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

fn get_type_and_unify(ident: Ident, scope: &mut Env) -> ASTResult<Type> {
	let mut type_value = scope.get_type(&ident)?;
	type_value.unify_with(&*ident.attributes.generics)?;
	Ok(type_value)
}