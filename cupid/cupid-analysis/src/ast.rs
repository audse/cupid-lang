use crate::*;

/// Top-level items such as type definitions and trait definitions go through 
/// similar steps to `Analyze` before anything else is analyzed
#[allow(unused_variables)]
pub trait PreAnalyze: UseAttributes + std::fmt::Display {
	fn pre_analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> { 
		self.attributes_mut().closure = scope.current_closure;
		Ok(())
	}
	fn pre_analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> { Ok(()) }
	fn pre_analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> { Ok(()) }
}

/// Makes three passes over parsed input:
/// 1. `analyze_scope` - adds the scope of each node as an attribute and modifies
/// the scope of its children
/// 2. `analyze_names` - sets symbols for declarations/definitions, and makes sure
/// all referenced symbols exist
/// 3. `analyze_types` - finds the type of each node
/// 4. `check_types` - makes sure that each node has the correct type, e.g. function
/// return types, declarations type annotation, etc.
#[allow(unused_variables)]
pub trait Analyze: PreAnalyze {
	fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> { 
		self.attributes_mut().closure = scope.current_closure;
		Ok(()) 
	}
	fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> { Ok(()) }
	fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> { Ok(()) }
	fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> { Ok(()) }
}