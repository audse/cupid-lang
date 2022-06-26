#![feature(let_chains)]

pub mod infer;

pub(self) type Str = std::borrow::Cow<'static, str>;
