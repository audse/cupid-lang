mod bidirectional_iterator;
pub use bidirectional_iterator::*;

mod displays;
pub use displays::*;

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

pub (crate) fn unwrap_or_string<T>(option: &Option<T>) -> String where T: ToString {
	if let Some(val) = option {
		val.to_string()
	} else {
		String::from("?")
	}
}