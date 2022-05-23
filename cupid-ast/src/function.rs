use crate::*;

mod signature;
pub use signature::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
	pub body: Vec<Exp>,
	pub return_type: Option<Ident>
}

impl Analyze for Block {} // TODO

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
	pub body: Block,
	pub signature: Signature,
	pub closure: Option<usize>,
}

impl Analyze for Function {
	// fn resolve_names(&mut self, scope: &mut Env) -> Result<(), Error> {
    // 	
	// }
	fn resolve_types(&mut self, scope: &mut Env) -> Result<(), Error> {
		self.body.resolve_types(scope)?;
		let closure = scope.add_closure();
		
		Ok(())
	}
}
