use crate::*;

macro_rules! vec_ast {
	($node:expr, $scope:expr) => {{
		let mut body = vec![];
		for x in $node.children.iter_mut() {
			body.push(create_ast(x, $scope)?);
		}
		body
	}};
}

pub(super) use vec_ast;

pub(super) fn untyped<T: Default>(node: T) -> Typed<T> {
	Typed::Untyped(node)
}

pub(super) fn attributes(node: &mut ParseNode, scope: &mut Env) -> Attributes {
	let source = scope.add_source(node);
	Attributes::new(source)
}

pub(super) fn to_type_hint(node: &mut ParseNode, scope: &mut Env) -> Result<Ident, ErrCode> {
	let mut ident = Ident::create_ast(&mut node.children[0], scope)?;	
	let mut generics: Vec<GenericParam> = vec![];
	
	for child in node.get_all("type_hint").iter_mut() {
		let argument = to_type_hint(child, scope)?;
		generics.push(GenericParam(None, Some(argument)));
	}
	ident.attributes.generics = GenericParams(generics);
	Ok(ident)
}

pub(super) fn to_generics(node: &mut ParseNode, scope: &mut Env) -> Result<Vec<GenericParam>, ErrCode> {
	node.map_named("generic_argument", |generic: &mut ParseNode| {
		let ident = Ident::create_ast(&mut generic.children[0], scope)?;
		let arg = generic.option_map("identifier", |arg| to_type_hint(arg, scope))?;
		Ok(GenericParam(Some(ident.name), arg))
	})
}