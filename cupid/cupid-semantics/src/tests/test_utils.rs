#![cfg(test)]
use cupid_ast::{stmt::{decl::Decl, type_def::TypeDef}, types::{typ::Type, traits::Trait}, expr::{function::Function, ident::Ident, value::{Value, Val}, Expr}, attr::Attr};
use cupid_env::{environment::Env, database::{table::QueryTable, symbol_table::query::Query, source_table::query::Query as SourceQuery}};
use cupid_util::Bx;

use crate::Address;

pub(super) fn env() -> Env {
    Env::default()
}

pub(super) fn int(i: i32) -> Value {
    Value {
        inner: Val::VInteger(i),
        ..Value::default()
    }
}

pub(super) fn id(ident: &'static str) -> Ident {
    ident.into()
}

// pub(super) fn namespaced_id(ident: &'static str, namespace: &'static str) -> Ident {
//     Ident { name: ident.into(), namespace: Some(Box::new(namespace.into())), ..Ident::default() }
// }

pub(super) fn decl(ident: &'static str) -> Decl {
    Decl {
        ident: ident.into(),
        ..Decl::default()
    }
}

pub(super) fn decl_val<V>(ident: &'static str, val: V) -> Decl where Expr: From<V> {
    Decl {
        ident: ident.into(),
        value: Expr::from(val).bx(),
        ..Decl::default()
    }
}

pub(super) fn typed_decl(ident: &'static str, typ: &'static str) -> Decl {
    Decl {
        ident: ident.into(),
        type_annotation: Some(typ.into()),
        ..Decl::default()
    }
}

pub(super) fn typ(ident: &'static str) -> Type {
    Type {
        ident: ident.into(),
        ..Type::default()
    }
}

pub(super) fn traits(ident: &'static str) -> Trait {
    Trait {
        ident: ident.into(),
        ..Trait::default()
    }
}

pub(super) fn fun<P: Into<Vec<&'static str>>>(params: P) -> Function {
    Function {
        params: params.into().iter().map(|p| decl(p)).collect(),
        ..Function::default()
    }
}

pub(super) fn type_def(ident: &'static str) -> TypeDef {
    TypeDef { ident: ident.into(), value: typ(ident), ..TypeDef::default() }
}

pub(super) fn read_ident(ident: &'static str, env: &mut Env) -> Option<Address> {
    env.database.symbol_table.read::<Address>(&Query::select(Ident::from(ident))).map(|a| *a)
}

pub(super) fn write_source(env: &mut Env) -> Address {
    env.database.source_table.insert(SourceQuery::insert())
}

pub(super) fn read_type(source: Address, env: &mut Env) -> Option<&Ident> {
    env.database.source_table.read::<Ident>(&SourceQuery::select(source))
}

pub(super) fn type_is(attr: Attr, typ: &'static str, env: &mut Env) -> bool {
    read_type(attr.source, env) == Some(&id(typ))
}