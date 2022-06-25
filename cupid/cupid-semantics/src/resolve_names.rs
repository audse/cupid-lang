use cupid_env::environment::Env;
use crate::Error;

pub trait ResolveNames where Self: Sized {
    fn resolve_names(self, env: &mut Env) -> Result<Self, Error>;
}