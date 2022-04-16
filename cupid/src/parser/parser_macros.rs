
macro_rules! node_name {
	($name:ident) => {
		stringify!($name).replacen("_", "", 1)
	}
}

macro_rules! all_of {
	($parser:ident, $node:ident, $pos:ident, $pass_through:literal, $block:block) => { 
		loop {
			$block
			return Some(($node, $pass_through));
		}
		$parser.reset_parse(&mut $node, $pos);
	};
}

macro_rules! parse {
	($parser:ident, $name:ident, $node:ident, $pos:ident, $($block:block),*) => {
		pub fn $name(&mut $parser) -> Option<(Node, bool)> {
			let start_pos = $parser.tokens.index();
			let $pos = start_pos;
			let mut $node = Node {
				name: crate::node_name!($name),
				tokens: vec![],
				children: vec![],
			};
			$($block)*
			return None as Option<(Node, bool)>;
		}
	}
}

pub(crate) use node_name;
pub(crate) use parse;
pub(crate) use all_of;