use crate::{
    Address,
    database::selector::{
        FilterFn,
        Selector
    },
};

#[allow(unused_variables)]
pub trait QueryTable<Q, Row> where Row: Selector<Row>, Address: Selector<Row> {
    fn insert<R: Into<Row>>(&mut self, query: R) -> Address;
    fn read<'table: 'q, 'q, Col: Selector<Row>>(&'table self, query: &'q Q) -> Option<&'table Col> where Row: 'table;
    fn write(&mut self, query: Q) -> Option<()>;
    fn take<'table: 'q, 'q, Col: Selector<Row> + Default>(&'table mut self, query: &'q Q) -> Option<Col> { None }
}

pub trait TableRow<'row: 'q, 'q, Q> {
    fn matches_query(&'row self, query: &'q Q) -> bool;
    fn filter_by(&'row self, selector: &FilterFn<Self>) -> Option<&'row Self>;
    fn select_by<Col: Selector<Self> + PartialEq>(&'row self, selector: Option<&'q Col>) -> Option<&'row Self>;
    fn unify(&'row mut self, query: Q);
}
