use crate::*;

mod utils;
use utils::*;

mod value;
pub use value::*;

pub trait CreateAST: Sized {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode>;
}

macro_rules! create_ast {
	($exp:tt, $node:ident, $scope:ident) => { 
		Ok(Exp::$exp($exp::create_ast($node, $scope)?)) 
	}
}

macro_rules! create_binary_op_or_ast {
	($exp:tt, $node:ident, $scope:ident) => {
		if $node.children.len() > 1 {
			Ok(Exp::$exp($exp::create_ast($node, $scope)?))
		} else {
			create_ast(&mut $node.children[0], $scope)
		}
	}
}

pub fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Exp, ErrCode> {
	match &*node.name {
		"expression" | "comment" => create_ast(&mut node.children[0], scope),
		"typed_declaration" => create_ast!(Declaration, node, scope),
		"block" => create_ast!(Block, node, scope),
		"function" => create_ast!(Function, node, scope),
		"identifier" => create_ast!(Ident, node, scope),
		"boolean" | "none" | "char" | "string" | "decimal" | "number" => create_ast!(Value, node, scope),
		"function_call" => create_binary_op_or_ast!(FunctionCall, node, scope),
		"property" => create_binary_op_or_ast!(Property, node, scope),
		
		// "logic_op" | "compare_op" | "add" | "multiply" | "exponent" | "type_cast" | "property" | "function_call" => {
		// 	
		// },
		_ => panic!("unrecognized: {node:?}")
	}
}


impl CreateAST for Block {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		let attributes = attributes(node, scope);
    	Ok(Self {
			body: vec_ast!(node, scope),
			attributes
		})
	}
}

impl CreateAST for Declaration {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		let attributes = attributes(node, scope);
		let value = node.children
			.get_mut(2)
			.map(|c| create_ast(c, scope))
			.unwrap_or(Ok(Exp::Empty));
		Ok(Self {
			type_hint: untyped(to_type_hint(node.get("type_hint"), scope)?),
			value: untyped(Box::new(value?)),
			mutable: node.has_token("mut"),
			name: Ident::create_ast(node.get("identifier"), scope)?,
			attributes,
		})
	}
}

impl CreateAST for Ident {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		let attributes = attributes(node, scope);
    	Ok(Self {
			name: node.take_token(0).source,
			attributes
		})
	}
}

impl CreateAST for FunctionCall {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		let attributes = attributes(node, scope);
		let mut args = vec![];
		for arg in node.get_children("arguments") {
			args.push(untyped(create_ast(arg, scope)?));
		}
		Ok(Self {
			function: untyped((Ident::create_ast(&mut node.children[0], scope)?, None)),
			args,
			attributes
		})
	}
}

impl CreateAST for Function {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		let attributes = attributes(node, scope);
		let mut params = vec![];
		for param in node.get_children("parameters") {
			params.push(Declaration::create_ast(param, scope)?);
		}
		Ok(Self {
			params,
			body: untyped(Block::create_ast(&mut node.children[1], scope)?),
			attributes
		})
	}
}

impl CreateAST for Property {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
    	todo!()
	}
}