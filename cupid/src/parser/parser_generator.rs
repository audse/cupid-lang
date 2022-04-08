use crate::{
	Rule,
	Alt,
	AltGroup
};

pub fn generate_parser(rules: Vec<Rule>) -> String {
	println!("{:?}", rules);
	let body: Vec<String> = rules
		.iter()
		.map(generate_rule)
		.collect();
	return body.join("");
}


fn generate_rule(rule: &Rule) -> String {
	let body: Vec<String> = rule.alts
		.iter()
		.map(|alt| generate_rule_body(rule, alt))
		.collect();
	
	return format!(
		"
		pub fn _{name}(&mut self, _arg: Option<String>) -> Option<(Node, bool)> {{
			if let Some(memo) = self.memoize(\"{name}\".to_string(), None) {{
				return Some(memo);
			}}
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {{
				name: \"{name}\".to_string(),
				tokens: vec![],
				children: vec![],
			}};
			{body}
			self.make_memo(start_pos, \"{name}\".to_string(), None);
			return None;
		}}
		",
		name = rule.name,
		body = body.join("\n"),
	);
}

fn generate_rule_body(rule: &Rule, items: &Vec<AltGroup>) -> String {
	let mut body = String::new();
	let mut loop_string = String::from("loop { \n");
	for alt in items {
		let group_prefix = alt.prefix_modifier.as_deref().unwrap_or("");
		let group_suffix = alt.suffix_modifier.as_deref().unwrap_or("");
		let group_concealed = group_prefix == "~";
		let item_list: Vec<(String, &str, &str, bool)> = alt.alts
			.iter()
			.map(|item| item_details(item, group_concealed))
			.collect();
		if group_suffix != "+" {
			for item in &item_list {
				loop_string += item_body(&item).as_str();
			}
		}
		if group_suffix == "*" || group_suffix == "+" {
			loop_string += "loop { \n";
			for item in &item_list {
				loop_string += item_body(&item).as_str();
			}
			loop_string += "}";
		}
	}
	
	loop_string.push_str(format!(
		"
			let result = Some((node, {:?}));
			return self.make_memo(start_pos, \"{}\".to_string(), result);
		\n}}
		",
		rule.pass_through,
		rule.name
	).as_str());
	
	body.push_str(loop_string.as_str());
	body += "self.reset_parse(&mut node, pos);";
	return body;
}

fn item_details(item: &Alt, group_concealed: bool) -> (String, &str, &str, bool) {
	let prefix = item.prefix_modifier.as_deref().unwrap_or("");
	let suffix = item.suffix_modifier.as_deref().unwrap_or("");
	let concealed = group_concealed || prefix == "~";
	let method = get_method(item);
	return (method, prefix, suffix, concealed);
}

fn item_body(item:&(String, &str, &str, bool)) -> String {
	match item.2 {
		"?" => format!("use_optional!(&mut node, {}, {});\n", item.0, item.3),
		"*" => format!("use_repeat!(&mut node, {}, {});\n", item.0, item.3),
		"+" => format!("use_item!(&mut node, {}, {});\n", item.0, item.3)
		 	+ format!("use_repeat!(&mut node, {}, {});\n", item.0, item.3).as_str(),
		_ => format!("use_item!(&mut node, {}, {});\n", item.0, item.3),
	}
}

fn get_method(i: &Alt) -> String {
	match i.kind.as_str() {
		// e.g. NUMBER becomes self.expect_number()
		"constant" => format!("self.expect_{}(None)", i.source.to_lowercase()),
		// e.g. keyword becomes self._keyword()
		"name" => format!("self._{}(None)", i.source),
		// e.g. 'abc' becomes self.expect("abc")
		"string" | _ => format!("self.expect({}.to_string())", i.source.replace("'", "\"")) //"),
	}
}
