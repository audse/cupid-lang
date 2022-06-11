use crate::create::CreateAST;
use crate::*;

#[macro_export]
macro_rules! vec_ast {
	($node:expr, $scope:expr) => {{
		let mut body = vec![];
		for x in $node.children.iter_mut() {
			body.push(Exp::create_ast(x, $scope)?);
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
	($exp:tt, $node:ident, $scope:ident, $func:expr) => {
		if $node.children.len() > 1 {
			Ok(Exp::$exp($func))
		} else {
			Exp::create_ast(&mut $node.children[0], $scope)
		}
	}
}

#[macro_export]
macro_rules! use_utils {
	(impl CreateAST for $node_name:ty { $($implement_trait:item)* }) => {
		impl CreateAST for $node_name {
			$($implement_trait)*
		}
		impl CreateASTUtils for $node_name {}
	}
}

pub trait CreateASTUtils: CreateAST + Default {
	fn boxed(self) -> Box<Self> {
		Box::new(self)
	}
	fn untyped_box(self) -> Box<Typed<Self>> {
		Box::new(Untyped(self))
	}
	fn untyped(self) -> Typed<Self> {
		Untyped(self)
	}
}

pub fn attributes(node: &mut node::ParseNode, scope: &mut Env) -> Result<Attributes, ErrCode> {
	let source = scope.add_source(node.to_owned());
	let generics: Result<Vec<Typed<Ident>>, ErrCode> = node.map_named(
		"type_hint", 
		|t| Ok(Untyped(Ident::create_ast(t, scope)?))
	);
	Ok(Attributes::build()
		.source(Some(source))
		.generics(GenericList(generics?))
		.build())
}