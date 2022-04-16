use crate::ParseNode;
use crate::tree::*;

pub fn to_tree(node: &ParseNode) -> Expression {
    let errors = collect_errors(node);
    if !errors.is_empty() {
        return errors[0].clone();
    }

    match node.name.as_str() {
        "file" => Expression::File(node.children.iter().map(to_tree).collect()),

        // Expression
        "expression" => to_tree(&node.children[0]),

        // Loops
        "for_loop" => {
            let (params, map, body) = {
                (
                    node.children[0]
                        .children
                        .iter()
                        .map(|p| Expression::to_symbol(to_tree(p)))
                        .collect(),
                    to_tree(&node.children[1]),
                    to_tree(&node.children[2]),
                )
            };
            Expression::new_for_in_loop(params, map, body, node.tokens[0].clone())
        }
        "while_loop" => Expression::new_while_loop(
            to_tree(&node.children[0]),
            to_tree(&node.children[1]),
            node.tokens[0].clone(),
        ),
        // "infinite_loop" => ()

        // Blocks
        "block" => Expression::new_block(node.children.iter().map(to_tree).collect()),
        "if_block" => {
            let else_if_bodies = node
                .children
                .iter()
                .filter(|n| n.name.as_str() == "else_if_block")
                .map(|n| {
                    let condition = to_tree(&n.children[0]);
                    let body = Expression::to_block(to_tree(&n.children[1]));
                    (condition, body)
                })
                .collect();

            let else_body = node.children.iter().find_map(|n| {
                if n.name.as_str() == "else_block" {
                    Some(Expression::to_block(to_tree(&n.children[0])))
                } else {
                    None
                }
            });

            Expression::new_if_block(
                to_tree(&node.children[0]),                       // condition
                Expression::to_block(to_tree(&node.children[1])), // block
                else_if_bodies,
                else_body,
            )
        }
		
		"type_definition" => {
            let identifier = &node.children[0];
            let type_name = identifier.tokens[0].source.clone();
            let symbol = TypeSymbol::new(type_name, identifier.tokens[0].clone());
            
			let fields: Vec<(Type, Symbol)> = node.children
				.iter()
				.filter_map(|n| 
					if n.name.as_str() == "type_field" {
                        let field_symbol = &n.children[0];
						let field_type = Type {
							symbol: TypeSymbol::new(
                                field_symbol.tokens[0].source.clone(), 
                                field_symbol.tokens[0].clone()
                            ),
							fields: vec![]
						};
						let field_name = Expression::to_symbol(to_tree(&n.children[1]));
						Some((field_type, field_name))
					} else { 
						None 
					})
				.collect();
			let new_type = Type {
				symbol,
				fields,
			};
			Expression::new_define_type(node.tokens[0].clone(), new_type)
		},
        
        "typed_declaration" => {
            let var_type = &node.tokens[0];
            let mutable = node.tokens.len() > 1;
            let identifier = to_tree(&node.children[0]);
            let value = if node.children.len() > 1 {
                to_tree(&node.children[1])
            } else {
                Expression::Empty
            };
            let type_symbol = TypeSymbol::new(var_type.source.clone(), var_type.clone());
            Expression::new_declare(identifier, type_symbol, mutable, true, value)
        },

        "declaration" => {
            let value = to_tree(&node.children[1]);
            match to_tree(&node.children[0]) {
                Expression::Declare(Declare {
                    symbol,
                    value: _,
                    mutable,
                    deep_mutable,
                    ..
                }) => Expression::Declare(Declare {
                    symbol,
                    mutable,
                    deep_mutable,
                    value: Box::new(value),
                    r#type: NONE.symbol,
                }),
                _ => panic!("Expected declaration"),
            }
        }
        "symbol_declaration" => {
            let mutable = node.tokens[0].source.as_str() == "let";
            let deep_mutable = node.tokens.len() > 1; // includes 'mut' keyword
            Expression::new_declare(
                to_tree(&node.children[0]),
                NONE.symbol,
                mutable,
                deep_mutable,
                Expression::Empty,
            )
        }
        "assignment" => Expression::new_assign(
            node.tokens[0].clone(),
            to_tree(&node.children[0]),
            to_tree(&node.children[1]),
        ),
        "property_assignment" => Expression::new_property_assign(
            to_tree(&node.children[0]),
            to_tree(&node.children[1]),
            node.tokens[0].clone(),
        ),
        "internal_property_assignment" => Expression::new_internal_property_assign(
            to_tree(&node.children[0]),
            to_tree(&node.children[1]),
            node.tokens[0].clone(),
        ),
        "binary_op" | "compare_op" | "add" | "multiply" | "exponent" => {
            if !node.tokens.is_empty() && node.children.len() > 1 {
                Expression::new_operator(
                    node.tokens[0].clone(),
                    to_tree(&node.children[0]),
                    to_tree(&node.children[1]),
                )
            } else {
                to_tree(&node.children[0])
            }
        }
        "unary_op" => Expression::new_operator(
            node.tokens[0].clone(),
            Expression::Empty,
            to_tree(&node.children[0]),
        ),

        // Terms
        "group" => to_tree(&node.children[0]),
        "log" => Expression::Logger(Logger(
            node.tokens[0].clone(),
            Args(node.children[0].children.iter().map(to_tree).collect()),
        )),
        "function" => {
            let (params, body) = if node.children.len() > 1 {
                (
                    node.children[0]
                        .children
                        .iter()
                        .map(|p| Expression::to_symbol(to_tree(p)))
                        .collect(),
                    to_tree(&node.children[1]),
                )
            } else {
                (vec![], to_tree(&node.children[0]))
            };
            Expression::new_function(params, body)
        }
        "function_call" => {
            let fun = to_tree(&node.children[0]);
            let args = if node.children.len() > 1 {
                node.children[1].children.iter().map(to_tree).collect()
            } else {
                vec![]
            };
            Expression::new_function_call(fun, args)
        }

        // Structures
        "dictionary" => {
            let entries: Vec<(Expression, (usize, Expression))> = node
                .children
                .iter()
                .enumerate()
                .map(|(i, e)| (to_tree(&e.children[0]), (i, to_tree(&e.children[1]))))
                .collect();
            Expression::new_map(entries, node.tokens[0].clone(), DICTIONARY)
        }
        "list" => {
            let entries: Vec<(Expression, (usize, Expression))> = node
                .children
                .iter()
                .enumerate()
                .map(|(i, e)| (Expression::new_int_node(i as i32, vec![]), (i, to_tree(e))))
                .collect();
            Expression::new_map(entries, node.tokens[0].clone(), LIST)
        },
        "internal_property_access" => {
            let term = to_tree(&node.children[0]);
            Expression::new_internal_property_access(term, node.tokens[0].clone())
        },
        "property_access" => {
            let map = to_tree(&node.children[0]);
            let term = to_tree(&node.children[1]);
            Expression::new_property_access(map, term, node.tokens[0].clone())
        }

        // Values
        "boolean" => match node.tokens[0].source.as_str() {
            "true" => Expression::new_bool_node(true, node.tokens.clone()),
            "false" => Expression::new_bool_node(false, node.tokens.clone()),
            _ => Expression::Empty,
        },
        "none" => Expression::new_none_node(node.tokens.clone()),
        "string" => Expression::new_string_node(node.tokens[0].source.clone(), node.tokens.clone()),
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
            node.tokens.clone(),
        )),
        _ => Expression::Empty,
    }
}

pub fn collect_errors(node: &ParseNode) -> Vec<Expression> {
    node.children
        .iter()
        .filter_map(|c| {
            if c.name.as_str() == "error" {
                let message = c.tokens[0]
                    .source
                    .clone()
                    .replace("<e ", "")
                    .replace('>', "")
                    .replace('\'', "");
                Some(Expression::new_node(
                    Value::error(&c.tokens[1], message, String::new()),
                    c.tokens.clone(),
                ))
            } else {
                None
            }
        })
        .collect()
}
