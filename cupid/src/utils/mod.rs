mod bidirectional_iterator;
pub use bidirectional_iterator::*;

mod is_type;
pub use is_type::*;

pub fn pluralize<N, S>(number: N, word: S) -> String where N: Into<usize>, S: Into<String> {
	let num: usize = number.into();
	if num == 0 || num > 1 {
		format!("{}s", word.into())
	} else {
		word.into()
	}
}

pub fn pluralize_word<N, S>(number: N, word: S) -> String where N:Into<usize>, S: Into<String> {
	let num: usize = number.into();
	let is_plural = num == 0 || num > 1;
	match is_plural {
		true => match word.into().as_str() {
			"was" => String::from("were"),
			_ => unreachable!()
		},
		false => word.into()
	}
}