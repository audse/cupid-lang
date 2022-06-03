pub use lazy_static::lazy_static;

mod type_definition;
pub use type_definition::*;

mod inference;
pub use inference::*;

mod methods;
pub use methods::*;

mod trait_definition;
pub use trait_definition::*;

mod traits;
pub use traits::*;

mod types;
pub use types::*;