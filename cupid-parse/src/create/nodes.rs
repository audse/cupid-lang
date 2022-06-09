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
				"implement_type" | "implement_trait" => create_ast!(Implement, node, scope),
				"property" => create_binary_op_or_ast!(Property, node, scope, Box::new(Property::create_ast(node, scope)?)),
				"typed_declaration" | "declaration" => create_ast!(Declaration, node, scope),
				"logic_op" | "compare_op" | "add" | "multiply" | "exponent" | "type_cast" | "group" => Exp::create_ast(node.get(0), scope),
				"type_def" => create_ast!(TypeDef, node, scope),
				"trait_def" => create_ast!(TraitDef, node, scope),
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
				.args(node
					.get_option_map(
						"arguments", 
						|c| c.children
							.iter_mut()
							.map(|arg| Ok(Untyped(Exp::create_ast(arg, scope)?)))
							.collect::<Result<Vec<Typed<Exp>>, ErrCode>>()
					)?
					.unwrap_or_default()
				)
				.function(Untyped((Ident::create_ast(node.get(0), scope)?, None)))
				.build())
		}
	}
}

use_utils! {
	impl CreateAST for Function {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			let body = node
				.get_option("function_body")
				.map(|n| Block::create_ast(n.get(0), scope))
				.invert()?
				.unwrap_or_default();
			Ok(Function::build()
				.attributes(attributes(node, scope)?)
				.params(node
					.get("parameters")
					.map_named(
						"parameter",
						|param| Declaration::create_ast(param, scope)
					)?)
				.return_type(Untyped(Ident::create_ast(node.get("type_hint"), scope)?))
				.body(Untyped(body))
				.build())
		}
	}
}

use_utils! {
	impl CreateAST for Property {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			let object = Exp::create_ast(node.get(0), scope)?;
			let mut property_term = PropertyTerm::create_ast(node.get(1), scope)?;

			// Use object as first arg of method (uniform function call syntax)
			match &mut property_term {
				PropertyTerm::FunctionCall(function_call) => {
					function_call.args.insert(0, Untyped(object.to_owned()));
				},
				_ => ()
			}

			Ok(PropertyBuilder::new()
				.object(object.untyped_box())
				.property(Untyped(property_term))
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
						.function(Untyped((Ident::new_name("get!"), None)))
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
	impl CreateAST for TypeDef {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			let ident = Ident::create_ast(node.get(0), scope)?;
			let fields_node = node.get("type_value");
			let fields = FieldSet::create_ast(fields_node, scope)?;
			let base_type = if fields_node.has("type_hint") {
				BaseType::Alias
			} else if fields_node.children.is_empty() {
				BaseType::Primitive(ident.name.to_owned())
			} else if &*node.token(0).source == "sum" {
				BaseType::Sum
			} else {
				BaseType::Struct
			};
			Ok(TypeDef::build()
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
			let mut fields = node.map_named("type_field", &mut get_field)?;
			if let Some(alias_type_hint) = node.get_option_map("type_hint", &mut get_field)? {
				fields.push(alias_type_hint);
			}
			Ok(FieldSet(fields))
		}
	}
}

use_utils! {
	impl CreateAST for Implement {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			let mut types = node.get_all_named("type_hint");
			let for_type = Ident::create_ast(types[0], scope)?;
			let for_trait = types.get_mut(1).map_mut(|t| Ident::create_ast(t, scope)).invert()?;
			let methods = node.map_children_of("methods", |method| Method::create_ast(method, scope))?;
			Ok(Implement::build()
				.for_type(for_type)
				.for_trait(for_trait)
				.methods(methods)
				.attributes(attributes(node, scope)?)
				.build())
		}
	}
}

use_utils! {
	impl CreateAST for Method {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			Ok(Method::build()
				.name(Ident::create_ast(node.get("type_hint"), scope)?)
				.value(Function::create_ast(node.get("method_function"), scope)?)
				.build())
		}
	}
}

use_utils! {
	impl CreateAST for TraitDef {
		fn create_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
			let ident = Ident::create_ast(node.get(0), scope)?;
			let trait_value = node.get("trait_value");
			let methods: Vec<Method> = if let Some(methods) = trait_value.get_option("methods") {
				methods.map_named("method", |method| Method::create_ast(method, scope))?
			} else {
				// single method with same ident as trait
				// e.g. `trait add [t] = left t, right t => left + right`
				// desugars to `trait add [t] = [ add [t]: ... ]`
				vec![Method::build()
					.name(ident.to_owned())
					.value(Function::create_ast(trait_value.get("method_function"), scope)?)
					.build()]
			};
			Ok(TraitDef::build()
				.name(ident)
				.methods(methods)
				.build())
		}
	}
}