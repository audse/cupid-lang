use crate::*;

#[allow(unused_variables)]
pub trait PreAnalyze: UseAttributes + std::fmt::Display {
	// Some nodes are analyzed before other things: e.g. top-level type definitions
	fn pre_analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> { 
		self.attributes_mut().closure = scope.current_closure;
		Ok(())
	}
	fn pre_analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> { Ok(()) }
	fn pre_analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> { Ok(()) }
}

#[allow(unused_variables)]
pub trait Analyze: PreAnalyze {
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