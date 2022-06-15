macro_rules! for_each_node {
    ($for:expr => $do:ident $args:tt) => {
        match $for {
            Expr::Block(x) => x.$do $args,
            Expr::Decl(x) => x.$do $args,
            Expr::Function(x) => x.$do $args,
            Expr::Ident(x) => x.$do $args,
            Expr::TypeDef(x) => x.$do $args,
            Expr::Value(x) => x.$do $args,
            _ => unreachable!()
        }
    }
}

/// Creates an `Expr` enum that contains the nodes for the current pass
macro_rules! define_pass_nodes {
    (
        Decl: $decl:item
		Function: $function:item
        Ident: $ident:item
        TypeDef: $type_def:item
    ) => {
        #[derive(Debug, Clone)]
        pub enum Expr {
            Block(crate::Block<Expr>),
            Decl(Decl),
			Function(Function),
            Ident(Ident),
            TypeDef(TypeDef),
			Value(crate::Value),
        }

        impl Default for Expr {
            fn default() -> Self {
                Self::Value(crate::Value::default())
            }
        }

        impl crate::AsNode for Expr {
            fn scope(&self) -> crate::Scope {
                crate::util::for_each_node!(self => scope())
            }
            fn source(&self) -> crate::Source {
                crate::util::for_each_node!(self => source())
            }
            fn typ(&self) -> crate::Address {
                crate::util::for_each_node!(self => typ())
            }
            fn set_source(&mut self, source: crate::Source) { 
                crate::util::for_each_node!(self => set_source(source));
            }
            fn set_scope(&mut self, scope: crate::Scope) {
                crate::util::for_each_node!(self => set_scope(scope));
            }
            fn set_typ(&mut self, typ: crate::Address) {
                crate::util::for_each_node!(self => set_typ(typ));
            }
        }

        $decl
		$function
        $ident
        $type_def
    }
}

pub (crate) use {for_each_node, define_pass_nodes};