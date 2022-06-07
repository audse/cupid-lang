use crate::*;

macro_rules! analyze_exp {
	($method:ident) => {
		fn $method(&mut self, scope: &mut Env) -> ASTResult<()> {
			if let Self::Empty = self { return Ok(()) }
			for_each_exp!(self, $method, scope)
		}
	};
}

impl PreAnalyze for Exp {
	analyze_exp!(pre_analyze_scope);
	analyze_exp!(pre_analyze_names);
	analyze_exp!(pre_analyze_types);
}

impl Analyze for Exp {
	analyze_exp!(analyze_scope);
	analyze_exp!(analyze_names);
	analyze_exp!(analyze_types);
	analyze_exp!(check_types);
}

impl TypeOf for Exp {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> { 
		if let Self::Empty = self {
            return Ok(nothing_type().into())
		}
		for_each_exp!(self, type_of, scope)
	}
}