#![cfg(test)]
use std::rc::Rc;
use cupid_lex::{token::Token, span::Position};
use crate::{source::*, error::*, hints::Hints};

#[allow(unused)]
const TEST: &'static str = "
type person = [
    name : string,
    age : int
]
let me : person = [
    name : Audrey,
    age : 23
]
";

#[test]
fn test() {
    let source = TEST.to_string();
    let msg = Error {
        message: "An error has occurred! Something has happened that we did not expect. We are not really sure what to do now, to be honest.".to_string(),
        source: Source(Rc::new(source)),
        context: Rc::new(ExprSource::Ident(IdentSource {
            token_name: Token::new(
                Position { line: 5, character: 4 },
                Position { line: 5, character: 5 },
                "me".to_string(),
                cupid_lex::token::TokenKind::Ident
            ),
            ..Default::default()
        })),
        hints: Hints(vec![
            "Have you tried turning the program off and back on again? I hear that usually works.".to_string(),
            "I got nothing else. You're on your own.".to_string()
        ]),
        ..Default::default()
    };
    eprintln!("{msg}")
}