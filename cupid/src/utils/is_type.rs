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