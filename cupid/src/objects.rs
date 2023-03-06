pub mod class;
pub use self::class::Class;

pub mod closure;
pub use self::closure::Closure;

pub mod function;
pub use self::function::{Function, NativeFunction};

pub mod instance;
pub use self::instance::Instance;

pub mod method;
pub use self::method::BoundMethod;

pub mod string;
pub use self::string::Str;

pub mod upvalue;
pub use self::upvalue::{FunctionUpvalue, Upvalue};

#[derive(Debug)]
pub enum ObjectType {
    Function,
    Closure,
    Str,
    Upvalue,
    Class,
    Instance,
    BoundMethod,
}
