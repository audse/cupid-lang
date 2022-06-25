use cupid_env::environment::Env;
use crate::Error;

pub trait Lint where Self: Sized {
    fn lint(self, env: &mut Env) -> Result<Self, Error>;
}