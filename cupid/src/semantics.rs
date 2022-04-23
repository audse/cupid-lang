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
        "box_block" => Expression::new_box_block(node.children.iter().map(to_tree).collect()),
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
        },
        
        "type_definition" => {
            let identifier = &node.children[0];
            let type_name = identifier.tokens[0].source.clone();
            let symbol = TypeSymbol::new(type_name, vec![], identifier.tokens[0].clone(), false);
            
            let fields: Vec<(TypeSymbol, Option<Symbol>)> = node.children
                .iter()
                .filter_map(|n| 
                    if n.name.as_str() == "type_field" {
                        if let Expression::TypeSymbol(field_symbol) = to_tree(&n.children[0]) {
                            let field_name = Expression::to_symbol(to_tree(&n.children[1]));
                            Some((field_symbol, Some(field_name)))
                        } else {
                            None
                        }
                    } else { 
                        None 
                    })
                .collect();
            let new_type = Type::Product(ProductType {
                symbol: symbol.clone(),
                fields,
            });
            Expression::new_define_type(node.tokens[0].clone(), symbol, new_type)
        },
        
        "type_alias_definition" => {
            let identifier = &node.children[0];
            let type_name = identifier.tokens[0].source.clone();
            let symbol = TypeSymbol::new(type_name, vec![], identifier.tokens[0].clone(), false);
            
            let arguments: Vec<TypeSymbol> = node.children
                .iter()
                .filter_map(|n| {
                    if let Expression::TypeSymbol(symbol) = to_tree(n) {
                        Some(symbol)
                    } else {
                        None
                    }
                })
                .collect();
            
            Expression::new_define_type_alias(node.tokens[0].clone(), symbol, arguments)
        },
        
        "typed_declaration" => {
            let type_symbol = Expression::to_type_symbol(to_tree(&node.children[0]));
            let identifier = to_tree(&node.children[1]);
            let mutable = node.tokens.len() > 0;
            let value = if node.children.len() > 2 {
                let new_val = to_tree(&node.children[2]);
                if let Expression::Map(map) = &new_val {
                    if type_symbol.name != "map" {
                        Expression::new_map(map.items.clone(), map.token.clone(), Some(type_symbol.clone()))
                    } else {
                        new_val
                    }
                } else {
                    new_val
                }
            } else {
                Expression::Empty
            };
            Expression::new_declare(identifier, type_symbol, mutable, true, value)
        },
        
        "type_hint" => {
            let main_type = &node.children[0].tokens[0];
            let nested_types: Vec<TypeSymbol> = if node.children.len() > 1 {
                node.children[1]
                    .children
                    .iter()
                    .filter_map(|t| {
                        let symbol = to_tree(t);
                        if let Expression::TypeSymbol(s) = symbol {
                            Some(s)
                        } else if let Expression::Symbol(s) = symbol {
                            Some(TypeSymbol::new(s.identifier.to_string(), vec![], s.token, false))
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                vec![]
            };
            Expression::new_type_symbol(main_type.source.clone(), main_type.clone(), nested_types)
        },

        "assignment" => Expression::new_assign(
            node.tokens[0].clone(),
            to_tree(&node.children[0]),
            to_tree(&node.children[1]),
        ),
        // "property_assignment" => Expression::new_property_assign(
        //     to_tree(&node.children[0]),
        //     to_tree(&node.children[1]),
        //     node.tokens[0].clone(),
        // ),
        // "internal_property_assignment" => Expression::new_internal_property_assign(
        //     to_tree(&node.children[0]),
        //     to_tree(&node.children[1]),
        //     node.tokens[0].clone(),
        // ),
        "op_assignment" => {
            let symbol = to_tree(&node.children[0]);
            let operator = node.tokens[0].clone();
            let value = to_tree(&node.children[1]);
            Expression::new_assign(
                operator.clone(),
                symbol.clone(),
                Expression::new_operator(operator, symbol, value)
            )
        },
        "op_increment_assignment" => {
            let symbol = to_tree(&node.children[0]);
            let operator = node.tokens[0].clone();
            let value = Expression::new_int_node(1, vec![operator.clone()]);
            Expression::new_assign(
                operator.clone(),
                symbol.clone(),
                Expression::new_operator(operator, symbol, value)
            )
        },
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
        
        "break" => {
            let value = if node.children.len() > 0 {
                to_tree(&node.children[0])
            } else {
                Expression::Empty
            };
            Expression::new_break(
                node.tokens[0].clone(), 
                value
            )
        },
        
        "return" => {
            let value = if node.children.len() > 0 {
                to_tree(&node.children[0])
            } else {
                Expression::Empty
            };
            Expression::new_return(
                node.tokens[0].clone(), 
                value
            )
        },
        "continue" => Expression::new_continue(node.tokens[0].clone()),

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
                        .map(|p| {
                            let (param_type, identifier) = (
                                &p.children[0].children[0].tokens[0], 
                                Expression::to_symbol(to_tree(&p.children[1]))
                            );
                            let type_symbol = TypeSymbol::new(param_type.source.clone(), vec![], param_type.clone(), false);
                            (type_symbol, identifier)
                        })
                        .collect(),
                    to_tree(&node.children[1]),
                )
            } else {
                (vec![], to_tree(&node.children[0]))
            };
            Expression::new_function(params, body)
        },
        "function_call" => {
            let fun = to_tree(&node.children[0]);
            let args = if node.children.len() > 1 {
                node.children[1].children.iter().map(to_tree).collect()
            } else {
                vec![]
            };
            Expression::new_function_call(fun, args)
        },

        // Structures
        // "dictionary" => {
        //     let entries: Vec<(Expression, (usize, Expression))> = node
        //         .children
        //         .iter()
        //         .enumerate()
        //         .map(|(i, e)| (to_tree(&e.children[0]), (i, to_tree(&e.children[1]))))
        //         .collect();
        //     Expression::new_map(entries, node.tokens[0].clone(), DICTIONARY)
        // }
        // "list" => {
        //     let entries: Vec<(Expression, (usize, Expression))> = node
        //         .children
        //         .iter()
        //         .enumerate()
        //         .map(|(i, e)| (Expression::new_int_node(i as i32, vec![]), (i, to_tree(e))))
        //         .collect();
        //     Expression::new_map(entries, node.tokens[0].clone(), LIST)
        // },
        // "range" => {
        //     let range = &node.children[0];
        //     let (include_start, include_end) = match range.name.as_str() {
        //         "range_inclusive_inclusive" => (true, true),
        //         "range_inclusive_exclusive" => (true, false),
        //         "range_exclusive_inclusive" => (false, true),
        //         "range_exclusive_exclusive" | _ => (false, false),
        //     };
        //     Expression::new_range(
        //         to_tree(&range.children[0]), 
        //         to_tree(&range.children[1]),
        //         (include_start, include_end),
        //         range.tokens[0].clone()
        //     )
        // },
        "map" => {
            let entries = node.children
                .iter()
                .map(|item| (to_tree(&item.children[0]), to_tree(&item.children[1])))
                .collect();
            Expression::new_map(entries, node.tokens[0].clone(), None)
        },
        "array" => Expression::new_array(node.children.iter().map(to_tree).collect()),
        
        // "internal_property_access" => {
        //     let term = to_tree(&node.children[0]);
        //     Expression::new_internal_property_access(term, node.tokens[0].clone())
        // },
        "property_access" => {
            let map = to_tree(&node.children[0]);
            let term = to_tree(&node.children[1]);
            // Expression::new_property_access(map, term, node.tokens[0].clone())
            Expression::new_property(map, term, node.tokens[0].clone())
        },
        
        "type" => Expression::new_type_symbol(node.tokens[0].source.clone(), node.tokens[0].clone(), vec![]),

        // Values
        "boolean" => match node.tokens[0].source.as_str() {
            "true" => Expression::new_bool_node(true, node.tokens.clone()),
            "false" => Expression::new_bool_node(false, node.tokens.clone()),
            _ => Expression::Empty,
        },
        "none" => Expression::new_none_node(node.tokens.clone()),
        "char" => Expression::new_char_node(node.tokens[1].source.clone(), vec![node.tokens[0].clone()]),
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
