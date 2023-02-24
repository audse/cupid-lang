use crate::environment::Env;
use cupid_ast::{
    attr::GetAttr,
    expr::{ident::Ident, Expr},
    stmt::{allocate::Allocation, decl::Mut},
    types::{traits::Trait, typ::Type},
};
use cupid_debug::code::ErrorCode;
use derive_more::{From, Into, TryInto, Unwrap};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

/// The idea here is that each AST node is a closure that can access its parent context.

pub type Arena = BTreeMap<Ident, Rc<Value>>;
pub type Decorations = BTreeMap<DecorationType, Decoration>;
pub type Closure = Rc<RefCell<ExprClosure>>;

#[derive(Debug, Default, Clone)]
pub struct ExprClosure {
    pub parent: Option<Closure>,
    pub namespaces: Vec<Closure>,
    pub symbols: Arena,
    pub decorations: Decorations,
}

impl ExprClosure {
    pub fn move_to_arena(&mut self, env: &mut Env) {
        env.arena.append(&mut self.symbols);
    }
    pub fn insert<V: Into<Value>>(&mut self, key: Ident, value: V) -> Result<(), ErrorCode> {
        let mut v = value.into();
        v.decorations
            .insert(DecorationType::Source, Source(key.attr.source).into());
        self.symbols.insert(key, Rc::new(v)); // allows variable shadowing
        Ok(())
    }
    pub fn lookup(&self, key: &Ident) -> Option<Rc<Value>> {
        if let Some(value) = self.symbols.get(key) {
            Some(value.clone())
        } else if let Some(parent) = self.parent.as_ref() {
            parent.borrow().lookup(key)
        } else {
            for namespace in self.namespaces.iter() {
                if let Some(value) = namespace.borrow().lookup(key) {
                    return Some(value);
                }
            }
            None
        }
    }
    pub fn has_symbol(&self, key: &Ident) -> bool {
        if self.symbols.get(&key).is_some() {
            true
        } else if let Some(parent) = self.parent.as_ref() {
            parent.borrow().has_symbol(key)
        } else {
            for namespace in self.namespaces.iter() {
                if namespace.borrow().has_symbol(key) {
                    return true;
                }
            }
            false
        }
    }
    pub fn decorate_closure<D: Into<Decoration>>(&mut self, decoration: D) {
        let d = decoration.into();
        self.decorations.insert((&d).into(), d);
    }
    pub fn decorate<D: Into<Decoration>>(
        &mut self,
        key: &Ident,
        decoration: D,
    ) -> Option<Rc<Value>> {
        let d = decoration.into();
        if let Some(value) = self.symbols.get_mut(key) {
            Rc::get_mut(value)?.decorations.insert((&d).into(), d);
            Some(value.clone())
        } else if let Some(parent) = self.parent.as_mut() {
            parent.borrow_mut().decorate(key, d)
        } else {
            for namespace in self.namespaces.iter_mut() {
                if namespace.borrow().has_symbol(key) {
                    return namespace.borrow_mut().decorate(key, d);
                }
            }
            None
        }
    }
    pub fn reference(&mut self, key: &Ident) -> Option<Rc<Value>> {
        if let Some(value) = self.symbols.get_mut(key) {
            Rc::get_mut(value)?.reference();
            Some(value.clone())
        } else if let Some(parent) = self.parent.as_mut() {
            parent.borrow_mut().reference(key)
        } else {
            for namespace in self.namespaces.iter_mut() {
                if let Some(value) = namespace.borrow_mut().reference(key) {
                    return Some(value.clone());
                }
            }
            None
        }
    }
    pub fn get_type(&self) -> Option<Rc<Type>> {
        self.decorations
            .get(&DecorationType::Type)
            .map(|t| {
                if let Decoration::Type(t) = t {
                    Some(t.clone())
                } else {
                    None
                }
            })
            .flatten()
    }
}

#[derive(Debug, Clone, Copy, From, Into)]
pub struct Source(usize);

#[derive(Debug, Clone, From, Unwrap, TryInto)]
#[try_into(ref, owned, ref_mut)]
pub enum Decoration {
    Value(Rc<RefCell<Expr>>),
    Mutable(Mut),
    Type(Rc<Type>),
    TypeValue(Rc<RefCell<Type>>),
    TraitValue(Rc<RefCell<Trait>>),
    Refs(usize),
    Source(Source),
}

type Trying<'a, T> = Result<&'a T, &'a str>;

impl Decoration {
    fn get_value(&self) -> Option<Rc<RefCell<Expr>>> {
        (self.try_into() as Trying<Rc<RefCell<Expr>>>).ok().cloned()
    }
    fn get_type(&self) -> Option<Rc<Type>> {
        (self.try_into() as Trying<Rc<Type>>).ok().cloned()
    }
    fn get_type_value(&self) -> Option<Rc<RefCell<Type>>> {
        (self.try_into() as Trying<Rc<RefCell<Type>>>).ok().cloned()
    }
    fn get_trait_value(&self) -> Option<Rc<RefCell<Trait>>> {
        (self.try_into() as Trying<Rc<RefCell<Trait>>>)
            .ok()
            .cloned()
    }
    fn get_refs(&self) -> usize {
        (self.try_into() as Trying<usize>)
            .ok()
            .copied()
            .unwrap_or_default()
    }
    fn get_source(&self) -> Option<usize> {
        (self.try_into() as Trying<Source>)
            .ok()
            .copied()
            .map(|s| s.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DecorationType {
    Value,
    Mutable,
    Type,
    TypeValue,
    TraitValue,
    Refs,
    Source,
}

impl From<&Decoration> for DecorationType {
    fn from(d: &Decoration) -> Self {
        match d {
            Decoration::Mutable(_) => Self::Mutable,
            Decoration::TraitValue(_) => Self::TraitValue,
            Decoration::Type(_) => Self::Type,
            Decoration::TypeValue(_) => Self::TypeValue,
            Decoration::Value(_) => Self::Value,
            Decoration::Refs(_) => Self::Refs,
            Decoration::Source(_) => Self::Source,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Value {
    pub decorations: Decorations,
}

impl Value {
    pub fn build() -> ValueBuilder {
        ValueBuilder::default()
    }
    pub fn reference(&mut self) {
        let refs = self.get_refs();
        self.decorations
            .insert(DecorationType::Refs, (refs + 1).into());
    }
    pub fn get_value(&self) -> Option<Rc<RefCell<Expr>>> {
        self.decorations
            .get(&DecorationType::Value)
            .map(|d| d.get_value())
            .flatten()
    }
    pub fn get_type(&self) -> Option<Rc<Type>> {
        self.decorations
            .get(&DecorationType::Type)
            .map(|d| d.get_type())
            .flatten()
    }
    pub fn get_refs(&self) -> usize {
        self.decorations
            .get(&DecorationType::Refs)
            .map(|d| d.get_refs())
            .unwrap_or_default()
    }
    pub fn is_mutable(&self) -> bool {
        if let Some(decoration) = self.decorations.get(&DecorationType::Mutable) {
            matches!(decoration, Decoration::Mutable(Mut::Mutable))
        } else {
            false
        }
    }
    pub fn get_trait_value(&self) -> Option<Rc<RefCell<Trait>>> {
        self.decorations
            .get(&DecorationType::TraitValue)
            .map(|d| d.get_trait_value())
            .flatten()
    }
    pub fn get_type_value(&self) -> Option<Rc<RefCell<Type>>> {
        self.decorations
            .get(&DecorationType::TypeValue)
            .map(|d| d.get_type_value())
            .flatten()
    }
    pub fn get_source(&self) -> Option<usize> {
        self.decorations
            .get(&DecorationType::Source)
            .map(|d| d.get_source())
            .flatten()
    }
}

#[derive(Debug, Default, Clone)]
pub struct ValueBuilder {
    pub decorations: Decorations,
}

impl ValueBuilder {
    pub fn value(self, inner: Allocation) -> Self {
        match inner.into() {
            Allocation::Expr(e) => self.expr(e),
            Allocation::Type(t) => self.type_def(t),
            Allocation::Trait(t) => self.trait_def(t),
        }
    }
    pub fn expr(mut self, expr: Rc<RefCell<Expr>>) -> Self {
        if !expr.borrow().is_empty() {
            self.decorations.insert(DecorationType::Source, Decoration::Source(Source(expr.borrow().attr().source)));
        }
        self.decorations
            .insert(DecorationType::Value, Decoration::Value(expr));
        self
    }
    pub fn type_def(mut self, typ: Rc<RefCell<Type>>) -> Self {
        self.decorations.insert(DecorationType::Source, Decoration::Source(Source(typ.borrow().attr().source)));
        self.decorations
            .insert(DecorationType::TypeValue, Decoration::TypeValue(typ));
        self
    }
    pub fn trait_def(mut self, trait_val: Rc<RefCell<Trait>>) -> Self {
        self.decorations.insert(DecorationType::Source, Decoration::Source(Source(trait_val.borrow().attr().source)));
        self.decorations.insert(
            DecorationType::TraitValue,
            Decoration::TraitValue(trait_val),
        );
        self
    }
    pub fn mutable(mut self, mutable: Mut) -> Self {
        self.decorations
            .insert(DecorationType::Mutable, Decoration::Mutable(mutable));
        self
    }
    pub fn build(self) -> Value {
        Value {
            decorations: self.decorations,
        }
    }
}

#[derive(Debug)]
pub struct FmtClosure<'a>(pub &'a Closure, pub i32);

impl std::fmt::Display for FmtClosure<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let closure = self.0.borrow();
        let depth = self.1;
        if depth > 0 {
            let parent = closure
                .parent
                .as_ref()
                .map(|parent| FmtClosure(parent, depth - 1));
            let namespaces = closure
                .namespaces
                .iter()
                .map(|n| FmtClosure(n, depth - 1))
                .collect::<Vec<FmtClosure>>();
            f.debug_struct("ExprClosure")
                .field("parent", &parent)
                .field("namespaces", &namespaces)
                .field("symbols", &closure.symbols)
                .finish()
        } else {
            f.debug_struct("ExprClosure")
                .field("parent", &"<out of depth>")
                .field("namespaces", &"<out of depth>")
                .field("symbols", &closure.symbols)
                .finish()
        }
    }
}
