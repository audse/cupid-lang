use crate::*;

pub fn generate_parser(rules: Vec<Rule>) -> Str {
	println!("{:#?}", rules);
	let body: Vec<String> = rules
		.iter()
		.map(|rule| rule.stringify())
		.collect();
	body.join("").into()
}

// fn generate_rule(rule: &Rule) -> Str {
// 	let body: Vec<Str> = rule.alts
// 		.iter()
// 		.map(|alt| generate_rule_body(&rule, alt))
// 		.collect();
// 	format!(
// 		"
// 		pub fn _{name}(&mut self) -> Option<(ParseNode, bool)> {{
// 			let (mut node, pos) = self.start_parse(\"{name}\");
// 			{body}
// 			None
// 		}}
// 		",
// 		name = rule.name,
// 		body = body.join("\n")
// 	).into()
// }
// 
// fn generate_rule_body(rule: &Rule, groups: &[StaticGroup]) -> Str {
// 	let group_strings: &[String] = &*groups
// 		.iter()
// 		.map(|group| group.stringify())
// 		.collect::<Vec<String>>();
// 	
// 	format!("
// 		alt! ((self, {pass_through}, node, pos) {{
// 			{alt_body}
// 		}});",
// 		pass_through = rule.pass_through,
// 		alt_body = group_strings.join("")
// 	).into()
// }
