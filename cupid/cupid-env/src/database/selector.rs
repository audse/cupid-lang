use std::rc::Rc;

use cupid_ast::expr::{Expr, ident::Ident};
use cupid_debug::source::ExprSource;
use crate::{
    Address,
    database::{
        source_table::row::SourceRow,
        symbol_table::row::SymbolRow,
    },
};

use super::symbol_table::row::Ref;

pub trait Selector<Row: ?Sized>: Clone {
    fn select(from: &Row) -> &Self;
    fn select_mut(from: &mut Row) -> &mut Self;
}

#[derive(Default, Clone)]
pub struct FilterFn<'q, Row: ?Sized>(pub Option<&'q dyn Fn(&Row) -> bool>);
impl<'q, Row: std::fmt::Debug> std::fmt::Debug for FilterFn<'q, Row> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "filter function") }
}

macro_rules! symbol_row_selector {
    ( $( $col:ty => |$row:ident| $field:expr ; )* ) => {
        $( impl Selector<SymbolRow> for $col {
            fn select($row: &SymbolRow) -> &Self { & $field }
            fn select_mut($row: &mut SymbolRow) -> &mut Self { &mut $field }
        } )*
    }
}

macro_rules! source_row_selector {
    ( $( $col:ty => |$row:ident| $field:expr ; )* ) => {
        $( impl Selector<SourceRow> for $col {
            fn select($row: &SourceRow) -> &Self { & $field }
            fn select_mut($row: &mut SourceRow) -> &mut Self { &mut $field }
        } )*
    }
}

symbol_row_selector! {
    Address => |row| row.address;
    Ident => |row| row.ident;
    Expr => |row| row.expr;
    Ref => |row| row.refs;
}

impl Selector<SymbolRow> for SymbolRow {
    fn select(from: &SymbolRow) -> &Self {
        from
    }
    fn select_mut(from: &mut SymbolRow) -> &mut Self {
        from
    }
}


source_row_selector! {
    Address => |row| row.address;
    Ident => |row| row.typ;
    Rc<ExprSource> => |row| row.source;
}

impl Selector<SourceRow> for SourceRow {
    fn select(from: &SourceRow) -> &Self {
        from
    }
    fn select_mut(from: &mut SourceRow) -> &mut Self {
        from
    }
}