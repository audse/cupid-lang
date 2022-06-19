use crate::{Address, Ident, Mut, env::SymbolType, Query};

#[derive(Debug, Default, Clone)]
pub struct Row {
    pub address: Address,
    pub ident: Ident,
    pub mutable: Mut,
    pub source: (), // TODO ParseNode
    pub typ: SymbolType,
    pub expr: RowExpr,
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl Row {
    pub(super) fn matches_query<V: Default>(&self, query: &Query<V>) -> bool where RowExpr: WriteRowExpr<V> {
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
        let Query { address, expr, ident, ident_ref, mutable, source, typ, ..} = query;

        address.map(|a| self.address = a);
        expr.map(|e| self.expr.write(e.into()));
        ident.map(|i| self.ident = i);
        ident_ref.map(|i| self.ident = i.to_owned());
        mutable.map(|m| self.mutable = m);
        typ.map(|t| self.typ = t);
        // if let Some(source) = source { self.source = source; } // TODO
    }
}

impl<V: Default> From<Query<'_, V>> for Row where RowExpr: WriteRowExpr<V> {
    fn from(q: Query<V>) -> Self {
        let Query { address, expr, ident, mutable, source, typ, ..} = q;
        
        let mut row_expr = RowExpr::default();
        expr.map(|e| row_expr.write(e));

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
    pub pre_analysis: Option<crate::pre_analysis::Expr>,
    pub package_resolution: Option<crate::package_resolution::Expr>,
    pub type_scope_analysis: Option<crate::type_scope_analysis::Expr>,
    pub type_name_resolution: Option<crate::type_name_resolution::Expr>,
    pub scope_analysis: Option<crate::scope_analysis::Expr>,
    pub name_resolution: Option<crate::name_resolution::Expr>,
    pub type_inference: Option<crate::type_inference::Expr>,
    pub type_checking: Option<crate::type_checking::Expr>,
    pub flow_checking: Option<crate::flow_checking::Expr>,
    pub linting: Option<crate::linting::Expr>,
}

pub trait WriteRowExpr<T> {
    fn write(&mut self, expr: T);
}

pub trait TakeRowExpr<Expr> {
    fn take(&mut self) -> Option<Expr>;
}

impl WriteRowExpr<RowExpr> for RowExpr {
    fn write(&mut self, expr: RowExpr) {
        *self = expr;
    }
}

impl TakeRowExpr<RowExpr> for RowExpr {
    fn take(&mut self) -> Option<RowExpr> {
        Some(std::mem::take(self))
    }
}

macro_rules! impl_row_expr {
    ( $($pass:ident;)* ) => {
        $( 
            impl WriteRowExpr<crate::$pass::Expr> for RowExpr {
                fn write(&mut self, expr: crate::$pass::Expr) {
                    self.$pass = Some(expr);
                }
            }
            impl TakeRowExpr<crate::$pass::Expr> for RowExpr {
                fn take(&mut self) -> Option<crate::$pass::Expr> {
                    std::mem::take(&mut self.$pass)
                }
            }
            // also implements a way to read the selected expression's value
            impl Selector for Option<crate::$pass::Expr> {
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

impl_row_expr! {
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