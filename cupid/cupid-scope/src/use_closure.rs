use crate::*;

pub trait UseClosure: UseAttributes {
	fn set_closure(&mut self, scope: &Env) {
		self.attributes_mut().closure = scope.current_closure;
	}
	fn set_closure_to(&mut self, closure: usize) {
		self.attributes_mut().closure = closure;
	}
	fn closure(&self) -> usize {
		self.attributes().closure
	}
	fn use_closure(&self, scope: &mut Env) {
		scope.use_closure(self.closure(), fmt_type!(Self));
	}
}

impl UseClosure for Block {}
impl UseClosure for Exp {}
impl UseClosure for FunctionCall {}
impl UseClosure for Function {}
impl UseClosure for Ident {}
impl UseClosure for Property {}
impl UseClosure for Implement {}
impl UseClosure for Method {}
impl UseClosure for TraitDef {}
impl UseClosure for Trait {}
impl UseClosure for TypeDef {}
impl UseClosure for Type {}