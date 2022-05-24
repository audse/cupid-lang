use crate::*;

pub trait Analyze: UseAttributes {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), ErrCode> { 
		self.attributes().closure = scope.current_closure;
		Ok(()) 
	}
	fn analyze_names(&mut self, _scope: &mut Env) -> Result<(), ErrCode> { Ok(()) }
	fn analyze_types(&mut self, _scope: &mut Env) -> Result<(), ErrCode> { Ok(()) }
	fn check_types(&mut self, _scope: &mut Env) -> Result<(), ErrCode> { Ok(()) }
}

pub trait Interpret {
	fn interpret<T>(&mut self, scope: &mut Env) -> Result<T, Error>;
}

