use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogNode {
	pub identifier: Cow<'static, str>,
	pub args: ArgumentsNode,
}

impl From<&mut ParseNode> for LogNode {
	fn from(node: &mut ParseNode) -> Self {
    	Self {
			identifier: node.tokens[0].source.to_owned(),
			args: ArgumentsNode::from(&mut node.children[0])
		}
	}
}

impl AST for LogNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
    	let args: ValueNode = self.args.resolve(scope)?;
		let args_list: &Vec<ValueNode> = if let Value::Values(values) = &args.value {
			values
		} else {
			panic!("expected value list in log args")
		};
		let strings: Vec<String> = args_list.iter().map(|a| a.to_string()).collect();
		let log_string = match &*self.identifier {
			"log" => format!("\n{}", strings.join("")),
			"log_line" => strings.join(""),
			"logs" => format!("\n{}", strings.join(" ")),
			"logs_line" => strings.join(" "),
			_ => panic!("unexpected log keyword")
		};
		print!("{log_string}");
		Ok(args)
	}
}