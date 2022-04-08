pub fn is_uppercase(string: String) -> bool {
	string.chars().all(|c| match c {
		'A'..='Z' | '_' => true,
		_ => false
	})
}

pub fn is_identifier(string: String) -> bool {
	string.chars().all(|c| match c {
		'a'..='z' | 'A'..='Z' | '_' => true,
		_ => false
	})
}

pub fn is_string(string: String) -> bool {
	let mut chars = string.chars();
	let start_quote = chars.next().unwrap_or('\0');
	if start_quote != '\'' && start_quote != '"' { return false; } //"
	return chars.last().unwrap_or('\0') == start_quote;
}

pub fn is_number(string: String) -> bool {
	string.chars().all(|c| match c {
		'0'..='9' | '_' => true,
		_ => false
	})
}