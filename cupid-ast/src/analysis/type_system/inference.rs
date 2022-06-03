use crate::*;

pub enum InferValue<'v> {
	Value(&'v Value),
	Val(&'v Val)
}

impl<'v> From<&'v Value> for InferValue<'v> {
	fn from(v: &'v Value) -> Self {
		Self::Value(v)
	}
}

impl<'v> From<&'v Val> for InferValue<'v> {
	fn from(v: &'v Val) -> Self {
		Self::Val(v)
	}
}

impl InferValue<'_> {
	fn val(&self) -> &Val {
		match self {
			Self::Value(v) => &*v.val,
			Self::Val(v) => v
		}
	}
	fn source(&self) -> usize {
		match self {
			InferValue::Val(_) => 0,
			InferValue::Value(v) => v.source()
		}
	}
}

pub fn infer_type<'v, V>(value: &'v V, scope: &mut Env) -> ASTResult<Type> where InferValue<'v>: From<&'v V> {
	use Val::*;
	let val: InferValue = value.into();
	match &val.val() {
		Array(array) => infer_array(array, scope),
		Boolean(_) => get_primitive("bool", scope),
		Char(_) => get_primitive("char", scope),
		Decimal(..) => get_primitive("dec", scope),
		Integer(_) => get_primitive("int", scope),
		String(_) => get_primitive("string", scope),
		Tuple(tuple) => infer_tuple(tuple, scope),
		Function(function) => infer_function(function, scope),
		None | BuiltinPlaceholder => get_primitive("nothing", scope),
		_ => Err((val.source(), ERR_CANNOT_INFER))
	}
}

fn get_primitive(primitive: &'static str, scope: &mut Env) -> ASTResult<Type> {
	let ident = Ident::new_name(primitive);
	scope.get_type(&ident)
}

fn infer_array(array: &[Val], scope: &mut Env) -> ASTResult<Type> {
	let mut ident = Ident::build().name("array".into());

	if let Some(first_element) = array.first() {
		let first_element = infer_type(first_element, scope)?;
		ident = ident.one_generic(IsTyped(first_element.to_ident(), first_element));
	}

	get_type_and_unify(ident.build(), scope)
}

fn infer_tuple(tuple: &[Val], scope: &mut Env) -> ASTResult<Type> {
	let mut ident = Ident::new_name("tuple");

	let types: ASTResult<Vec<Typed<Ident>>> = tuple
		.iter()
		.map(|t| Ok(IsTyped(Ident::default(), infer_type(t, scope)?)))
		.collect();
	let types = types?;

	ident.attributes.generics = GenericList(types);
	
	get_type_and_unify(ident, scope)
}

pub fn infer_function(function: &Function, scope: &mut Env) -> ASTResult<Type> {
	let mut ident = Ident::new_name("fun");
	
	let mut fields = function.params.iter().map(|p| p.into()).collect::<Vec<Field>>();
	let return_type = function.return_type.type_of(scope)?;
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