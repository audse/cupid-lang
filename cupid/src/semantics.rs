use crate::{ParseNode, Expression, Declare, Value, Type, Logger, Args};

pub fn to_tree(node: &ParseNode) -> Expression {
	let errors = collect_errors(node);
	if errors.len() > 0 {
		return errors[0].clone();
	}
	
	match node.name.as_str() {
		"file" => Expression::File(
			node.children
			.iter()
			.map(to_tree)
			.collect()
		),
		
		// Expression
		"expression" => to_tree(&node.children[0]),
		
		// Loops
		// "for_loop" => (),
		// "while_loop" => (),
		// "infinite_loop" => ()
		
		// Blocks
		"block" => Expression::new_block(
			node.children
				.iter()
				.map(to_tree)
				.collect()
		),
		"if_block" => {
			let else_if_bodies = node.children
				.iter()
				.filter(|n| n.name.as_str() == "else_if_block")
				.map(|n| {
					let condition = to_tree(&n.children[0]);
					let body = Expression::to_block(to_tree(&n.children[1]));
					(condition, body)
				})
				.collect();
			
			let else_body = node.children
				.iter()
				.find_map(|n| if n.name.as_str() == "else_block" {
					Some(Expression::to_block(to_tree(&n.children[0])))
				} else { None });
			
			Expression::new_if_block(
				to_tree(&node.children[0]), // condition
				Expression::to_block(to_tree(&node.children[1])), // block
				else_if_bodies,
				else_body,
			)
		},
		
		"boolean_declaration"
		| "integer_declaration"
		| "decimal_declaration"
		| "string_declaration"
		| "function_declaration" => {
			let mutable = node.tokens.len() > 0; // has `mut` keyword
			let identifier = to_tree(&node.children[0]);
			let value = if node.children.len() > 1 {
				to_tree(&node.children[1])
			} else { 
				Expression::Empty 
			};
			let r#type = match node.name.as_str() {
				"boolean_declaration" => Type::Boolean,
				"integer_declaration" => Type::Integer,
				"decimal_declaration" => Type::Decimal,
				"string_declaration" => Type::String,
				"function_declaration" => Type::Function,
				_ => Type::None
			};
			Expression::new_declare(
				identifier,
				r#type,
				mutable,
				false,
				value
			)
		},
		"declaration" => {
			let value = to_tree(&node.children[1]);
			match to_tree(&node.children[0]) {
				Expression::Declare(Declare { symbol, value: _, mutable, deep_mutable, ..}) => Expression::Declare(Declare {
					symbol,
					mutable,
					deep_mutable,
					value: Box::new(value.clone()),
					r#type: Type::None
				}),
				_ => panic!("Expected declaration")
			}
		},
		"symbol_declaration" => {
			let mutable = node.tokens[0].source.as_str() == "let";
			let deep_mutable = node.tokens.len() > 1; // includes 'mut' keyword
			Expression::new_declare(
				to_tree(&node.children[0]), 
				Type::None,
				mutable,
				deep_mutable,
				Expression::Empty,
			)
		},
		"assignment" => Expression::new_assign(
			node.tokens[0].clone(),
			to_tree(&node.children[0]),
			to_tree(&node.children[1]),
		),
		"binary_op" => Expression::new_operator(
			node.tokens[0].clone(),
			to_tree(&node.children[0]),
			to_tree(&node.children[1]),
		),
		"unary_op" => Expression::new_operator(
			node.tokens[0].clone(),
			Expression::Empty,
			to_tree(&node.children[0])
		),
		
		// Terms
		"group" => to_tree(&node.children[0]),
		"log" => Expression::Logger(
			Logger(
				node.tokens[0].clone(),
				Args(node.children[0].children.iter().map(to_tree).collect())
			)
		),
		"function" => {
			let (params, body) = if node.children.len() > 1 {
				(
					node.children[0].children
						.iter()
						.map(|p| Expression::to_symbol(to_tree(p)))
						.collect(),
					to_tree(&node.children[1])
				)
			} else {
				(vec![], to_tree(&node.children[0]))
			};
			Expression::new_function(params, body)
		},
		"function_call" => {
			let fun = to_tree(&node.children[0]);
			let args = if node.children.len() > 1 {
				node.children[1].children
					.iter()
					.map(to_tree)
					.collect()
			} else {
				vec![]
			};
			Expression::new_function_call(fun, args)
		},
		
		// Values
		"boolean" => match node.tokens[0].source.as_str() {
			"true" => Expression::new_bool_node(true, node.tokens.clone()),
			"false" => Expression::new_bool_node(false, node.tokens.clone()),
			_ => Expression::Empty,
		},
		"none" => Expression::new_none_node(node.tokens.clone()),
		"string" => Expression::new_string_node(
			node.tokens[0].source.clone(),
			node.tokens.clone(),
		),
		"decimal" => Expression::new_dec_node(
			node.tokens[0].source.parse::<i32>().unwrap_or(0),
			node.tokens[1].source.parse::<u32>().unwrap_or(0),
			node.tokens.clone(),
		),
		"number" => Expression::new_int_node(
			node.tokens[0].source.parse::<i32>().unwrap_or(0),
			node.tokens.clone(),
		),
		"identifier" => Expression::new_symbol(Expression::new_string_node(
			node.tokens[0].source.clone(), 
			node.tokens.clone()
		)),
		_ => Expression::Empty
	}
}

pub fn collect_errors(node: &ParseNode) -> Vec<Expression> {
	node.children
		.iter()
		.filter_map(|c| if c.name.as_str() == "error" {
			let message = c.tokens[0].source.clone().replace("<e ", "").replace('>', "").replace('\'', "");
			Some(
				Expression::new_node(
					Value::error(
						&c.tokens[1], 
						message,
					),
					c.tokens.clone()
				)
			)	
		} else {
			None
		})
		.collect()
}