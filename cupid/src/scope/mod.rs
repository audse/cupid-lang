use std::{collections::HashMap, fmt};

use crate::{
    arena::{EntryId, ExprArena, UseArena},
    error::CupidError,
    pointer::Pointer,
    token::Token,
    ty::Type,
};

use self::symbol::{ClassId, ClassTable, Symbol, SymbolValue};

pub mod symbol;

#[derive(Clone, Default)]
pub struct Scope<'src> {
    pub context: ScopeContext,
    pub parent: Option<Pointer<Scope<'src>>>,
    pub symbols: HashMap<&'src str, Pointer<Symbol<'src>>>,
    pub classes: HashMap<ClassId<'src>, ClassTable<'src>>,
    pub depth: usize,
}

pub trait Lookup<'src, K, V> {
    fn lookup(&self, key: K) -> Option<V>;
    fn lookup_current(&self, key: K) -> Option<V>;
}

impl<'src> Lookup<'src, &'src str, Pointer<Symbol<'src>>> for Scope<'src> {
    fn lookup(&self, key: &'src str) -> Option<Pointer<Symbol<'src>>> {
        match self.lookup_current(key) {
            Some(symbol) => Some(symbol),
            None => match &self.parent {
                Some(parent) => parent.borrow().lookup(key),
                None => None,
            },
        }
    }
    fn lookup_current(&self, key: &'src str) -> Option<Pointer<Symbol<'src>>> {
        self.symbols.get(key).cloned()
    }
}

impl<'src> Lookup<'src, ClassId<'src>, ClassTable<'src>> for Scope<'src> {
    fn lookup(&self, key: ClassId<'src>) -> Option<ClassTable<'src>> {
        match self.lookup_current(key) {
            Some(class) => Some(class),
            None => match &self.parent {
                Some(parent) => parent.borrow().lookup(key),
                None => None,
            },
        }
    }
    fn lookup_current(&self, key: ClassId<'src>) -> Option<ClassTable<'src>> {
        self.classes.get(&key).cloned()
    }
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

    pub fn lookup_property(
        &self,
        receiver_ty: Type<'src>,
        prop: Token<'src>,
    ) -> Result<Option<Pointer<Symbol<'src>>>, CupidError> {
        let class_table = match receiver_ty {
            Type::Class(class_name) => self.lookup(class_name),
            Type::Instance(instance_class_name) => self.lookup(instance_class_name),
            _ => {
                return Err(CupidError::type_error(
                    "Only classes have properties.",
                    prop.to_static(),
                ))
            }
        };
        match class_table {
            Some(class) => match class.scope.borrow().lookup(prop.lexeme) {
                Some(value) => Ok(Some(value)),
                None => Ok(None),
            },
            None => Err(CupidError::name_error("Undefined", prop.to_static())),
        }
        // match receiver_ty {
        //     Type::Class(class_name) => {
        //         let class = self.lookup(class_name);
        //         match class {
        //             Some(class) => match class.scope.borrow().lookup(prop.lexeme) {
        //                 Some(value) => Ok(Some(value)),
        //                 None => Ok(None),
        //             },
        //             None => Err(CupidError::name_error("Undefined", prop.to_static())),
        //         }
        //     }
        //     Type::Instance(class_name) => {
        //         let instance = self.lookup(class_name);
        //         match instance {
        //             Some(class) => match class.scope.borrow().lookup(prop.lexeme) {
        //                 Some(value) => Ok(Some(value)),
        //                 None => Ok(None),
        //             },
        //             None => Err(CupidError::name_error("Undefined", prop.to_static())),
        //         }
        //     }
        //     _ => Err(CupidError::type_error("Only classes have properties.", prop.to_static())),
        // }
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

    pub fn annotate_expr(&mut self, name: &'src str, expr: EntryId) {
        self.symbols
            .entry(name)
            .and_modify(|sym| sym.borrow_mut().value = SymbolValue::Expr(expr));
    }

    pub fn annotate_class(&mut self, name: &'src str) {
        self.symbols
            .entry(name)
            .and_modify(|sym| sym.borrow_mut().value = SymbolValue::Class(ClassId(name)));
    }

    pub fn insert_class(&mut self, name: &'src str, class_scope: Pointer<Scope<'src>>) {
        self.classes.insert(ClassId(name), ClassTable { scope: class_scope });
        self.annotate_class(name)
    }

    pub fn initialize(&mut self, arena: &mut ExprArena<'src>) {
        self.define("log");
        let nil_ty = arena.insert(Type::Nil);
        self.annotate_ty("log", Type::Function { returns: nil_ty });

        self.define("panic");
        self.annotate_ty("panic", Type::Function { returns: nil_ty });

        self.define("push");
        let int_ty = arena.insert(Type::Int);
        self.annotate_ty("push", Type::Function { returns: int_ty });

        self.define("pop");
        let unknown_ty = arena.insert(Type::Unknown);
        self.annotate_ty(
            "pop",
            Type::Function {
                returns: unknown_ty,
            },
        );

        self.define("len");
        self.annotate_ty("len", Type::Function { returns: int_ty });

        self.define("get");
        self.annotate_ty(
            "get",
            Type::Function {
                returns: unknown_ty,
            },
        );
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
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
        Self::new(Scope::default())
    }

    pub fn subscope(&self, context: ScopeContext) -> Self {
        let scope = Scope::new(context, Some(self.clone()));
        Self::new(scope)
    }

    pub fn parent(&self) -> Option<Pointer<Scope<'src>>> {
        self.borrow().parent.as_ref().cloned()
    }

    pub fn is_within_context(&self, context: ScopeContext) -> bool {
        if self.is_context(context) {
            return true;
        } else if let Some(parent) = self.borrow().parent.as_ref() {
            return parent.is_context(context);
        } else {
            false
        }
    }

    pub fn is_context(&self, context: ScopeContext) -> bool {
        self.borrow().context == context
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
