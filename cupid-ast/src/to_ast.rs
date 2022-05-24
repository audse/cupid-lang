use crate::*;

pub trait ToAST: Sized {
	fn to_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode>;
}

macro_rules! ast {
	($exp:tt, $node:ident, $scope:ident) => { Ok(Exp::$exp($exp::to_ast($node, $scope)?)) }
}

pub fn to_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Exp, ErrCode> {
	match &*node.name {
		"typed_declaration" => ast!(Declaration, node, scope),
		"block" => ast!(Block, node, scope),
		"function_call" => ast!(FunctionCall, node, scope),
		"function" => ast!(Function, node, scope),
		"boolean"
		| "none"
		| "char"
		| "string"
		| "decimal"
		| "number" => ast!(Value, node, scope),
		_ => panic!("unrecognized {node:?}")
	}
}

impl ToAST for Value {
	fn to_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		let attributes = attributes(node, scope);
		let tokens = node.tokens.to_owned();
    	Ok(Value(untyped(match &*node.name {
			"boolean" => match &*tokens[0].source {
				"true" => Val::Boolean(true),
				"false" => Val::Boolean(false),
				_ => panic!("booleans can only be 'true' or 'false'"),
			},
			"none" => Val::None,
			"char" => {
				if tokens.len() == 2 {
					Val::Char(tokens[1].source.chars().next().unwrap_or('\0'))
				} else {
					let chars = [
						tokens[1].source.chars().next().unwrap_or_else(|| panic!("expected char")),
						tokens[2].source.chars().next().unwrap_or_else(|| panic!("expected char")),
					];
					let c = match format!("{}{}", chars[0], chars[1]).as_str() {
						r"\n" => '\n',
						r"\t" => '\t',
						r"\r" => '\r',
						r"\s" => ' ',
						c => panic!("not char: {c}")
					};
					Val::Char(c)
				}
			},
			"string" => {
				let mut string = tokens[0].source.clone();
				if let Some(first) = string.chars().next() {
					if is_quote(first) {
						string = Cow::Owned(string[1..string.len()-1].to_string());
					}
				}
				Val::String(string)
			},
			"decimal" => Val::Decimal(
				tokens[0].source.parse::<i32>().unwrap(),
				tokens[1].source.parse::<u32>().unwrap(),
			),
			"number" => Val::Integer(tokens[0].source.parse::<i32>().unwrap()),
			_ => panic!("could not parse value: {node:?}")
		}), attributes))
	}
}

macro_rules! vec_ast {
	($node:expr, $scope:expr) => {{
		let mut body = vec![];
		for x in $node.children.iter_mut() {
			body.push(to_ast(x, $scope)?);
		}
		body
	}};
}

fn untyped<T: Default>(node: T) -> Typed<T> {
	Typed::Untyped(node)
}

fn attributes(node: &mut ParseNode, scope: &mut Env) -> Attributes {
	let source = scope.add_source(node);
	Attributes::new(source)
}

impl ToAST for Block {
	fn to_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		let attributes = attributes(node, scope);
    	Ok(Self {
			body: vec_ast!(node, scope),
			attributes
		})
	}
}

impl ToAST for Declaration {
	fn to_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		let attributes = attributes(node, scope);
		let value = if node.children.len() > 2 {
			to_ast(node.children.last_mut().unwrap(), scope)?
		} else {
			Exp::Empty
		};
		Ok(Self {
			type_hint: untyped(to_type_hint(node.get("type_hint"), scope)?),
			value: untyped(Box::new(value)),
			mutable: node.has_token("mut"),
			name: Ident::to_ast(node.get("identifier"), scope)?,
			attributes,
		})
	}
}

impl ToAST for Ident {
	fn to_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		let attributes = attributes(node, scope);
    	Ok(Self {
			name: node.take_token(0).source,
			attributes
		})
	}
}

impl ToAST for FunctionCall {
	fn to_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		let attributes = attributes(node, scope);
		let mut args = vec![];
		for arg in node.get("arguments").children.iter_mut() {
			args.push(untyped(to_ast(arg, scope)?));
		}
		Ok(Self {
			function: untyped((Ident::to_ast(&mut node.children[0], scope)?, None)),
			args,
			attributes
		})
	}
}

impl ToAST for Function {
	fn to_ast(node: &mut ParseNode, scope: &mut Env) -> Result<Self, ErrCode> {
		let attributes = attributes(node, scope);
		let mut params = vec![];
		for param in node.get("parameters").children.iter_mut() {
			params.push(Declaration::to_ast(param, scope)?);
		}
		Ok(Self {
			params,
			body: untyped(Block::to_ast(&mut node.children[1], scope)?),
			attributes
		})
	}
}

fn to_type_hint(node: &mut ParseNode, scope: &mut Env) -> Result<Ident, ErrCode> {
	let mut ident = Ident::to_ast(&mut node.children[0], scope)?;	
	let mut generics: Vec<GenericParam> = vec![];
	
	for child in node.get_all("type_hint").iter_mut() {
		let argument = to_type_hint(child, scope)?;
		generics.push(GenericParam(None, Some(argument)));
	}
	ident.attributes.generics = generics;
	Ok(ident)
}

fn to_generics(node: &mut ParseNode, scope: &mut Env) -> Result<Vec<GenericParam>, ErrCode> {
	node.map_named("generic_argument", |generic: &mut ParseNode| {
		let ident = Ident::to_ast(&mut generic.children[0], scope)?;
		let arg = generic.option_map("identifier", |arg| to_type_hint(arg, scope))?;
		Ok(GenericParam(Some(ident.name), arg))
	})
}


fn is_quote(c: char) -> bool {
	c == '"' || c =='\''
}