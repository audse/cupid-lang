use crate::*;

impl Analyze for Exp {
	fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.traceback.push(quick_fmt!("Analyzing scope of ", self));
		if let Self::Empty = self { return Ok(()) }
		for_each_exp!(self, analyze_scope, scope)
	}
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.traceback.push(quick_fmt!("Analyzing names of ", self));
		if let Self::Empty = self { return Ok(()) }
		for_each_exp!(self, analyze_names, scope)
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.traceback.push(quick_fmt!("Analyzing types of ", self));
		if let Self::Empty = self { return Ok(()) }
		for_each_exp!(self, analyze_types, scope)
	}
	fn check_types(&mut self, scope: &mut Env) -> Result<(), ASTErr> {
		scope.traceback.push(quick_fmt!("Checking types of ", self));
		if let Self::Empty = self { return Ok(()) }
		for_each_exp!(self, check_types, scope)
	}
}

impl UseAttributes for Exp {
	fn attributes(&self) -> &Attributes {
		for_each_exp!(self, attributes)
	}
	fn attributes_mut(&mut self) -> &mut Attributes {
		for_each_exp!(self, attributes_mut)
	}
}

impl TypeOf for Exp {
	fn type_of(&self, scope: &mut Env) -> Result<Type, ASTErr> {
		if let Self::Empty = self {
			return Ok(NOTHING.to_owned())
		}
		for_each_exp!(self, type_of, scope)
	}
}