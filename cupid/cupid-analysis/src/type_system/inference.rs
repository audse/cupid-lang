use crate::*;

pub trait InferType: UseAttributes {
	fn infer(&self, scope: &mut Env) -> ASTResult<Type>;
}

fn get_primitive(primitive: &'static str, scope: &mut Env) -> ASTResult<Type> {
	let ident = Ident::new_name(primitive);
	scope.get_type(&ident)
}

fn get_type_and_unify(ident: Ident, scope: &mut Env) -> ASTResult<Type> {
	let mut type_value = scope.get_type(&ident)?;
	type_value
		.unify_with(&*ident.attributes.generics)
		.ast_result(&ident)?;
	Ok(type_value)
}

impl InferType for Value {
	fn infer(&self, scope: &mut Env) -> ASTResult<Type> {
		match self {
			VArray(array, ..) => infer_array(array, scope),
			VBoolean(..) => get_primitive("bool", scope),
			VChar(..) => get_primitive("char", scope),
			VDecimal(..) => get_primitive("dec", scope),
			VInteger(..) => get_primitive("int", scope),
			VString(..) => get_primitive("string", scope),
			VTuple(tuple, ..) => infer_tuple(tuple, scope),
			VFunction(function) => function.infer(scope),
			VNone(..) | VBuiltinPlaceholder(..) => get_primitive("nothing", scope),
			_ => self.to_err(ERR_CANNOT_INFER)
		}
	}
}

impl InferType for Function {
	fn infer(&self, scope: &mut Env) -> ASTResult<Type> {
		let mut ident = Ident::new_name("fun");
		
		let mut fields = self.params.iter().map(|p| p.into()).collect::<Vec<Field>>();
		let return_type = self.return_type.type_of(scope)?.into_owned();
		fields.push(Field {
			name: Ident::new_name("returns"),
			type_hint: Some(return_type.into())
		});

		let generics = fields.iter().map(|f| f.type_hint.to_owned().unwrap()).collect::<Vec<Typed<Ident>>>();
		ident.attributes.generics = GenericList(generics);

		let mut fun_type = get_type_and_unify(ident, scope)?;
		fun_type.fields = FieldSet(fields);
		fun_type.base_type = BaseType::Function;
		Ok(fun_type)
	}
}

fn infer_array(array: &[Value], scope: &mut Env) -> ASTResult<Type> {
	let mut ident = Ident::build().name("array".into());

	if let Some(first_element) = array.first() {
		let first_element = first_element.infer(scope)?;
		ident = ident.one_generic(IsTyped(first_element.to_ident(), first_element));
	}

	get_type_and_unify(ident.build(), scope)
}

fn infer_tuple(tuple: &[Value], scope: &mut Env) -> ASTResult<Type> {
	let mut ident = Ident::new_name("tuple");

	let mut types = vec![];
	for item in tuple {
		types.push(IsTyped(Ident::default(), item.infer(scope)?.to_owned()));
	}

	ident.attributes.generics = GenericList(types);
	
	get_type_and_unify(ident, scope)
}