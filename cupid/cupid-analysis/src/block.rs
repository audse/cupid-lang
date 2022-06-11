use crate::*;

impl PreAnalyze for Block {}

impl Analyze for Block {
    #[trace]
    fn analyze_scope(&mut self, scope: &mut Env) -> ASTResult<()> {
        self.set_closure(scope);
        for exp in self.body.iter_mut() {
            exp.analyze_scope(scope)?;
        }
        Ok(())
    }
    #[trace]
    fn analyze_names(&mut self, scope: &mut Env) -> ASTResult<()> {
        for exp in self.body.iter_mut() {
            exp.analyze_names(scope)?;
        }
        Ok(())
    }
    #[trace]
    fn analyze_types(&mut self, scope: &mut Env) -> ASTResult<()> {
        for exp in self.body.iter_mut() {
            exp.analyze_types(scope)?;
        }
        Ok(())
    }
    #[trace]
    fn check_types(&mut self, scope: &mut Env) -> ASTResult<()> {
        for exp in self.body.iter_mut() {
            exp.check_types(scope)?;
        }
        Ok(())
    }
}

impl TypeOf for Block {
	fn type_of(&self, scope: &mut Env) -> ASTResult<Cow<Type>> { 
        if let Some(exp) = (*self.body).last() {
            exp.type_of(scope)
        } else {
            Ok((Type::none()).into())
        }
    }
}