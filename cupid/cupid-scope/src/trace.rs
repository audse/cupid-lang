use crate::*;

pub trait Trace {}

#[trace_this]
impl Trace for Block {
    fn trace_enter_block(&self, scope: &mut Env) {
        scope.trace("Entered block");
    }
    fn trace_exit_block(&self, scope: &mut Env) {
        scope.trace("Exited block");
    }
}

#[trace_this]
impl Trace for Declaration {
	fn trace_declare(&self, scope: &mut Env) {
		scope.trace(quick_fmt!("Declaring variable..."));
	}
	fn trace_declare_param(&self, scope: &mut Env) {
		scope.trace("Declaring parameter...");
	}
	fn trace_type_mismatch(&self, scope: &mut Env) {
		let (expected, found) = (
			self.type_hint.get_type().unwrap(), 
			self.value.get_type().unwrap()
		);
		scope.trace(format!("Expected type\n{}, found type\n{}", expected as &dyn Fmt, found as &dyn Fmt));
	}
}


#[trace_this]
impl Trace for FunctionCall {
	fn trace_analyze_arg_names(&self, scope: &mut Env) {
		scope.trace("Analyzing names in arguments...");
	}
	fn trace_analyze_arg_types(&self, scope: &mut Env) {
		scope.trace("Analyzing types of arguments...");
	}
}

#[trace_this]
impl Trace for Function {
	fn trace_type_mismatch(&self, scope: &mut Env) {
		let (body_type, return_type) = (
			self.body.get_type().unwrap(),
			self.return_type.get_type().unwrap()
		);
		scope.trace(format!("\nExpected to return: \n{}Actually returned: \n{}", return_type as &dyn Fmt, body_type as &dyn Fmt));
	}
}

#[trace_this]
impl Trace for Ident {
	fn trace_find_generic_type(&self, scope: &mut Env) {
		scope.trace(format!("Finding type of generic `{}`", self as &dyn Fmt));
	}
}

#[trace_this]
impl Trace for Method {
	fn trace_define_method(&self, scope: &mut Env) {
		scope.trace(format!("Defining method `{}`", &self.name as &dyn Fmt));
	}
}

#[trace_this]
impl Trace for Property {
	fn trace_find_property(&self, object_type: &Type, scope: &mut Env) {
		scope.trace(format!(
			"Finding property `{}` of type \n{}", 
			&*self.property as &dyn Fmt, 
			object_type as &dyn Fmt
		));
	}
}

#[trace_this]
impl Trace for TraitDef {
	fn trace_define_trait(&self, scope: &mut Env) {
		scope.trace(quick_fmt!("Defining trait ", &self.name as &dyn Fmt));
	}
	fn trace_analyze_generic_names(&self, scope: &mut Env) {
		scope.trace(quick_fmt!("Analyzing generics of trait ", &self.name as &dyn Fmt));
	}
	fn trace_analyze_generic_types(&self, scope: &mut Env) {
		scope.trace(quick_fmt!("Analyzing types of generics of trait ", &self.name as &dyn Fmt));
	}
}

#[trace_this]
impl Trace for TypeDef {
	fn trace_define_type(&self, scope: &mut Env) {
		scope.trace(quick_fmt!("Defining trait ", &self.name as &dyn Fmt));
	}
	fn trace_analyze_generic_names(&self, scope: &mut Env) {
		scope.trace(quick_fmt!("Analyzing generics of type ", &self.name as &dyn Fmt));
	}
	fn trace_analyze_generic_types(&self, scope: &mut Env) {
		scope.trace(quick_fmt!("Analyzing types of generics of type ", &self.name as &dyn Fmt));
	}
}