use crate::*;

#[trace_this]
impl Trace for Declaration {
	fn trace_declare(&self, scope: &mut Env) {
		scope.trace(quick_fmt!("Declaring variable `", self.name, "` [", self.type_hint, "]"));
	}
	fn trace_declare_param(&self, scope: &mut Env) {
		scope.trace("Declaring parameter...");
	}
	fn trace_type_mismatch(&self, scope: &mut Env) {
		let (expected, found) = (
			self.type_hint.get_node_type().unwrap(), 
			self.value.get_node_type().unwrap()
		);
		scope.trace(format!("Expected type\n{expected}, found type\n{found}"));
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
			self.body.get_node_type().unwrap(),
			self.return_type.get_node_type().unwrap()
		);
		scope.trace(format!("\nExpected to return: \n{return_type}Actually returned: \n{body_type}"));
	}
}

#[trace_this]
impl Trace for Ident {
	fn trace_find_generic_type(&self, scope: &mut Env) {
		scope.trace(format!("Finding type of generic `{self}`"));
	}
}

#[trace_this]
impl Trace for Method {
	fn trace_define_method(&self, scope: &mut Env) {
		scope.trace(format!("Defining method `{}`", self.name));
	}
}

#[trace_this]
impl Trace for Property {
	fn trace_find_property(&self, object_type: &Type, scope: &mut Env) {
		scope.trace(format!("Finding property `{}` of type \n{object_type}", *self.property));
	}
}

#[trace_this]
impl Trace for TraitDef {
	fn trace_define_trait(&self, scope: &mut Env) {
		scope.trace(quick_fmt!("Defining trait ", self.name));
	}
	fn trace_analyze_generic_names(&self, scope: &mut Env) {
		scope.trace(quick_fmt!("Analyzing generics of trait ", self.name));
	}
	fn trace_analyze_generic_types(&self, scope: &mut Env) {
		scope.trace(quick_fmt!("Analyzing types of generics of trait ", self.name));
	}
}

#[trace_this]
impl Trace for TypeDef {
	fn trace_define_type(&self, scope: &mut Env) {
		scope.trace(quick_fmt!("Defining trait ", self.name));
	}
	fn trace_analyze_generic_names(&self, scope: &mut Env) {
		scope.trace(quick_fmt!("Analyzing generics of type ", self.name));
	}
	fn trace_analyze_generic_types(&self, scope: &mut Env) {
		scope.trace(quick_fmt!("Analyzing types of generics of type ", self.name));
	}
}