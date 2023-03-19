use std::ops;

use crate::{
    ast::Expr,
    cst::{expr::ExprSource, SourceArena},
    ty::Type,
};

#[derive(Default)]
pub struct Arena<T> {
    pub entries: Vec<Entry<T>>,
}

impl Default for Arena<Expr<'_>> {
    fn default() -> Self {
        Self { entries: vec![] }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct EntryId(pub usize);

pub struct Entry<T> {
    pub id: EntryId,
    pub value: Option<T>,
}

impl ops::Deref for EntryId {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for EntryId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

pub trait UseArena<T> {
    fn insert(&mut self, value: T) -> EntryId;
    fn get_entry(&self, id: EntryId) -> Option<&Entry<T>>;
    fn get_entry_mut(&mut self, id: EntryId) -> Option<&mut Entry<T>>;
    fn get(&self, id: impl Into<EntryId>) -> Option<&T> {
        self.get_entry(id.into()).map(|e| e.value.as_ref()).flatten()
    }
    fn get_mut(&mut self, id: impl Into<EntryId>) -> Option<&mut T> {
        self.get_entry_mut(id.into()).map(|e| e.value.as_mut()).flatten()
    }
    fn expect_entry(&self, id: impl Into<EntryId>) -> &Entry<T> {
        self.get_entry(id.into()).unwrap()
    }
    fn expect_entry_mut(&mut self, id: impl Into<EntryId>) -> &mut Entry<T> {
        self.get_entry_mut(id.into()).unwrap()
    }
    fn expect(&self, id: impl Into<EntryId>) -> &T {
        self.get(id).unwrap()
    }
    fn expect_mut(&mut self, id: impl Into<EntryId>) -> &mut T {
        self.get_mut(id).unwrap()
    }
    fn take(&mut self, id: impl Into<EntryId>) -> T {
        self.get_entry_mut(id.into()).unwrap().value.take().unwrap()
    }
    fn replace(&mut self, id: impl Into<EntryId>, val: T) {
        self.get_entry_mut(id.into()).unwrap().value = Some(val);
    }
}

impl<T> UseArena<T> for Arena<T> {
    fn insert(&mut self, value: T) -> EntryId {
        let id = self.entries.len().into();
        self.entries.push(Entry {
            id,
            value: Some(value),
        });
        id
    }
    fn get_entry(&self, id: EntryId) -> Option<&Entry<T>> {
        self.entries.get(*id)
    }
    fn get_entry_mut(&mut self, id: EntryId) -> Option<&mut Entry<T>> {
        self.entries.get_mut(*id)
    }
}

pub struct ExprArena<'src> {
    pub expr: Arena<Expr<'src>>,
    pub ty: Arena<Type<'src>>,
    pub source: SourceArena<'src>,
}

impl<'src> Default for ExprArena<'src> {
    fn default() -> Self {
        Self {
            expr: Arena::default(),
            ty: Arena::default(),
            source: SourceArena::default(),
        }
    }
}

impl<'src> ExprArena<'src> {
    pub fn expect_expr(&self, id: impl Into<EntryId>) -> &Expr<'src> {
        UseArena::<Expr>::expect(self, id)
    }
    pub fn expect_source(&self, id: impl Into<EntryId>) -> &ExprSource<'src> {
        UseArena::<ExprSource>::expect(self, id)
    }
    pub fn take_source(&mut self, id: impl Into<EntryId>) -> ExprSource<'src> {
        UseArena::<ExprSource>::take(self, id)
    }
    pub fn replace_source(&mut self, id: impl Into<EntryId>, value: ExprSource<'src>) {
        UseArena::<ExprSource>::replace(self, id, value)
    }
    pub fn expect_ty(&self, id: impl Into<EntryId>) -> &Type<'src> {
        UseArena::<Type>::expect(self, id)
    }
    pub fn expect_expr_mut(&mut self, id: impl Into<EntryId>) -> &mut Expr<'src> {
        UseArena::<Expr>::expect_mut(self, id)
    }
    pub fn expect_ty_mut(&mut self, id: impl Into<EntryId>) -> &mut Type<'src> {
        UseArena::<Type>::expect_mut(self, id)
    }
    pub fn expect_source_mut(&mut self, id: impl Into<EntryId>) -> &mut ExprSource<'src> {
        UseArena::<ExprSource>::expect_mut(self, id)
    }
}

impl<'src> UseArena<Expr<'src>> for ExprArena<'src> {
    fn insert(&mut self, value: Expr<'src>) -> EntryId {
        self.expr.insert(value)
    }
    fn get_entry(&self, id: EntryId) -> Option<&Entry<Expr<'src>>> {
        self.expr.get_entry(id)
    }
    fn get_entry_mut(&mut self, id: EntryId) -> Option<&mut Entry<Expr<'src>>> {
        self.expr.get_entry_mut(id)
    }
}

impl<'src> UseArena<Type<'src>> for ExprArena<'src> {
    fn insert(&mut self, value: Type<'src>) -> EntryId {
        self.ty.insert(value)
    }
    fn get_entry(&self, id: EntryId) -> Option<&Entry<Type<'src>>> {
        self.ty.get_entry(id)
    }
    fn get_entry_mut(&mut self, id: EntryId) -> Option<&mut Entry<Type<'src>>> {
        self.ty.get_entry_mut(id)
    }
}

impl<'src> UseArena<ExprSource<'src>> for ExprArena<'src> {
    fn insert(&mut self, value: ExprSource<'src>) -> EntryId {
        self.source.insert(value)
    }
    fn get_entry(&self, id: EntryId) -> Option<&Entry<ExprSource<'src>>> {
        self.source.get_entry(id)
    }
    fn get_entry_mut(&mut self, id: EntryId) -> Option<&mut Entry<ExprSource<'src>>> {
        self.source.get_entry_mut(id)
    }
}
