use crate::*;

#[allow(unused_variables)]
pub trait Analyze: UseAttributes + std::fmt::Display {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), ASTErr> { 
		self.attributes_mut().closure = scope.current_closure;
		Ok(()) 
	}
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ASTErr> { Ok(()) }
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> { Ok(()) }
	fn check_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> { Ok(()) }
}

pub trait Interpret {
	fn interpret<T>(&mut self, scope: &mut Env) -> Result<T, Error>;
}