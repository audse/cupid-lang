use colored::Colorize;

/// Creates a `&str` for a given type name (stripping away namespace info)
/// # Examples
/// ```
/// use cupid_util::fmt_type;
/// 
/// assert_eq!(fmt_type!(i32), "i32");
/// assert_eq!(fmt_type!(std::collections::HashMap<(), ()>), "HashMap<(), ()>");
/// ```
#[macro_export]
macro_rules! fmt_type {
	($name:path) => {{
		let name = std::any::type_name::<$name>();
		name.rsplit_once("::").map(|n| n.1).unwrap_or_else(|| std::any::type_name::<$name>())
	}}
}

/// Formats each item in a list from the provided arguments
/// 
/// # Examples
/// 
/// ## Basic invocation
/// ```
/// let (input, output) = ([1, 2, 3], vec!["1", "2", "3"]);
/// assert_eq!(cupid_util::fmt_list!(input), output);
/// ```
/// 
/// ## Using separator
/// ```
/// let (input, output) = ([1, 2, 3], "1, 2, 3");
/// assert_eq!(cupid_util::fmt_list!(input, ", "), output);
/// ```
/// 
/// ## Using closure
/// ```
/// let (input, output) = ([1, 2, 3], vec!["1.0", "2.0", "3.0"]);
/// assert_eq!(cupid_util::fmt_list!(input => |i| format!("{i}.0")), output);
/// ```
/// 
/// ## Using separator and closure
/// ```
/// let (input, output) = ([1, 2, 3], "1.0, 2.0, 3.0");
/// assert_eq!(cupid_util::fmt_list!(input, ", " => |i| format!("{i}.0")), output);
/// ```
#[macro_export]
macro_rules! fmt_list {
	($list:expr) => { $list.iter().map(|x| x.to_string()).collect::<Vec<String>>() };
	($list:expr, $sep:tt) => { $list.iter().map(|x| x.to_string()).collect::<Vec<String>>().join($sep) };
	($list:expr => $closure:expr) => { $list.iter().map($closure).collect::<Vec<String>>() };
	($list:expr, $sep:tt => $closure:expr) => { $list.iter().map($closure).collect::<Vec<String>>().join($sep) };
}

/// Provided an option type, returns a formatted string if the option is `Some`, and
/// an empty string otherwise.
/// Optionally, a closure can be provided for more precise formatting.
/// 
/// # Examples
/// 
/// ## Basic invocation
/// ```
/// use cupid_util::fmt_option;
/// assert_eq!(fmt_option!(Option::<&str>::Some("hello")), "hello");
/// assert_eq!(fmt_option!(Option::<&str>::None), "");
/// ```
/// 
/// ## With a closure
/// ```
/// use cupid_util::fmt_option;
/// let x: Option<&str> = Some("hello world");
/// assert_eq!(fmt_option!(x => |x| x.to_uppercase()), "HELLO WORLD");
/// ```
#[macro_export]
macro_rules! fmt_option {
	($option:expr) => { 
		if let Some(x) = $option { 
			x.to_string()
		} else { 
			String::new() 
		}
	};
	($option:expr => |$some:ident| $closure:expr) => { 
		if let Some($some) = $option { $closure } else { String::new() }
	};
}

/// Formats a list with the provided closure only if the list is not empty.
/// If the list is empty, returns an empty string.
/// 
/// # Examples
/// ```
/// use cupid_util::{fmt_if_nonempty, fmt_list};
/// 
/// let generic_formatter = |g: &[&str]| format!("<{}>", fmt_list!(g, ", "));
/// 
/// let empty_list: [&str; 0] = [];
/// let generic_list: [&str; 2] = ["K", "V"];
/// 
/// assert_eq!(fmt_if_nonempty!(&generic_list => generic_formatter), "<K, V>");
/// assert_eq!(fmt_if_nonempty!(&empty_list => generic_formatter), "");
/// ```
#[macro_export]
macro_rules! fmt_if_nonempty {
	($list:expr => $closure:expr) => {
		if $list.is_empty() { String::new() } else { $closure($list) }
	}
}

pub fn draw_line(c: &str, len: usize) -> String {
    (0..=len)
        .collect::<Vec<usize>>()
        .iter()
        .map(|_| c)
        .collect::<Vec<&str>>()
        .join("")
}

pub fn draw_underline(len: usize) -> String {
    draw_line("▔", len)
}

#[macro_export]
macro_rules! lines {
    ($string:expr) => {
        $string.lines().collect::<Vec<&str>>()
    };
}

pub fn bullet_list(m: &[String], point: &str) -> Vec<String> {
    m.iter()
        .map(|p| bullet_point(&p, point))
        .collect()
}

pub fn bullet_point(m: &str, point: &str) -> String {
    textwrap::wrap(m, 50)
        .into_iter()
        .enumerate()
        .map(|(i, line)| if i == 0 {
            format!(" {} {line}", point.dimmed())
        } else {
            format!("   {line}")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn wrap_indent(m: &str, wrap: usize, indent: usize) -> String {
    let indent = draw_line(" ", indent);
    textwrap::wrap(m, wrap)
        .into_iter()
        .map(|line| format!("{indent}{line}"))
        .collect::<Vec<String>>()
        .join("\n")
}