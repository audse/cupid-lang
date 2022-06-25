#![cfg(test)]
use cupid_ast::{stmt::{decl::Decl, type_def::TypeDef}, types::{typ::Type, traits::Trait}, expr::{function::Function, ident::Ident}};
use cupid_env::{environment::Env, database::{table::QueryTable, symbol_table::query::Query}};

use crate::Address;

pub(super) fn env() -> Env {
    Env::default()
}

pub(super) fn id(ident: &'static str) -> Ident {
    ident.into()
}

pub(super) fn namespaced_id(ident: &'static str, namespace: &'static str) -> Ident {
    Ident { name: ident.into(), namespace: Some(Box::new(namespace.into())), ..Ident::default() }
}

pub(super) fn decl(ident: &'static str) -> Decl {
    Decl {
        ident: ident.into(),
        ..Decl::default()
    }
}

pub(super) fn fun_decl(ident: &'static str, val: Function) -> Decl {
    Decl {
        ident: ident.into(),
        value: val.into(),
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

pub(super) fn get_ident(ident: &'static str, env: &mut Env) -> Option<Address> {
    env.database
        .symbol_table
        .read::<Address>(&Query::select(Ident::from(ident)))
        .map(|a| *a)
}