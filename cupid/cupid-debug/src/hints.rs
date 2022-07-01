use std::fmt;
use colored::Colorize;
use cupid_util::{fmt_if_nonempty, bullet_list};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Hints(pub Vec<String>);

impl fmt::Display for Hints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", fmt_if_nonempty!(&self.0 => |_| format!(
            "\n\n{}\n{}", 
            "[help]".green().bold(), 
            bullet_list(&self.0, "*").join("\n")
        )))
    }
}