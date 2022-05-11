use crate::{
	Rule,
	Alt,
	AltGroup,
	Cow
};

pub fn generate_parser(rules: Vec<Rule>) -> Cow<'static, str> {
	println!("{:?}", rules);
	let body: Vec<Cow<'static, str>> = rules
		.iter()
		.map(|rule| generate_rule(&rule))
		.collect();
	body.join("").into()
}


fn generate_rule(rule: &Rule) -> Cow<'static, str> {
	// let mut body: String = String::new();
	// for alt in rule.alts {
	// 	body += &generate_rule_body(rule, alt);
	// }
	let body: Vec<Cow<'static, str>> = rule.alts
		.iter()
		.map(|alt| generate_rule_body(&rule, alt))
		.collect();
	format!(
		"
		pub fn _{name}(&mut self, _arg: Option<Token>) -> Option<(Node, bool)> {{
			let start_pos = self.tokens.index();
			let pos = start_pos;
			let mut node = Node {{
				name: \"{name}\".into(),
				tokens: vec![],
				children: vec![],
			}};
			{body}
			None
		}}
		",
		name = rule.name,
		body = body.join("\n")
	).into()
}

fn generate_rule_body(rule: &Rule, items: &[Cow<'static, AltGroup>]) -> Cow<'static, str> {
	let mut body = String::new();
	let mut loop_string = String::from("loop { \n");
	for alt in &*items {
		let group_prefix = alt.prefix_modifier.as_deref().unwrap_or("");
		let group_suffix = alt.suffix_modifier.as_deref().unwrap_or("");
		let group_concealed = group_prefix == "~";
		let item_list: Vec<(Cow<'static, str>, Cow<'static, str>, Cow<'static, str>, bool)> = alt.alts
			.iter()
			.map(|item| item_details(item.clone(), group_concealed))
			.collect();
		if group_suffix != "*" && group_suffix != "?" {
			for item in &item_list {
				loop_string += &item_body(item, group_prefix);
			}
		}
		if group_suffix == "*" || group_suffix == "+" || group_suffix == "?" {
			loop_string += "loop { \n";
			for item in &item_list {
				loop_string += &item_body(item, group_prefix);
			}
			if group_suffix == "?" {
				loop_string += "break";
			}
			loop_string += "}";
		}
	}
	
	loop_string.push_str(format!(
		"
			return Some((node, {:?}));
		\n}}
		",
		rule.pass_through
	).as_str());
	
	body.push_str(loop_string.as_str());
	body += "self.reset_parse(&mut node, pos);";
	body.into()
}

fn item_details(item: Cow<'static, Alt>, group_concealed: bool) -> (Cow<'static, str>, Cow<'static, str>, Cow<'static, str>, bool) {
	let prefix: Cow<'static, str> = if let Some(p) = item.prefix_modifier.to_owned() {
		Cow::Owned(p.to_string())
	} else {
		Cow::Owned(String::new())
	};
	let suffix: Cow<'static, str> = if let Some(p) = item.suffix_modifier.to_owned() {
		Cow::Owned(p.to_string())
	} else {
		Cow::Owned(String::new())
	};
	let concealed = group_concealed || prefix == "~";
	let method = get_method(&item);
	(method, prefix.to_owned().into(), suffix.to_owned().into(), concealed)
}

fn item_body(item:&(Cow<'static, str>, Cow<'static, str>, Cow<'static, str>, bool), group_prefix: &str) -> Cow<'static, str> {
	if item.1 == "&" || group_prefix == "&" {
		return format!("use_positive_lookahead!(self, self.tokens.index(), &mut {});\n", item.0).into();
	} else if item.1 == "!" || group_prefix == "!" {
		return format!("use_negative_lookahead!(self, self.tokens.index(), &mut {});\n", item.0).into();
	}
	match &*item.2 {
		"?" => format!("use_optional!(&mut node, {}, {});\n", item.0, item.3),
		"*" => format!("use_repeat!(&mut node, {}, {});\n", item.0, item.3),
		"+" => format!("use_item!(&mut node, {}, {});\n", item.0, item.3)
		 	+ format!("use_repeat!(&mut node, {}, {});\n", item.0, item.3).as_str(),
		_ => format!("use_item!(&mut node, {}, {});\n", item.0, item.3),
	}.into()
}

fn get_method(i: &Alt) -> Cow<'static, str> {
	match i.kind.to_string().as_str() {
		// e.g. NUMBER becomes self.expect_number()
		"constant" => format!("self.expect_{}(None)", i.source.source.to_lowercase()),
		// e.g. keyword becomes self._keyword()
		"name" => format!("self._{}(None)", i.source.source),
		// string: e.g. 'abc' becomes self.expect("abc")
		"tag" => format!("self.expect_tag(\"{}\")", i.source.source),
		_ => format!("self.expect({})", i.source.source.replace('\'', "\"")) //"),
	}.into()
}
