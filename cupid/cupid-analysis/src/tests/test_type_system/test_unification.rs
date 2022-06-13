#![allow(unused)]

#[cfg(test)]
use crate::*;

fn string() -> cupid_ast::Type { 
    cupid_builder::primitive!("string") 
}

fn string_ident() -> cupid_ast::Typed<cupid_ast::Ident> { 
    cupid_builder::primitive!("string").into()
}

fn int() -> cupid_ast::Type { 
    cupid_builder::primitive!("int") 
}

fn int_ident() -> cupid_ast::Typed<cupid_ast::Ident> { 
    cupid_builder::primitive!("int").into()
}

#[test]
fn test_unify_generics() {
    /// assert `unify ( map (<k>, <v>), map (string, int) )` is ok: `map (string, int)`
    let mut map_type = cupid_builder::build! {
        "map" ("k", "v") = [
            "key" => "k",
            "value" => "v"
        ]
    };
    let concrete_map_type = cupid_builder::build! {
        "map" (string_ident(), int_ident()) = [
            "key": string_ident(),
            "value": int_ident()
        ]
    };
    assert!(map_type.unify(&concrete_map_type).is_ok());
    assert_eq!(map_type, concrete_map_type);
}

#[test]
fn test_unify_one_generic() {
    /// assert `unify ( result (<t>, <e>), result (<t>, string) )` is ok: `result (<t>, string)`
    // `type result (t, e) = [ ok : t, error : e ]`
    let mut result_type = cupid_builder::build! {
        "result" ("t", "e") = [
            "ok" => "t",
            "error" => "e"
        ]
    };
    // `let my_val = result::error('something went wrong')`
    // infers to `result (t) = [ ok : t, error : string ]`
    let example_result_type = cupid_builder::build! {
        "result" ("t".into(), string_ident()) = [
            "ok": "t".into(),
            "error": string_ident()
        ]
    };
    assert!(result_type.unify(&example_result_type).is_ok());
    assert_eq!(result_type, example_result_type);
}

#[test]
fn test_unify_no_generics() {
    /// assert `unify (int, int)` is ok
    let mut int_type = int();
    assert!(int_type.unify(&int()).is_ok());
    assert_eq!(int_type, int());
}

#[test]
fn test_cannot_unify_primitive() {
    /// assert `unify (int, string)` is error
    let mut int_type = int();
    let string_type = string();
    assert!(int_type.unify(&string_type).is_err());
}

#[test]
fn test_cannot_unify_too_many_fields() {
    /// assert `unify ( type[field], type[field, field] )` is error
    let mut type_a = cupid_builder::build! {
        "type" () = [
            "field" : int_ident()
        ]
    };
    let type_b = cupid_builder::build! {
        "type" () = [
            "field" : int_ident(),
            "field" : int_ident()
        ]
    };
    assert!(type_a.unify(&type_b).is_err());
}

#[test]
fn test_cannot_unify_wrong_fields() {
    /// assert `unify ( type[field_a], type[field_b] )` is error
    let mut type_a = cupid_builder::build! {
        "type" () = [
            "field_a" : int_ident()
        ]
    };
    let type_b = cupid_builder::build! {
        "type" () = [
            "field_b" : int_ident()
        ]
    };
    assert!(type_a.unify(&type_b).is_err());
}

#[test]
fn test_cannot_unify_not_generic() {
    /// assert `unify (int, int (alias for string))` is error
    let mut int = int_ident();
    let alias = IsTyped(
        cupid_builder::ident!("int"),
        string()
    );
    assert!(int.unify(&alias).is_err());
}

#[test]
fn test_unify_untyped_ident() {
    /// assert `unify (<t>, int)` is ok
    let mut generic = Untyped(cupid_builder::ident!("t"));
    assert!(generic.unify(&int_ident()).is_ok());
    assert_eq!(generic, int_ident())
}

#[test]
fn test_unify_ident_no_effect() {
    /// assert `unify (int, <t>)` is ok
    let mut ident = int_ident();
    let generic = Untyped(cupid_builder::ident!("t"));
    assert!(ident.unify(&generic).is_ok());
    assert_eq!(ident, int_ident());
}