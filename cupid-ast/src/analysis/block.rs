use crate::*;

impl PreAnalyze for Block<'_> {}

impl Analyze for Block<'_> {
    fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
        for exp in self.body.iter_mut() {
            exp.analyze_scope(scope)?;
        }
        Ok(())
    }
    fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
        for exp in self.body.iter_mut() {
            exp.analyze_names(scope)?;
        }
        Ok(())
    }
    fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
        for exp in self.body.iter_mut() {
            exp.analyze_types(scope)?;
        }
        Ok(())
    }
    fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
        for exp in self.body.iter_mut() {
            exp.check_types(scope)?;
        }
        Ok(())
    }
}

impl UseAttributes for Block<'_> {
    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}

impl TypeOf for Block<'_> {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<'_, Type>> { 
        if let Some(exp) = (*self.body).last() {
            exp.type_of(scope)
        } else {
            Ok((&*NOTHING).into())
        }
    }
}
