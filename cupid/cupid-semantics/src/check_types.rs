use cupid_env::environment::Env;
use crate::Error;

pub trait CheckTypes where Self: Sized {
    fn check_types(self, env: &mut Env) -> Result<Self, Error>;
}