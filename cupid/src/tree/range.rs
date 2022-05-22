use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeNode {
	pub inclusive: (bool, bool),
	pub start: BoxAST,
	pub end: BoxAST,
	pub meta: Meta<()>
}

impl FromParse for Result<RangeNode, Error> {
	fn from_parse(node: &mut ParseNode) -> Self {
		let range = &mut node.children[0];
		let inclusive = match &*range.name {
			"range_inclusive_inclusive" => (true, true),
			"range_inclusive_exclusive" => (true, false),
			"range_exclusive_exclusive" => (false, false),
			"range_exclusive_inclusive" => (false, true),
			_ => panic!()
		};
		let start = parse(&mut range.children[0])?;
		let end = parse(&mut range.children[1])?;
		Ok(RangeNode {
			inclusive,
			start,
			end,
			meta: Meta::with_tokens(node.collect_tokens())
		})
	}
}

impl AST for RangeNode {
	fn resolve(&self, scope: &mut LexicalScope) -> Result<ValueNode, Error> {
		let start = self.start.resolve(scope)?;
		let end = self.end.resolve(scope)?;
		
		let mut meta = Meta::<Flag>::from(&self.meta);
		meta.set_token_store(scope);
		let mut start_meta = Meta::<Flag>::from(&start.meta);
		start_meta.set_token_store(scope);
		
		match (&start.value, &end.value) {
			(Value::Integer(s), Value::Integer(e)) => {
				let s = *s;
				let e = *e;
				let r: Vec<i32> = match (self.inclusive.0, self.inclusive.1, s < e) {
					// [0..10]
					(true, true, true) => (s..=e).collect(),
					// 0[..]10
					(false, false, true) => (s + 1..e).collect(),
					// [0..]10
					(true, false, true) => (s..e).collect(),
					// 0[..10]
					(false, true, true) => (s + 1..=e).collect(),
					
					// [10..0]
					(true, true, false) => (e..=s).rev().collect(),
					// 10[..]0
					(false, false, false) => (e..s - 1).rev().collect(),
					// [10..]0
					(true, false, false) => (e..s).rev().collect(),
					// 10[..0]
					(false, true, false) => (e..=s - 1).rev().collect(),
					// _ => panic!()
				};
				let a: Vec<ValueNode> = r.into_iter().map(
					|i| ValueNode::from((Value::Integer(i), meta.to_owned()))
				).collect();
				let value = ValueNode::from((Value::Array(a), start_meta));
				Ok(value)
			},
			(x, y) => Err(start.error(format!("start and end of an array must by integers, not {x} and {y}"), scope))
		}
	}
}

impl Display for RangeNode {
	fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
		write!(f, "{self:?}")
	}
}