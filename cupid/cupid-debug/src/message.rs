#![allow(unused)]
use cupid_ast::expr::ident::Ident;
use crate::code::ErrorCode;

pub trait ErrorMessage<Args> {
    fn message(&self, code: ErrorCode, context: impl Fn(Args) -> String) -> String {
        "Something went wrong, but we are not sure what.".to_string()
    }
}

impl ErrorMessage<Ident> for ErrorCode {
    fn message(&self, code: ErrorCode, context: impl Fn(Ident) -> String) -> String {
        todo!("
            Match a code to an error and provide extra context
            e.g. `already defined` error could show where the original definition happened
        ")
    }
}