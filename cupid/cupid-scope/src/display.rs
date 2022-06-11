use super::*;

impl Display for Scope {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.as_table().with(Style::modern()))
	}
}

impl Display for Closure {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", &self.scopes.to_owned().table().with(Style::modern()))
	}
}

impl Display for Env {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.global.as_named_table("Global scope"))?;
		write!(f, "{}", self.closures.iter()
			.cloned()
			.enumerate()
			.map(|(i, c)| TablePair(
				quick_fmt!(
					i, 
					fmt_option!(&c.parent, |p| format!(" (parent: {p})")), 
					"\n", 
					fmt_option!(&c.name)
				), 
				c
			))
			.table()
			.with(Style::modern())
		)
	}
}