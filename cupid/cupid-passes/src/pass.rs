

#[macro_export]
macro_rules! ast_pass_nodes {
    (
        Decl: $decl:item
		Function: $function:item
        Ident: $ident:item
    ) => {
        #[derive(Debug, Default, Clone)]
        pub enum Expr {
            Block(crate::Block<Expr>),
            Decl(Decl),
			Function(Function),
            Ident(Ident),

			Value(crate::Value),

            #[default]
            Empty,
        }

        $decl
		$function
        $ident
    }
}

/// Calls the provided AST pass trait for each member of the current passes' `Expr` enum
#[macro_export]
macro_rules! impl_expr_ast_pass {
    (impl $_trait:ident<$_return:ty> for $_type:ty { $_fn:ident }) => {
        impl $_trait<crate::Value> for crate::Value {
            fn $_fn(self, _: &mut Env) -> PassResult<crate::Value> { Ok(self) }
        }
        impl $_trait<$_return> for $_type {
            fn $_fn(self, env: &mut Env) -> PassResult<$_return> {
                match self {
                    Self::Block(block) => Ok(Expr::Block(block.$_fn(env)?)),
                    Self::Decl(decl) => Ok(Expr::Decl(decl.$_fn(env)?)),
                    Self::Function(function) => Ok(Expr::Function(function.$_fn(env)?)),
                    Self::Ident(ident) => Ok(Expr::Ident(ident.$_fn(env)?)),
                    Self::Value(value) => Ok(Expr::Value(value.$_fn(env)?)),
                    _ => todo!()
                }
            }
        }
    }
}

/// In most cases, the `Block` node can be automatically implemented,
/// because it's just a list of expressions. In some cases, it will need
/// to be handled separately though, so the default implementation
/// can be overridden
#[macro_export]
macro_rules! impl_block_ast_pass {
    // default implementation
    (impl $_trait:ident<$_return:ty> for $_type:ty { $_fn:ident }) => {
        impl $_trait<$_return> for $_type {
            fn $_fn(self, env: &mut Env) -> PassResult<$_return> {
                let crate::Block { expressions, attr } = self;
                Ok(crate::Block::build()
                    .expressions(expressions.$_fn(env)?)
                    .attr(attr)
                    .build())
            }
        }
    };
    // override implementation
    (impl $_trait:ident<$_return:ty> for $_type:ty { $_fn_body:item }) => {
        impl $_trait<$_return> for $_type {
            $_fn_body
        }
    }
}