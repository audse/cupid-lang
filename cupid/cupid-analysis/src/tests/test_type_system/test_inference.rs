#![allow(unused)]
#[cfg(test)]
use cupid_scope::*;
use cupid_ast::*;
use crate::*;

fn scope(symbols: Vec<Type>) -> Env {
    let mut env = Env::default();
    for symbol in symbols.into_iter() {
        let (ident, value) = type_symbol(symbol);
        env.set_symbol(&ident, value);
    }
    env
}

fn type_symbol(t: Type) -> (Ident, SymbolValue) {
    (t.to_ident(), SymbolValue {
        value: Some(VType(t)),
        type_hint: Type::type_ty().into_ident(),
        mutable: false,
    })
}

fn int_type() -> Type { cupid_builder::primitive!("int") }
fn bool_type() -> Type { cupid_builder::primitive!("bool") }

fn int(i: i32) -> Value { VInteger(i, Attributes::default()) }
fn bool(b: bool) -> Value { VBoolean(b, Attributes::default()) }
fn array(vals: Vec<Value>) -> Value { VArray(vals, Attributes::default()) }
fn tuple(vals: Vec<Value>) -> Value { VTuple(vals, Attributes::default()) }

#[test]
fn test_infer_int() {
    let val = int(1);
    let mut scope = scope(vec![int_type()]);
    let inferred_type = val.infer(&mut scope).unwrap();
    assert_eq!(inferred_type, int_type());
}

#[test]
fn test_infer_array() {
    let val = array(vec![int(1), int(2), int(3)]);
    let mut scope = scope(vec![
        int_type(),
        cupid_builder::array!()
    ]);
    let inferred_type = val.infer(&mut scope).unwrap();
    let expected_type = cupid_builder::array!(int_type().into());
    assert_eq!(inferred_type, expected_type);
}

#[test]
fn test_infer_tuple() {
    let val = tuple(vec![int(1), bool(true)]);
    let mut scope = scope(vec![
        int_type(),
        bool_type(),
        cupid_builder::primitive!("tuple"),
    ]);
    let inferred_type = val.infer(&mut scope).unwrap();
    let expected_type = cupid_builder::build! {
        "tuple" (int_type().into(), bool_type().into()) = [
            "": int_type().into(),
            "": bool_type().into()
        ]
    };
    assert_eq!(inferred_type, expected_type);
}