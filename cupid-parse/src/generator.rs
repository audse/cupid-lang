use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grammar {
	pub name: Str,
	pub rules: Vec<Rule>,
}

impl Grammar {
	pub fn stringify(&self) -> String {
		let (_, body) = (&self.name, self.body());
		body
	}
	fn body(&self) -> String {
		let body: Vec<String> = self.rules
			.iter()
			.map(|rule| rule.stringify())
			.collect();
		body.join("")
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rule {
	pub name: Str,
	pub alts: Vec<Alt>,
	// to be included in the rule tree, or "passed through" to an encapsulating rule
	pub pass_through: bool,
	pub inverse: bool,
	pub params: Vec<Str>,
}

impl Rule {
	pub fn stringify(&self) -> String {
		let params = self.params();
		let body: Vec<String> = self.alts
			.iter()
			.map(|alt| self.body(alt))
			.collect();
		format!(
			"
			pub fn _{name}(&mut self{params}) -> Option<(ParseNode, bool)> {{
				let (mut node, pos) = self.start_parse(\"{name}\");
				{body}
				None
			}}
			",
			name = self.name,
			body = body.join("\n")
		)
	}
	fn params(&self) -> String {
		let params: Vec<String> = self.params
			.iter()
			.map(|param| format!("{param}: &ParseFun"))
			.collect();
		if params.is_empty() {
			String::new()
		} else {
			format!(", {}", params.join(", "))
		}
	}
	fn body(&self, groups: &[StaticGroup]) -> String {
		let group_strings: Vec<String> = groups
			.iter()
			.map(|group| group.stringify(&self.params))
			.collect();
		let macro_name = if self.inverse {
			"alt_inverse"
		} else {
			"alt"
		};
		format!("
			{macro_name}! ((self, {pass_through}, node, pos) {{
				{alt_body}
			}});",
			pass_through = self.pass_through,
			alt_body = group_strings.join("\n")
		)
	}
}

pub type Alt = Vec<StaticGroup>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Group {
	pub items: Vec<StaticItem>,
	pub prefix_modifier: Option<Str>,
	pub suffix_modifier: Option<Str>,
}

pub type StaticGroup = Cow<'static, Group>;

impl Group {
	pub fn stringify(&self, params: &[Str]) -> String {
		let mut string = String::new();
		let items = self.items(params).join("\n");
		
		let (once, multiple, optional) = self.repeat();
				
		if once {
			string += &items;
		}
		if multiple {
			let concealed = self.concealed();
			let break_statement = if optional { "break;" } else { "" };
			
			string += &format!("
				group! ((self, {concealed}, node, pos) {{
					{items}
					{break_statement}
				}});
			")
		}
		string
	}
	fn prefix(&self) -> &str {
		self.prefix_modifier.as_deref().unwrap_or("")
	}
	fn suffix(&self) -> &str {
		self.suffix_modifier.as_deref().unwrap_or("")
	}
	fn concealed(&self) -> bool {
		&*(self.prefix()) == "~"
	}
	fn items(&self, params: &[Str]) -> Vec<String> {
		let prefix = self.prefix();
		self.items
			.iter()
			.map(|item| item.stringify(prefix, params))
			.collect()
	}
	fn repeat(&self) -> (bool, bool, bool) {
		let suffix = self.suffix();
		(
			(suffix != "*" && suffix != "?"), // at least once
			(suffix == "*" || suffix == "+" || suffix == "?"), // n amount of times
			(suffix == "?") // optional
		)
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Item {
	pub kind: Str,
	pub source: Token,
	pub prefix_modifier: Option<Str>,
	pub suffix_modifier: Option<Str>,
	pub args: Vec<StaticItem>,
}

pub type StaticItem = Cow<'static, Item>;

impl Item {
	pub fn stringify(&self, group_prefix: &str, params: &[Str]) -> String {
		let body = self.body(group_prefix, params);
		body.join("\n")
	}
	fn source(&self) -> &Str {
		&self.source.source
	}
	fn prefix(&self) -> &str {
		self.prefix_modifier.as_deref().unwrap_or("")
	}
	fn suffix(&self) -> &str {
		self.suffix_modifier.as_deref().unwrap_or("")
	}
	fn is_param(&self, params: &[Str]) -> bool {
		params.iter().any(|param| **param == *self.source())
	}
	fn method(&self) -> Str {
		let source = self.source();
		
		match &*self.kind {
			"constant" => format!("expect_{}", source.to_lowercase()),
			"name" => format!("_{source}"),
			"tag" => format!("expect_tag(\"{source}\")"),
			_ => format!("expect(r{})", escape(source)),
		}.into()
	}
	fn method_call(&self, params: &[Str]) -> String {
		let args = self.args(params);
		if self.is_param(params) {
			format!("{}{args}", self.source())
		} else {
			let method = self.method();
			format!("self.{method}{args}")
		}
	}
	fn args(&self, params: &[Str]) -> String {
		if !["constant", "name"].contains(&&*self.kind) {
			return String::new();
		}
		let args: Vec<String> = self.args
			.iter()
			.map(|arg| if arg.is_param(params) {
				format!("{}", arg.source())
			} else {
				format!("&Self::{}", arg.method())
			})
			.collect();
		if self.is_param(params) {
			"(self)".to_string()
		} else if args.is_empty() {
			"()".to_string()
		} else {
			format!("({})", args.join(", "))
		}
	}
	fn macros(&self, group_prefix: &str) -> Vec<&str> {
		match (&*(self.prefix()), group_prefix) {
			("&", _) | (_, "&") => return vec!["positive_lookahead"],
			("!", _) | (_, "!") => return vec!["negative_lookahead"],
			("=!", _) | (_, "=!") => return vec!["negative_lookbehind"],
			("=&", _) | (_, "=&") => return vec!["positive_lookbehind"],
			_ => ()
		};
		match &*(self.suffix()) {
			"?" => vec!["optional"],
			"*" => vec!["repeat"],
			"+" => vec!["once", "repeat"],
			_ => vec!["once"],
		}
	}
	fn concealed(&self, group_prefix: &str) -> bool {
		&*(self.prefix()) == "~" || group_prefix == "~"
	}
	fn body(&self, group_prefix: &str, params: &[Str]) -> Vec<String> {
		let method = self.method_call(params);
		let concealed = self.concealed(group_prefix);
		let macros = self.macros(group_prefix);
		match macros[..] {
			["positive_lookahead", ..]
			| ["negative_lookahead", ..]
			| ["negative_lookbehind", ..]
			| ["positive_lookbehind", ..] => {
				let name = macros[0];
				vec![format!("use_{name}!(self, self.tokens().index(), {method});")]
			},
			_ => macros
				.iter()
				.map(|macro_name| format!("{macro_name}!(&mut node, {method}, {concealed});"))
				.collect()
		}
	}
}

fn escape(string: &str) -> String {
	string.replace('\'', "\"") //"
}