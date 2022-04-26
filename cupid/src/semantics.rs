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
        
        "builtin_type_definition" => {
            let token = &node.tokens[1];
            Expression::BuiltInType(BuiltInType {
                symbol: Symbol {
                    identifier: Value::String(token.source.clone()),
                    token: token.clone()
                }
            })
        },
        
        "alias_type_definition" => {
            let generics = get_generics(&node);
            let (type_symbol, type_hint) = if generics.len() > 0 {
                (&node.children[1], &node.children[2])
            } else {
                (&node.children[0], &node.children[1])
            };
            let type_symbol = Expression::to_symbol(to_tree(&type_symbol));
            let type_hint = to_tree(type_hint);
            Expression::new_define_type_alias(node.tokens[0].clone(), type_symbol, type_hint, generics)
        },
        
        "struct_type_definition" => {
            let generics = get_generics(&node);
            let type_symbol = if generics.len() > 0 {
                &node.children[1]
            } else {
                &node.children[0]
            };
            let type_symbol = Expression::to_symbol(to_tree(&type_symbol));
            let members: Vec<(Symbol, Expression)> = node.children
                .iter()
                .skip(1)
                .filter_map(|member| {
                    if member.name.as_str() == "struct_member" {
                        let symbol = to_tree(&member.children[1]);
                        let type_exp = to_tree(&member.children[0]);
                        if let Expression::Symbol(symbol) = symbol {
                            Some((symbol, type_exp))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();
            Expression::new_define_struct(node.tokens[0].clone(), type_symbol, members, generics)
        },
        
        "sum_type_definition" => {
            let generics = get_generics(&node);
            let type_symbol = if generics.len() > 0 {
                &node.children[1]
            } else {
                &node.children[0]
            };
            let type_symbol = Expression::to_symbol(to_tree(&type_symbol));
            let members: Vec<Expression> = node.children
                .iter()
                .skip(1)
                .filter_map(|member| {
                    if member.name.as_str() == "sum_member" {
                        Some(to_tree(&member.children[0]))
                    } else {
                        None
                    }
                })
                .collect();
            Expression::new_define_sum(node.tokens[0].clone(), type_symbol, members, generics)
        },
        
        "array_type_hint" => Expression::ArrayTypeHint(
            ArrayTypeHint {
                token: node.tokens[0].clone(),
                element_type: Box::new(to_tree(&node.children[0]))
            }
        ),
        
        "function_type_hint" => Expression::FunctionTypeHint(
            FunctionTypeHint {
                token: node.tokens[0].clone(),
                return_type: Box::new(to_tree(&node.children[0]))
            }
        ),
        
        "map_type_hint" => Expression::MapTypeHint(
            MapTypeHint {
                token: node.tokens[0].clone(),
                key_type: Box::new(to_tree(&node.children[0])),
                value_type: Box::new(to_tree(&node.children[1]))
            }
        ),
        
        "struct_type_hint" => {
            let member_args: Vec<(Symbol, Expression)> = node.children
                .iter()
                .filter_map(|child| {
                    if child.name.as_str() == "struct_member_type_hint" {
                        let symbol = Expression::to_symbol(to_tree(&child.children[0]));
                        let value = to_tree(&child.children[1]);
                        Some((symbol, value))
                    } else {
                        None
                    }
                })
                .collect();
            Expression::StructTypeHint(
                StructTypeHint {
                    token: node.tokens[0].clone(),
                    struct_type: Box::new(to_tree(&node.children[0])),
                    member_args
                }
            )
        },
        
        "primitive_type_hint" => to_tree(&node.children[0]),
        
        "implement_block" => {
            let token = node.tokens[0].clone();
            let identifier = Symbol::new_string(node.tokens[1].source.clone(), node.tokens[1].clone());
            let generics = get_generics(&node);
            // let identifier = if generics.len() > 0 {
            //     to_tree(&node.children[1])
            // } else {
            //     to_tree(&node.children[0])
            // };
            let declarations: Vec<Expression> = node.children
                .iter()
                .filter_map(|n| {
                    if n.name.as_str() == "typed_declaration" {
                        Some(to_tree(&n))
                    } else {
                        None
                    }
                })
                .collect();
            Expression::Implement(Implement::new(token, identifier, declarations))
        }

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
        
        "typed_declaration" => {
            let type_hint = to_tree(&node.children[0]);
            let identifier = to_tree(&node.children[1]);
            let mutable = node.tokens.len() > 0;
            
            let value = if node.children.len() > 2 {
                to_tree(&node.children[2])
            } else {
                Expression::Empty
            };
            Expression::new_declare(identifier, type_hint, mutable, true, value)
        },

        "assignment" => Expression::new_assign(
            node.tokens[0].clone(),
            to_tree(&node.children[0]),
            to_tree(&node.children[1]),
        ),
        "property_assignment" => {
            if let Expression::Property(property) = to_tree(&node.children[0]) {
                Expression::new_property_assign(
                    property,
                    to_tree(&node.children[1]),
                    node.tokens[0].clone(),
                )
            } else {
                panic!("expected a property")
            }
        },
        // "internal_property_assignment" => Expression::new_internal_property_assign(
        //     to_tree(&node.children[0]),
        //     to_tree(&node.children[1]),
        //     node.tokens[0].clone(),
        // ),
        "property_op_assignment" => {
            let property_exp = to_tree(&node.children[0]);
            if let Expression::Property(property) = property_exp.clone() {
                let operator = node.tokens[0].clone();
                let value = to_tree(&node.children[1]);
                Expression::new_property_assign(
                    property,
                    Expression::new_operator(operator, property_exp, value),
                    node.tokens[0].clone(),
                )
            } else {
                panic!("expected a property")
            }
        },
        "property_op_increment_assignment" => {
            let property_exp = to_tree(&node.children[0]);
            if let Expression::Property(property) = property_exp.clone() {
                let operator = node.tokens[0].clone();
                let value = Expression::new_int_node(1, vec![operator.clone()]);
                Expression::new_property_assign(
                    property,
                    Expression::new_operator(operator, property_exp, value),
                    node.tokens[0].clone(),
                )
            } else {
                panic!("expected a property")
            }
        },
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
                            if p.name.as_str() == "annotated_parameter" {
                                (
                                    to_tree(&p.children[0]),
                                    Expression::to_symbol(to_tree(&p.children[1]))
                                )
                            } else {
                                unreachable!()
                            }
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
        
        "map" => {
            let entries = node.children
                .iter()
                .map(|item| (to_tree(&item.children[0]), to_tree(&item.children[1])))
                .collect();
            Expression::new_map(entries, node.tokens[0].clone())
        },
        "array" => Expression::new_array(node.children.iter().map(to_tree).collect()),
        
        // "internal_property_access" => {
        //     let term = to_tree(&node.children[0]);
        //     Expression::new_internal_property_access(term, node.tokens[0].clone())
        // },
        "property_access" => {
            if node.children.len() > 1 {
                let map = to_tree(&node.children[0]);
                let term = to_tree(&node.children[1]);
                
                Expression::new_property(map, term, node.tokens[0].clone())
            } else {
                to_tree(&node.children[0])
            }
        },

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

pub fn get_generics(node: &ParseNode) -> Vec<Symbol> {
    node.children
    .iter()
    .find_map(|child| {
        if child.name.as_str() == "generics" {
            let generics = child.children
                .iter()
                .filter_map(|generic| {
                    if let Expression::Symbol(symbol) = to_tree(generic) {
                        Some(symbol)
                    } else {
                        None
                    }
                })
                .collect();
            Some(generics)
        } else {
            None
        }
    })
    .unwrap_or(vec![])
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
