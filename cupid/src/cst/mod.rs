use crate::{
    arena::{Arena, Entry, EntryId, UseArena},
    for_expr_variant,
    token::Token,
};

use self::expr::ExprSource;

pub mod array;
pub mod binop;
pub mod block;
pub mod r#break;
pub mod call;
pub mod class;
pub mod constant;
pub mod define;
pub mod expr;
pub mod fun;
pub mod get;
pub mod get_property;
pub mod get_super;
pub mod r#if;
pub mod invoke;
pub mod invoke_super;
pub mod r#loop;
pub mod method;
pub mod r#return;
pub mod set;
pub mod set_property;
pub mod unop;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct SourceId(pub EntryId);

impl From<EntryId> for SourceId {
    fn from(value: EntryId) -> Self {
        Self(value)
    }
}

impl From<SourceId> for EntryId {
    fn from(value: SourceId) -> Self {
        value.0
    }
}

pub trait HasToken<'src> {
    fn has_token(&self, token: Token<'src>) -> bool;
}

impl<'src> HasToken<'src> for ExprSource<'src> {
    fn has_token(&self, token: Token<'src>) -> bool {
        for_expr_variant!(self => |inner| inner.has_token(token))
    }
}

pub struct SourceArena<'src> {
    pub arena: Arena<ExprSource<'src>>,
}

impl<'src> Default for SourceArena<'src> {
    fn default() -> Self {
        Self {
            arena: Arena { entries: vec![] },
        }
    }
}

impl<'src> SourceArena<'src> {
    pub fn find(&self, token: Token<'src>) -> Option<&ExprSource<'src>> {
        self.arena.entries.iter().find_map(|entry| match &entry.value {
            Some(value) if value.has_token(token) => Some(value),
            _ => None,
        })
    }
}

impl<'src> UseArena<ExprSource<'src>> for SourceArena<'src> {
    fn insert(&mut self, value: ExprSource<'src>) -> EntryId {
        self.arena.insert(value)
    }
    fn get_entry(&self, id: EntryId) -> Option<&Entry<ExprSource<'src>>> {
        self.arena.get_entry(id)
    }
    fn get_entry_mut(&mut self, id: EntryId) -> Option<&mut Entry<ExprSource<'src>>> {
        self.arena.get_entry_mut(id)
    }
}
