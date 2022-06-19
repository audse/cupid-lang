use cupid_util::build_struct;
use super::query::Query;
use crate::{Address, Ident, Mut, env::SymbolType, pre_analysis, package_resolution, type_scope_analysis, type_name_resolution, scope_analysis, name_resolution, type_inference, type_checking, flow_checking, linting};

#[derive(Debug, Default, Clone)]
pub struct Database {
    rows: Vec<Row>,
}

impl Database {

    pub fn insert<V: Default>(&mut self, query: Query<V>) -> Address 
        where RowExpr: WriteRowExpr<V> 
    {
        let mut row: Row = query.into();
        row.address = self.rows.len();
        self.rows.push(row);
        self.rows.len() - 1
    }

    pub fn read<Col: Selector>(&self, query: &Query<()>) -> Option<&Col> { 
        let row = self.rows.iter().find(|row| row.matches_query(query))?;
        Some(Col::select(row))
    }

    pub fn write<V: Default>(&mut self, query: Query<V>) -> Option<()> 
        where RowExpr: WriteRowExpr<V> 
    {
        let row = self.rows.iter_mut().find(|row| row.matches_query(&query))?;
        row.unify(query);
        Some(())
    }

    pub fn take<Col: Selector + Default>(&mut self, query: &Query<()>) -> Option<Col> {
        if let Some(row) = self.rows.iter_mut().find(|row| row.matches_query(&query)) {
            let col = Col::select_mut(row);
            Some(std::mem::take(col))
        } else {
            None
        }
    }
}

build_struct! {
    #[derive(Debug, Default, Clone)]
    pub RowBuilder => pub Row {
        pub address: Address,
        pub ident: Ident,
        pub mutable: Mut,
        pub source: (), // TODO ParseNode
        pub typ: SymbolType,
        pub expr: RowExpr,
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl Row {
    fn matches_query<V: Default>(&self, query: &Query<V>) -> bool where RowExpr: WriteRowExpr<V> {
        let Query { address, ident, ident_ref, ..} = query;
        self.select_by(address.as_ref())
            .or_else(|| self.select_by(ident.as_ref()))
            .or_else(|| self.select_by(*ident_ref))
            .is_some()
    }
    fn select_by<Col: Selector + PartialEq>(&self, selector: Option<&Col>) -> Option<&Row> { 
        if let Some(selector) = selector {
            if Col::select(self) == selector { return Some(self) }
        }
        None
    }
    pub(super) fn unify<V: Default>(&mut self, query: Query<V>) where RowExpr: WriteRowExpr<V> {
        let Query { address, expr, ident, mutable, source, typ, ..} = query;
        if let Some(address) = address { self.address = address; }
        if let Some(expr) = expr { self.expr.write(expr.into()); }
        if let Some(ident) = ident { self.ident = ident; }
        if let Some(mutable) = mutable { self.mutable = mutable; }
        // if let Some(source) = source { self.source = source; }
        if let Some(typ) = typ { self.typ = typ; }
    }
}

impl<V: Default> From<Query<'_, V>> for Row where RowExpr: WriteRowExpr<V> {
    fn from(q: Query<V>) -> Self {
        let Query { address, expr, ident, mutable, source, typ, ..} = q;
        let mut row_expr = RowExpr::default();
        if let Some(expr) = expr { row_expr.write(expr); }
        Row {
            address: address.unwrap_or_default(),
            expr: row_expr,
            ident: ident.unwrap_or_default(),
            mutable: mutable.unwrap_or_default(),
            source,
            typ: typ.unwrap_or_default(),
        }
    }
}

pub trait Selector: Clone {
    fn select(from: &Row) -> &Self;
    fn select_mut(from: &mut Row) -> &mut Self;
}

impl Selector for Address {
    fn select(from: &Row) -> &Self { &from.address }
    fn select_mut(from: &mut Row) -> &mut Self { &mut from.address }
}

impl Selector for Ident {
    fn select(from: &Row) -> &Self { &from.ident }
    fn select_mut(from: &mut Row) -> &mut Self { &mut from.ident }
}

impl Selector for Row {
    fn select(from: &Row) -> &Self { from }
    fn select_mut(from: &mut Row) -> &mut Self { from }
}

impl Selector for RowExpr {
    fn select(from: &Row) -> &Self { &from.expr }
    fn select_mut(from: &mut Row) -> &mut Self { &mut from.expr }
}

impl Selector for SymbolType {
    fn select(from: &Row) -> &Self { &from.typ }
    fn select_mut(from: &mut Row) -> &mut Self { &mut from.typ }
}

#[derive(Debug, Default, Clone)]
pub struct RowExpr {
    pub pre_analysis: Option<pre_analysis::Expr>,
    pub package_resolution: Option<package_resolution::Expr>,
    pub type_scope_analysis: Option<type_scope_analysis::Expr>,
    pub type_name_resolution: Option<type_name_resolution::Expr>,
    pub scope_analysis: Option<scope_analysis::Expr>,
    pub name_resolution: Option<name_resolution::Expr>,
    pub type_inference: Option<type_inference::Expr>,
    pub type_checking: Option<type_checking::Expr>,
    pub flow_checking: Option<flow_checking::Expr>,
    pub linting: Option<linting::Expr>,
}

pub trait WriteRowExpr<T> {
    fn write(&mut self, expr: T);
}

impl WriteRowExpr<RowExpr> for RowExpr {
    fn write(&mut self, expr: RowExpr) {
        *self = expr;
    }
}

macro_rules! impl_write_row_expr {
    ( $($pass:ident;)* ) => {
        $( 
            impl WriteRowExpr<$pass::Expr> for RowExpr {
                fn write(&mut self, expr: $pass::Expr) {
                    self.$pass = Some(expr);
                }
            }
            // also implements a way to read the selected expression's value
            impl Selector for Option<$pass::Expr> {
                fn select(from: &Row) -> &Self {
                    &from.expr.$pass
                }
                fn select_mut(from: &mut Row) -> &mut Self {
                    &mut from.expr.$pass
                }
            }
        )*
    }
}

impl_write_row_expr! {
    pre_analysis;
    package_resolution;
    type_scope_analysis;
    type_name_resolution;
    scope_analysis;
    name_resolution;
    type_inference;
    type_checking;
    flow_checking;
    linting;
}

impl WriteRowExpr<()> for RowExpr {
    fn write(&mut self, expr: ()) {}
}