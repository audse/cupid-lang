use cupid_env::environment::Env;
use crate::Error;

pub trait CheckFlow where Self: Sized {
    fn check_flow(self, env: &mut Env) -> Result<Self, Error>;
}