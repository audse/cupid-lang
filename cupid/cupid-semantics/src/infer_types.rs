use cupid_env::environment::Env;
use crate::Error;

pub trait InferTypes where Self: Sized {
    fn infer_types(self, env: &mut Env) -> Result<Self, Error>;
}