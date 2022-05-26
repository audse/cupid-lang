use super::*;

impl Display for Scope {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.debug_map().entries(self.symbols.iter()).finish()
	}
}

impl Display for Closure {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    	f.debug_list().entries(self.scopes.iter()).finish()
	}
}

impl Display for Env {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    	f.debug_struct("Env")
			.field("Global scope", &self.global)
			.field("Closures", &self.closures)
			.finish()
	}
}