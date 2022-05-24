use crate::*;

pub fn parse_import(node: &mut ParseNode) -> ImportNode {
	match &*node.name {
		"packages" => ImportNode::PackageList(node.filter_map_noresult(&|n| {
			if let ImportNode::Package(package) = parse_import(n) {
				Some(package)
			} else {
				None
			}
		})),
		"package" => {
			match parse_import(&mut node.children[0]) {
				ImportNode::NameSpace(name_space) => ImportNode::Package(Package {
					name_space: Some(name_space),
					items: if let ImportNode::Items(items) = parse_import(&mut node.children[1]) {
						items
					} else {
						panic!()
					}
				}),
				ImportNode::Items(items) => ImportNode::Package(Package {
					name_space: None,
					items
				}),
				_ => panic!()
			}
		},
		"name_space" => {
			let mut name_node = NameSpace {
				name: node.children[0].tokens[0].source.to_owned(),
				tokens: node.children[0].tokens.to_owned(),
			};
			if node.children.len() == 1 {
				ImportNode::NameSpace(name_node)
			} else if let ImportNode::NameSpace(mut name_space) = parse_import(&mut node.children[1]) {
				name_node.name = (name_node.name.to_string() + "/" + &*name_space.name).into();
				name_node.tokens.append(&mut name_space.tokens);
				ImportNode::NameSpace(name_node)
			} else {
				ImportNode::NameSpace(name_node)
			}
		},
		"item_group" => ImportNode::Items(node.filter_map_noresult(&|child| {
			if let ImportNode::Items(items) = parse_import(child) {
				Some(items[0].to_owned())
			} else {
				None
			}
		})),
		"item" => ImportNode::Items(vec![ImportItem {
			identifier: node.tokens[0].source.to_owned(),
			tokens: node.tokens.to_owned()
		}]),
		_ => todo!()
	}
}