use crate::*;

pub type Source = usize;

pub trait Analyze: UseAttributes {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> { 
		self.attributes().closure = scope.current_closure;
		Ok(()) 
	}
	fn analyze_names(&mut self, _scope: &mut Env) -> Result<(), (Source, ErrCode)> { Ok(()) }
	fn analyze_types(&mut self, _scope: &mut Env) -> Result<(), (Source, ErrCode)> { Ok(()) }
	fn check_types(&mut self, _scope: &mut Env) -> Result<(), (Source, ErrCode)> { Ok(()) }
}

pub trait Interpret {
	fn interpret<T>(&mut self, scope: &mut Env) -> Result<T, Error>;
}
