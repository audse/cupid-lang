use crate::*;

impl Analyze for Block {
    fn analyze_scope(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
        for exp in self.body.iter_mut() {
            exp.analyze_scope(scope)?;
        }
        Ok(())
    }
    fn analyze_names(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
        for exp in self.body.iter_mut() {
            exp.analyze_names(scope)?;
        }
        Ok(())
    }
    fn analyze_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
        for exp in self.body.iter_mut() {
            exp.analyze_types(scope)?;
        }
        Ok(())
    }
    fn check_types(&mut self, scope: &mut Env) -> Result<(), (Source, ErrCode)> {
        for exp in self.body.iter_mut() {
            exp.check_types(scope)?;
        }
        Ok(())
    }
}

impl UseAttributes for Block {
    fn attributes(&mut self) -> &mut Attributes {
        &mut self.attributes
    }
}

impl TypeOf for Block {
    fn type_of(&self, scope: &mut Env) -> Result<Type, (Source, ErrCode)> {
        if let Some(exp) = (*self.body).last() {
            exp.type_of(scope)
        } else {
            Ok(NOTHING.to_owned())
        }
    }
}
