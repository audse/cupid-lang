
#[derive(Debug, Default, Clone)]
pub struct Field<Id>(pub Id, pub Option<Id>);

impl<Id: Default + Clone> Field<Id> {
	pub fn pass<NextId: Default + Clone>(
		self,
		fun: impl FnOnce(Id, &mut crate::Env) -> crate::PassResult<NextId>,
		option_fun: impl FnOnce(Option<Id>, &mut crate::Env) -> crate::PassResult<Option<NextId>>,
		env: &mut crate::Env,
	) -> crate::PassResult<Field<NextId>> {
		let Field(name, annotation) = self;
		Ok(Field(
			fun(name, env)?,
			option_fun(annotation, env)?
		))
	}
}
