use crate::*;

#[macro_export]
macro_rules! vec_ast {
	($node:expr, $scope:expr) => {{
		let mut body = vec![];
		for x in $node.children.iter_mut() {
			body.push(create_ast(x, $scope)?);
		}
		body
	}};
}

#[macro_export]
macro_rules! create_ast {
	($exp:tt, $node:ident, $scope:ident) => { 
		Ok(Exp::$exp($exp::create_ast($node, $scope)?)) 
	}
}

#[macro_export]
macro_rules! create_binary_op_or_ast {
	($exp:tt, $node:ident, $scope:ident) => {
		if $node.children.len() > 1 {
			Ok(Exp::$exp($exp::create_ast($node, $scope)?))
		} else {
			create_ast(&mut $node.children[0], $scope)
		}
	}
}

pub(super) fn untyped_box<T: Default>(node: T) -> Typed<Box<T>> {
	Typed::Untyped(Box::new(node))
}

pub(super) fn attributes(node: &mut ParseNode, scope: &mut Env) -> Attributes {
	let source = scope.add_source(node);
	Attributes::new(source)
}

pub(super) fn to_type_hint(node: &mut ParseNode, scope: &mut Env) -> Result<Ident, ErrCode> {
	let mut ident = Ident::create_ast(node.child(0), scope)?;
	let mut generics: Vec<Generic> = vec![];
	
	for child in node.get_all("type_hint").iter_mut() {
		let argument = to_type_hint(child, scope)?;
		generics.push(Generic { ident: None, arg: Some(argument) });
	}
	ident.attributes.generics = GenericList(generics);
	Ok(ident)
}

#[allow(dead_code)]
pub(super) fn to_generics(node: &mut ParseNode, scope: &mut Env) -> Result<Vec<Generic>, ErrCode> {
	node.map_named("generic_argument", |generic| {
		let ident = Ident::create_ast(generic.child(0), scope)?;
		let arg = generic.option_map("identifier", |arg| to_type_hint(arg, scope))?;
		Ok(Generic { ident: Some(ident.name), arg })
	})
}