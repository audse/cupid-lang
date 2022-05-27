use crate::*;

mod utils;
use utils::*;

mod value;
pub use value::*;

pub trait CreateAST: Sized {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode>;
}

pub fn create_file_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Vec<Exp>, ErrCode> {
	node.children
		.iter_mut()
		.map(|c| create_ast(c, scope))
		.collect::<Result<Vec<Exp>, ErrCode>>()
}

pub fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Exp, ErrCode> {
	match &*node.name {
		"block" => create_ast!(Block, node, scope),
		"boolean" | "none" | "char" | "string" | "decimal" | "number" => create_ast!(Value, node, scope),
		"expression" | "comment" => create_ast(node.child(0), scope),
		"function_call" => create_binary_op_or_ast!(FunctionCall, node, scope),
		"function" => create_ast!(Function, node, scope),
		"identifier" => create_ast!(Ident, node, scope),
		"property" => if node.children.len() > 1 {
			Ok(Exp::Property(Box::new(Property::create_ast(node, scope)?)))
		} else {
			create_ast(node.child(0), scope)
		},
		"typed_declaration" => create_ast!(Declaration, node, scope),
		"logic_op" | "compare_op" | "add" | "multiply" | "exponent" | "type_cast" | "group" => create_ast(node.child(0), scope),
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
		Ok(DeclarationBuilder::new()
			.attributes(attributes(node, scope))
			.type_hint(Untyped(to_type_hint(node.get("type_hint"), scope)?))
			.name(Ident::create_ast(node.get("identifier"), scope)?)
			.value(untyped_box(node.some_child(2).map(|c| create_ast(c, scope)).unwrap_or(Ok(Exp::Empty))?))
			.mutable(node.has_token("mut"))
			.build())
	}
}

impl CreateAST for Ident {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		Ok(IdentBuilder::new()
			.attributes(attributes(node, scope))
			.name(node.take_token(0).source)
			.build())
	}
}

impl CreateAST for FunctionCall {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		type ArgList = Result<Vec<Typed<Exp>>, ErrCode>;
		Ok(FunctionCallBuilder::new()
			.attributes(attributes(node, scope))
			.args(node.get_children("arguments")
					.iter_mut()
					.map(|a| Ok(Untyped(create_ast(a, scope)?)))
					.collect::<ArgList>()?)
			.function(Untyped((Ident::create_ast(&mut node.children[0], scope)?, None)))
			.build())
	}
}

impl CreateAST for Function {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		type ParamList = Result<Vec<Declaration>, ErrCode>;
		Ok(FunctionBuilder::new()
			.attributes(attributes(node, scope))
			.params(node.get_children("parameters")
					.into_iter()
					.map(|param| Declaration::create_ast(param, scope))
					.collect::<ParamList>()?)
			.body(Untyped(Block::create_ast(&mut node.children[1], scope)?))
			.build())
	}
}

impl CreateAST for Property {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		Ok(PropertyBuilder::new()
			.attributes(attributes(node, scope))
			.object(untyped_box(create_ast(node.child(0), scope)?))
			.property(Untyped(PropertyTerm::create_ast(node.child(1), scope)?))
			.build())
	}
}

impl CreateAST for PropertyTerm {
	fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		Ok(match &*node.name {
			"function_call" => PropertyTerm::FunctionCall(Box::new(FunctionCall::create_ast(node, scope)?)),
			"number" => PropertyTerm::Index(
				node.tokens[0].source.parse::<usize>().unwrap(), 
				attributes(node, scope)
			),
			"identifier" => PropertyTerm::FunctionCall(
				Box::new(FunctionCall::build()
					.function(Untyped((GET.to_ident(), None)))
					.args(vec![Untyped(create_ast(node, scope)?)])
					.build())
			),
			"group" => PropertyTerm::Term(Box::new(create_ast(node, scope)?)),
			"property" => Self::create_ast(node.child(0), scope)?,
			_ => unreachable!()
		})
	}
}
