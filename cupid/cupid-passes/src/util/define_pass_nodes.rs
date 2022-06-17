macro_rules! for_each_node {
    ($for:expr => $do:ident $args:tt) => {
        match $for {
            Expr::Block(x) => x.$do $args,
            Expr::Decl(x) => x.$do $args,
            Expr::Function(x) => x.$do $args,
            Expr::Ident(x) => x.$do $args,
            Expr::TypeDef(x) => x.$do $args,
            Expr::Value(x) => x.$do $args
        }
    }
}

macro_rules! as_expr {
    (fn(self) -> $expr:ident) => {
        fn as_expr(self) -> crate::PassResult<crate::$expr> {
            use crate::AsNode;
            crate::util::as_expr!($expr, self)
        }
    };
    ($expr:ident, $self:ident) => {
        match $self {
            Self::$expr(x) => Ok(x),
            _ => Err(($self.source(), cupid_util::ERR_EXPECTED_EXPRESSION))
        }
    }
}

/// Creates `From<T> for Expr` impl block for every provided node
macro_rules! impl_expr_from_node {
    ($node:ident;) => {
        impl From<$node> for Expr {
            fn from(node: $node) -> Self {
                Self::$node(node)
            }
        }
        impl From<$node> for crate::PassExpr {
            fn from(node: $node) -> Self {
                Self::from(Expr::from(node))
            }
        }
    };
    ($variant:ident($node:ty);) => {
        impl From<$node> for Expr {
            fn from(node: $node) -> Self {
                Self::$variant(node)
            }
        }
    };
    ( $( $variant:ident $(($node:ty))? ;)* ) => { 
        $( crate::util::impl_expr_from_node! { $variant $(($node))?; } )* 
    };
    () => {};
}

/// Creates an `Expr` enum that contains the nodes for the current pass
macro_rules! define_pass_nodes {
    (
        Decl: $decl:item
		Function: $function:item
        TypeDef: $type_def:item
    ) => {
        #[derive(Debug, Clone)]
        pub enum Expr {
            Block(crate::Block<Expr>),
            Decl(Decl),
			Function(Function),
            Ident(crate::Ident),
            TypeDef(TypeDef),
			Value(crate::Value),
        }

        crate::util::impl_expr_from_node! { 
            Block(crate::Block<Expr>);
            Decl;
            Function;
            Ident(crate::Ident);
            TypeDef;
            Value(crate::Value);
        }

        impl Default for Expr {
            fn default() -> Self {
                Self::Value(crate::Value::default())
            }
        }

        impl crate::AsExpr<crate::Value> for Expr {
            crate::util::as_expr! { fn(self) -> Value }
        }

        impl crate::AsExpr<crate::Block<Expr>> for Expr {
            fn as_expr(self) -> crate::PassResult<crate::Block<Expr>> {
                use crate::AsNode;
                crate::util::as_expr!(Block, self)
            }
        }

        impl crate::AsExpr<crate::Ident> for Expr {
            crate::util::as_expr! { fn(self) -> Ident }
        }

        impl crate::AsNode for Expr {
            fn scope(&self) -> crate::ScopeId {
                crate::util::for_each_node!(self => scope())
            }
            fn source(&self) -> crate::Source {
                crate::util::for_each_node!(self => source())
            }
            fn set_source(&mut self, source: crate::Source) { 
                crate::util::for_each_node!(self => set_source(source));
            }
            fn set_scope(&mut self, scope: crate::ScopeId) {
                crate::util::for_each_node!(self => set_scope(scope));
            }
        }

        $decl
		$function
        $type_def
    }
}

pub (crate) use {for_each_node, as_expr, define_pass_nodes, impl_expr_from_node};