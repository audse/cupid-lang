use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogNode {
	pub identifier: Cow<'static, str>,
	pub args: ArgumentsNode,
}

impl From<&mut ParseNode> for Result<LogNode, Error> {
	fn from(node: &mut ParseNode) -> Self {
    	Ok(LogNode {
			identifier: node.tokens[0].source.to_owned(),
			args: Result::<ArgumentsNode, Error>::from(&mut node.children[0])?
		})
	}
}

impl AST for LogNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
    	let args: Vec<ValueNode> = self.args.resolve_to(scope)?;
		let strings: Vec<String> = args.iter().map(|a| a.to_string()).collect();
		let log_string = match &*self.identifier {
			"log" => format!("\n{}", strings.join("")),
			"log_line" => strings.join(""),
			"logs" => format!("\n{}", strings.join(" ")),
			"logs_line" => strings.join(" "),
			"debug" => if let Value::String(string) = &args[0].value {
				match &**string {
					"scope" => format!("{scope}"),
					_ => panic!("unexpected debug keyword")
				}
			} else {
				panic!("unexpected debug argument")
			}
			_ => panic!("unexpected log keyword")
		};
		print!("{log_string}");
		Ok(ValueNode::new_none()) // TODO change to values
	}
}