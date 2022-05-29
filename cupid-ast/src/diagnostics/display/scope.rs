use super::*;

impl AsTable for Scope {}
impl Display for Scope {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{}", self.as_table().with(Style::modern()))
	}
}

impl AsTable for Closure {}
impl Display for Closure {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{}", &self.scopes.to_owned().table().with(Style::modern()))
	}
}

impl AsTable for Env {}
impl Display for Env {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{}", self.global.as_named_table("Global scope"))?;
		write!(f, "{}", self.closures.iter()
			.cloned()
			.enumerate()
			.map(|(i, (n, c))| TablePair(quick_fmt!(i, fmt_option!(c.parent, |p| format!(" (parent: {p})")), "\n", fmt_option!(n)), c))
			.table()
			.with(Style::modern())
		)
	}
}