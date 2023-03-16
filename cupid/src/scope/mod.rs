use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::{pointer::Pointer, ty::Type};

use self::symbol::{Class, ClassId, Symbol, SymbolValue};

pub mod symbol;

#[derive(Clone, Default)]
pub struct Scope<'src> {
    pub context: ScopeContext,
    pub parent: Option<Pointer<Scope<'src>>>,
    pub symbols: HashMap<&'src str, Pointer<Symbol<'src>>>,
    pub classes: HashMap<&'src str, Class<'src>>,
    pub depth: usize,
}

impl<'src> Scope<'src> {
    pub fn new(context: ScopeContext, parent: Option<Pointer<Scope<'src>>>) -> Self {
        let depth = parent.as_ref().map(|parent| parent.0.borrow().depth + 1).unwrap_or_default();
        Self {
            context,
            parent,
            depth,
            ..Default::default()
        }
    }

    pub fn lookup(&self, name: &str) -> Option<Pointer<Symbol<'src>>> {
        match self.lookup_current(name) {
            Some(symbol) => Some(symbol),
            None => match &self.parent {
                Some(parent) => parent.borrow().lookup(name),
                None => None,
            },
        }
    }

    pub fn lookup_current(&self, name: &str) -> Option<Pointer<Symbol<'src>>> {
        self.symbols.get(name).cloned()
    }

    pub fn define(&mut self, name: &'src str) {
        self.symbols.insert(
            name,
            Pointer::new(Symbol {
                ty: Type::Unknown,
                ..Default::default()
            }),
        );
    }

    pub fn annotate_ty(&mut self, name: &'src str, ty: Type<'src>) {
        self.symbols.entry(name).and_modify(|sym| sym.borrow_mut().ty = ty);
    }

    pub fn annotate_class(&mut self, name: &'src str) {
        self.symbols
            .entry(name)
            .and_modify(|sym| sym.borrow_mut().value = SymbolValue::Class(ClassId(name)));
    }

    pub fn insert_class(&mut self, name: &'src str, class_scope: Pointer<Scope<'src>>) {
        self.classes.insert(name, Class { scope: class_scope });
        self.annotate_class(name)
    }

    pub fn initialize(&mut self) {
        self.define("log");
        self.annotate_ty("log", Type::Function);

        self.define("panic");
        self.annotate_ty("panic", Type::Function);

        self.define("push");
        self.annotate_ty("push", Type::Function);

        self.define("pop");
        self.annotate_ty("pop", Type::Function);

        self.define("len");
        self.annotate_ty("len", Type::Function);

        self.define("get");
        self.annotate_ty("get", Type::Function);
    }
}

#[derive(Debug, Clone, Default)]
pub enum ScopeContext {
    Block,
    Class,
    Fun,
    Loop,
    #[default]
    Global,
}

impl<'src> Pointer<Scope<'src>> {
    pub fn global() -> Self {
        Pointer::<Scope<'src>>::new(Scope::default())
    }

    pub fn subscope(&self, context: ScopeContext) -> Self {
        let scope = Scope::new(context, Some(self.clone()));
        Self(Rc::new(RefCell::new(scope)))
    }

    pub fn parent(&self) -> Option<Pointer<Scope<'src>>> {
        self.borrow().parent.as_ref().cloned()
    }
}

impl fmt::Debug for Scope<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ... }}")
    }
}

impl fmt::Display for Scope<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Scope")
            .field("depth", &self.depth)
            .field("context", &self.context)
            .field("symbols", &self.symbols.len())
            .field("parent", &self.parent)
            .finish()
    }
}
