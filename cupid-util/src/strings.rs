use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

pub struct CowStr(Cow<'static, str>);
impl Deref for CowStr {
	type Target = Cow<'static, str>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for CowStr {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl CowStr {
	pub fn borrow(s: &'static str) -> Self {
		Self(Cow::Borrowed(s))
	}
	pub fn own(s: String) -> Self {
		Self(Cow::Owned(s))
	}
}

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

pub fn unwrap_or_string<T>(option: &Option<T>) -> String where T: ToString {
	if let Some(val) = option {
		val.to_string()
	} else {
		String::from("?")
	}
}

pub fn dir_from_path<T: ToString>(path: T) -> String {
	path.to_string().rsplit_once('/').unwrap_or_default().0.to_string()
}

pub fn is_uppercase(string: &str) -> bool {
	string.chars().all(|c| matches!(c, 'A'..='Z' | '_'))
}

pub fn is_identifier(string: &str) -> bool {
	string.chars().all(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '!'))
}

pub fn is_string(string: &str) -> bool {
	let mut chars = string.chars();
	let start_quote = chars.next().unwrap_or('\0');
	if start_quote != '\'' && start_quote != '"' { return false; } //"
	chars.last().unwrap_or('\0') == start_quote
}

pub fn is_number(string: &str) -> bool {
	string.chars().all(|c| matches!(c, '0'..='9' | '_'))
}

pub fn is_tag(string: &str) -> bool {
	let mut chars = string.chars();
	if chars.next().unwrap_or('\0') != '<' { return false; }
	chars.last().unwrap_or('\0') == '>'
}