use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Block {
	pub body: Vec<Exp>,
	pub attributes: Attributes,
}

impl Analyze for Block {
	fn analyze_names(&mut self, scope: &mut Env) -> Result<(), ErrCode> {
    	for exp in self.body.iter_mut() {
			exp.analyze_names(scope)?;
		}
		Ok(())
	}
	fn analyze_types(&mut self, scope: &mut Env) -> Result<(), ErrCode> {
    	for exp in self.body.iter_mut() {
			exp.analyze_types(scope)?;
		}
		Ok(())
	}
}

impl UseAttributes for Block {
	fn attributes(&mut self) -> &mut Attributes { &mut self.attributes }
}

impl TypeOf for Block {
	fn type_of(&self, scope: &mut Env) -> Result<Type, ErrCode> {
    	if let Some(exp) = (*self.body).last() {
			exp.type_of(scope)
		} else {
			Ok(Type::default())
		}
	}
}