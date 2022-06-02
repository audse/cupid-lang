use crate::*;


use_utils! {
	impl CreateAST for Exp {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			match &*node.name {
				"block" => create_ast!(Block, node, scope),
				"boolean" | "none" | "char" | "string" | "decimal" | "number" => create_ast!(Value, node, scope),
				"expression" | "comment" => Exp::create_ast(node.get(0), scope),
				"function_call" =>  create_binary_op_or_ast!(FunctionCall, node, scope, Box::new(FunctionCall::create_ast(node, scope)?)),
				"function" => create_ast!(Function, node, scope),
				"identifier" => create_ast!(Ident, node, scope),
				"property" => create_binary_op_or_ast!(Property, node, scope, Box::new(Property::create_ast(node, scope)?)),
				"typed_declaration" => create_ast!(Declaration, node, scope),
				"logic_op" | "compare_op" | "add" | "multiply" | "exponent" | "type_cast" | "group" => Exp::create_ast(node.get(0), scope),
				"type_def" => create_ast!(TypeDefinition, node, scope),
				_ => panic!("unrecognized: {node:?}")
			}
		}
	}
}

use_utils! {
	impl CreateAST for Block {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			let attributes = attributes(node, scope)?;
			Ok(Self {
				body: vec_ast!(node, scope),
				attributes
			})
		}
	}
}

use_utils! {
	impl CreateAST for Declaration {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			Ok(DeclarationBuilder::new()
				.attributes(attributes(node, scope)?)
				.type_hint(Untyped(to_type_hint(node.get("type_hint"), scope)?))
				.name(Ident::create_ast(node.get("identifier"), scope)?)
				.value(node
					.get_option_map(2, |c| Exp::create_ast(c, scope))?
					.unwrap_or(Exp::Empty)
					.untyped_box())
				.mutable(node.has_token("mut"))
				.build())
		}
	}
}

use_utils! {
	impl CreateAST for Ident {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			let mut ident = Ident::build().attributes(attributes(node, scope)?);
			ident = match &*node.name {
				"identifier" => ident.name(node.token(0).source),
				"type_hint" => ident.name(node.get(0).token(0).source),
				_ => panic!()
			};
			Ok(ident.build())
		}
	}
}

use_utils! {
	impl CreateAST for FunctionCall {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			Ok(FunctionCall::build()
				.attributes(attributes(node, scope)?)
				.args(node
					.map_children_of::<&str, Typed<Exp>, ErrCode>(
						"arguments", 
						|a| Ok(Untyped(Exp::create_ast(a, scope)?))
					)?)
				.function(Untyped((Ident::create_ast(node.get(0), scope)?, None)))
				.build())
		}
	}
}

use_utils! {
	impl CreateAST for Function {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			Ok(FunctionBuilder::new()
				.attributes(attributes(node, scope)?)
				.params(node
					.map_children_of(
						"parameters", 
						|param| Declaration::create_ast(param, scope)
					)?)
				.body(Untyped(Block::create_ast(node.get(1), scope)?))
				.build())
		}
	}
}

use_utils! {
	impl CreateAST for Property {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			Ok(PropertyBuilder::new()
				.attributes(attributes(node, scope)?)
				.object(Exp::create_ast(node.get(0), scope)?.untyped_box())
				.property(Untyped(PropertyTerm::create_ast(node.get(1), scope)?))
				.build())
		}
	}
}

use_utils! {
	impl CreateAST for PropertyTerm {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			Ok(match &*node.name {
				"function_call" => PropertyTerm::FunctionCall(FunctionCall::create_ast(node, scope)?.boxed()),
				"number" => PropertyTerm::Index(
					node.tokens[0].source.parse::<usize>().unwrap(), 
					attributes(node, scope)?
				),
				"identifier" => PropertyTerm::FunctionCall(
					FunctionCall::build()
						.function(Untyped((GET.to_ident(), None)))
						.args(vec![Untyped(Exp::create_ast(node, scope)?)])
						.build()
						.boxed()
				),
				"group" => PropertyTerm::Term(Exp::create_ast(node, scope)?.boxed()),
				"property" => Self::create_ast(node.get(0), scope)?,
				_ => unreachable!()
			})
		}
	}
}

use_utils! {
	impl CreateAST for TypeDefinition {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			let ident = Ident::create_ast(node.get(0), scope)?;
			let fields_node = node.get("type_value");
			let fields = FieldSet::create_ast(fields_node, scope)?;
			let base_type = if fields_node.has("type_hint") {
				BaseType::Alias
			} else if fields_node.children.is_empty() {
				BaseType::Primitive(ident.name.to_owned())
			} else {
				BaseType::Struct
			};
			Ok(TypeDefinition::build()
				.name(ident)
				.fields(fields)
				.base_type(base_type)
				.build())
		}
	}
}

use_utils! {
	impl CreateAST for FieldSet {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			let mut get_field = |c: &mut ParseNode| -> Result<Field, ErrCode> {
				let type_hint = Untyped(Ident::create_ast(c.get(c.children.len() - 1), scope)?);
				let name = if c.children.len() > 1 {
					Some(c.get(0).token(0).source)
				} else {
					None
				};
				Ok((name, type_hint))
			};
			let mut fields = node.map_named::<Field, ErrCode>("type_field", &mut get_field)?;
			if let Some(alias_type_hint) = node.get_option_map("type_hint", &mut get_field)? {
				fields.push(alias_type_hint);
			}
			Ok(FieldSet(fields))
		}
	}
}